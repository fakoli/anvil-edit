# Algorithms and data structures

Status: **accepted foundation defaults; most runtime behavior not yet implemented**

Last reviewed: **2026-08-24**

This document explains which algorithms and in-memory structures best fit
Anvil Edit's current architecture and why. It is implementation guidance under
[`ARCHITECTURE.md`](ARCHITECTURE.md), [`CONTRACTS.md`](CONTRACTS.md), and the
accepted decisions in [`DECISIONS.md`](DECISIONS.md). If they disagree, those
canonical sources win.

These choices do not settle O003's wire protocol, IPC transport, generated
bindings, or durable database schema. They also are not latency evidence. Each
choice remains subject to profiling and the rollback conditions below.

## Design forces

The workload has three different computational shapes:

1. a deadline-sensitive hot path that should reject ineligible or stale work
   in constant time;
2. bounded selection and validation steps over intentionally small candidate
   sets; and
3. an append-only evidence path that must preserve causality, retries,
   missingness, retention, and deterministic replay without delaying editing.

The resulting foundation spine is:

```text
editor event
  -> single-writer session coordinator
  -> deterministic opportunity gate
  -> bounded context selector
  -> explicit fast capability
  -> normalize and validate
  -> expected-utility display gate
  -> conditional application against the exact revision

all stages
  -> bounded asynchronous evidence writer
  -> causal journal
  -> deterministic Lab replay and materialized views
```

## Decision summary

| Concern | Initial algorithm | Primary structure | Expected cost |
| --- | --- | --- | --- |
| Session coordination | Single-writer actor/event loop | Bounded MPSC inbox and actor-owned maps | O(1) state transition |
| Stale work | Latest-relevant-revision wins, with cooperative cancellation and final exact fence | Local generation, cancellation handle, `DocumentRevisionRef` | O(1) fast rejection |
| Configuration | Read-copy-update | `ArcSwap<ConfigurationSnapshot>` and request-local `Arc` | O(1) request pin |
| Opportunity gate | Deterministic finite-state gate with debounce and hysteresis | Monotonic timer, EWMA, token bucket, bounded LRU | O(1) per editor event |
| Recent edit context | Fixed-capacity history | `VecDeque` | O(1) amortized append/evict |
| Semantic context | Bounded greedy marginal-utility selection | Adjacency-list symbol graph plus sorted `Vec` or top-k heap | O(n log k) static; O(nk) bounded marginal scoring |
| Candidate edits | Normalize, sort a view, scan adjacent ranges | Small `Vec<NormalizedTextEdit>` | O(e log e) |
| Visible policy | Fast-first deadline-aware cascade with abstention | Explicit state and calibrated utility score | Bounded by one visible attempt in v0 |
| Authorization | Deterministic finite policy fold | Fixed `PermissionSet`, temporary sets, canonical sorted manifests | O(p + c log c) for bounded policy/content inputs |
| Evidence capture | Append-only non-blocking handoff | Bounded queue and immutable lifecycle records | Amortized O(1) append |
| Replay | Idempotency filtering plus stable causal topological traversal | Record/idempotency indexes and adjacency lists | O(V + E), or O((V + E) log V) with stable ready queue |
| Live latency summaries | Streaming bounded-memory measurement | HDR histogram and counters | O(1) record |
| Durable utility | Censor-aware survival analysis | Checkpoint table and Kaplan-Meier risk sets | Offline/reporting path |

`n`, `k`, and `e` are bounded context candidates, selected items, and candidate
edits. A limit is part of configuration and evidence, not merely a defensive
implementation detail.

## Session coordination

### Choose a single-writer actor

One logical actor owns each editor session's mutable coordination state. Many
session actors may run concurrently, and bounded worker tasks may perform
context preparation or inference, but their results return through the owning
actor before changing session state.

A future session state is expected to contain structures resembling:

```text
SessionState
  documents          HashMap<DocumentKey, DocumentSlot>
  recent_edits       VecDeque<EditReference>
  in_flight          HashMap<RequestId, InFlightRequest>
  duplicate_cache    bounded LRU<OpportunityFingerprint, MonotonicTick>

DocumentSlot
  current_revision   DocumentRevisionRef
  local_generation   u64
  cancellation       cancellation handle
```

The actor uses a bounded MPSC inbox. When it is full, adapters coalesce or
discard superseded low-value observations according to explicit policy rather
than growing memory without limit.

This was chosen because events within one editor session already have a natural
serialization point. It makes state transitions, fencing, backpressure, and
testing easier to reason about than shared mutable maps guarded by locks. It
also prevents completion order from silently becoming lifecycle order.

Do not introduce `DashMap`, a lock-free graph, or shared `RwLock` session state
until profiling shows the actor is a bottleneck. CPU work can be parallel
without making state ownership parallel.

## Revision fencing, cancellation, and deadlines

Every asynchronous operation carries:

- the exact `DocumentRevisionRef` and required context dependency revisions;
- an ephemeral local generation used for cheap supersession checks;
- a cancellation identity or handle; and
- a remaining relative deadline budget.

When a newer relevant revision arrives, the actor increments the affected
local generation and signals cancellation. Context, executor, normalization,
display, and application boundaries may reject a returned result in O(1) by
generation before performing the complete revision comparison.

The generation is only a process-local optimization. It is not durable
identity, is not comparable across restarts or actors, and cannot replace the
document incarnation, editor version namespace/value, text semantics, byte
length, and digest in `DocumentRevisionRef`. Cooperative cancellation saves
resources; exact revision checks at decision, presentation, and application
are the correctness controls.

Deadlines use one monotonic clock inside a process. Cross-process requests
transmit a remaining duration budget. A receiver never subtracts wall clocks
from different processes. Work that misses the interactive deadline is not
shown, even if cancellation was unavailable; it may become separately
permitted shadow evidence.

## Immutable configuration activation

Core validates and compiles a complete configuration outside the prediction
path, then atomically swaps one immutable pointer. Each request pins an `Arc`
to the complete snapshot it started with.

`ArcSwap<ConfigurationSnapshot>` is the intended foundation shape because
reads dominate writes, activation is infrequent, and a request must retain the
old snapshot across concurrent replacement. It avoids a read lock on every
opportunity while preserving exact configuration identity. The current code
proves this behavior for `ConfigurationIdentity`; expanding the same seam to
the complete snapshot remains future work.

## Opportunity detection and duplicate suppression

The initial opportunity gate is a deterministic finite-state machine, not a
model call:

```text
idle -> debouncing -> eligible -> in_flight
            |            |          |
            +-> suppress <-+---------+
```

Cheap inputs may include trigger kind, explicit reveal, syntax state, recent
acceptance, typing velocity, duplicate fingerprint, and active concurrency.
The implementation uses:

- a monotonic debounce timer with hysteresis so small timing jitter does not
  repeatedly cross the eligibility boundary;
- an exponentially weighted moving average for typing velocity rather than an
  unbounded event history;
- a token bucket for request-rate and concurrency pressure; and
- a bounded LRU keyed by a source-free opportunity fingerprint for duplicate
  suppression.

These structures keep per-event work and memory bounded, make suppression
reasons replayable, and provide an interpretable baseline. They are preferred
over an online learned gate until visible dogfood supplies valid outcome and
interruption labels.

## Bounded context selection

Context compilation follows permission filtering, never precedes it. The
initial selection algorithm is:

1. reject content that is unauthorized, protected, stale, or outside the
   configured source and byte bounds;
2. reserve budget for the required active prefix/suffix and any other
   policy-mandatory item;
3. create a bounded candidate pool from recent edits, visible/recent regions,
   diagnostics, semantic references, and optional task fields;
4. score candidates for intent relevance, freshness, locality, semantic
   connection, expected value, token cost, and privacy/latency cost;
5. repeatedly choose the highest positive marginal value per cost while
   penalizing redundant coverage; and
6. stop at the token, byte, item, time, or permission boundary.

This is a greedy budgeted maximum-coverage strategy with an MMR-style
redundancy penalty. Exact knapsack optimization is not justified under an
interactive deadline, and whole-repository inclusion defeats the latency and
privacy contract.

The candidate pool stays in a `Vec`. A static score can be sorted directly or
trimmed to top-k with a min-heap in O(n log k). Iterative marginal scoring is
O(nk), which remains acceptable only because both values are configured and
small. Recent patches use a fixed `VecDeque`. Precomputed language-service
relationships use an adjacency list such as
`HashMap<SymbolId, Vec<SymbolEdge>>`; Core does not construct an unbounded
repository graph synchronously.

An LRU cache may reuse content/tokenization results keyed by exact revision,
context-policy revision, protocol/tokenizer identity, and content digest. A
cache hit never relaxes a grant or freshness check. Executor-owned prefix cache
state remains executor evidence rather than inferred Core state.

## Candidate representation and conditional application

Keep normalized edits in a small ordered `Vec`. To validate a candidate, make
a sorted index or sorted copy by normalized `(start, end, original_index)` and
scan adjacent ranges for overlap. This is O(e log e), simple to audit, and
already matches the semantic contract implementation.

An interval tree is inappropriate while edit count is tightly bounded and
validation is a one-shot operation. It becomes worth reconsidering only if
measured workloads contain large, repeatedly queried edit sets.

The semantic result retains `base_relative_as_listed` order. The editor adapter
uses an atomic conditional batch when available. A local text implementation
would normally apply non-overlapping base-relative edits from higher offsets to
lower offsets, reversing equal-position insertions during physical application
so the semantic list order is preserved. That implementation detail must be
proved against editor-native batch semantics rather than assumed portable.

The editor remains the source of truth. Core does not need a CRDT or operational
transformation system because v0 rejects stale suggestions instead of rebasing
them. If measurements later justify a mutable Core-side document mirror, use a
rope or piece-tree plus a line-start index; do not introduce that complexity
while bounded immutable windows are sufficient.

## Prediction routing and abstention

The initial visible policy is a cascade, not an ensemble:

```text
cheap gate -> one fast capability -> validator -> display gate -> show/abstain
```

Alternative and semantic candidates begin in replay or shadow. A semantic
attempt becomes eligible only through a separate, observable Edit decision and
only after a controlled experiment shows incremental durable value.

The long-term display score should approximate:

```text
expected utility =
    P(useful survival) * edit value
    - interruption cost
    - latency cost
    - compute cost
```

The first score is deterministic and inspectable. After sufficient real data,
a calibrated logistic model or small boosted-tree classifier may estimate the
terms offline. A contextual bandit may later explore show/abstain or routing
choices inside predeclared safety and privacy bounds. Deep or online RL is not
an initial dependency: Anvil Edit does not yet possess enough unbiased visible
outcomes to justify it.

## Authorization compilation

Authorization remains a finite product-specific fold, not a general policy
DSL:

- union denies;
- intersect allowlists;
- choose the shortest permitted retention;
- let local pause win;
- allow fleet input to narrow but never widen local permission; and
- fail closed on missing or unknown inputs.

`PermissionSet` remains explicit because independent grant dimensions are
reviewable in code and fixtures. A compiler may use temporary `HashSet` or
bitset-like structures for bounded membership operations, but it canonicalizes
content classes and handles into sorted, duplicate-free vectors before hashing
or recording a grant. Runtime iteration order must not become evidence order.

## Evidence journal and deterministic replay

The hot path hands lifecycle records to a bounded asynchronous trace queue.
The writer appends immutable-during-retention records and updates rebuildable
indexes. Optional evidence is degraded or dropped before the queue delays the
editor, and the loss count itself is evidence.

Useful logical indexes include:

```text
record_id                              -> lifecycle record
(producer_instance, idempotency_key)  -> canonical delivery
(producer_instance, producer_sequence)-> record_id
causal_parent_id                       -> child record IDs
request_correlation_id                 -> Edit/executor observations
(document incarnation, version, digest)-> lifecycle references
experiment/config/model/protocol IDs   -> report strata
```

Replay first applies producer-scoped idempotency, then creates edges from
causal parents and observed producer order. It performs Kahn topological
traversal with a stable ready queue, using producer identity, producer sequence,
and record ID only as deterministic tie-breakers between otherwise independent
nodes.

The tie-breaker makes repeated materialization byte-reproducible; it does not
claim a causal relationship. Wall time never establishes order. Missing causal
parents, producer-sequence gaps, cycles, duplicate identities, and conflicting
evidence remain explicit failures or unresolved observations rather than being
filled in. Replay is O(V + E), or O((V + E) log V) when the stable ready queue
is a balanced tree or heap.

Do not use one globally ordered distributed log as the semantic model. The
contract has producer-local sequence plus causal edges because clocks and
delivery order are not globally comparable.

## Measurement structures

Core may maintain HDR histograms for bounded-memory live latency summaries and
simple counters for funnel outcomes. HDR histograms were chosen because the
product cares about wide-range p50 through p99 latency without allocating one
value per hot-path observation. Welford's online algorithm is sufficient for
streaming mean and variance when those diagnostics are useful.

These summaries do not replace permitted raw durations or a lossless benchmark
artifact. Lab computes authoritative report quantiles from retained raw values
when allowed and reports denominators, terminal failures, and coverage.

Survival checkpoints are right-censored. Lab therefore uses Kaplan-Meier risk
sets for descriptive checkpoint survival rather than dropping missing future
observations or treating them as failures. Cluster-aware uncertainty and the
predeclared experiment analysis still determine what conclusions are valid.

Experiment assignment occurs on the common opportunity stream before policy
gating. A predeclared seeded assignment function over the declared unit
(developer, repository, session, or N-of-1 period) provides reproducibility;
per-opportunity randomness must not accidentally mix treatments inside a unit
whose outcomes interfere.

## Durable-store candidate, not decision

SQLite in WAL mode plus a repository-and-purpose-scoped content-addressed blob
store is the strongest initial candidate:

- SQLite fits local transactional metadata, indexes, export, and deterministic
  materialized views;
- WAL can separate the writer from readers; and
- content-addressed blobs make integrity and governed replay explicit without
  putting source bytes in lifecycle rows.

This remains a candidate under O003 and O004. Selection requires concurrency,
tail-latency, migration, export, encryption, backup, and physical-erasure
evidence. Global content deduplication is forbidden by default because it
creates a cross-repository correlation surface. Deletion tests must cover WAL,
free pages, checkpoints, temporary files, derived indexes, backups, and export
copies.

## Structures deliberately not selected for v0

| Structure or algorithm | Why it is not the default |
| --- | --- |
| CRDT or operational transformation | Core suggests against an exact editor revision; it does not own collaborative document truth or rebase stale edits |
| Shared multi-writer session graph | Adds synchronization and ordering ambiguity where one editor session already supplies a natural owner |
| Lock-free containers everywhere | Higher proof and maintenance burden without measured contention |
| Interval tree for candidate edits | Edit lists are bounded, small, and validated once |
| Exact knapsack context packing | Extra compute is not justified under the render deadline; greedy selection is inspectable and bounded |
| Synchronous vector database or repository scan | Makes latency, permission, and cancellation difficult to bound |
| Global wall-clock or total-order event stream | Cross-process clocks are not causal and unrelated events need no fabricated order |
| Model race on every opportunity | Confounds the fast baseline and consumes latency/compute before incremental value is proven |
| Online RL from the first release | Visible outcomes are initially sparse, clustered, censored, and policy-influenced |
| Core-side rope or piece tree immediately | The editor remains authoritative and bounded immutable windows may be sufficient |

## Current Rust foothold

The foundation code currently contains:

- atomic request-local configuration-identity pinning through `ArcSwap`;
- semantic candidate validation using a sorted range view and adjacent overlap
  scan; and
- `LatestRevision`, a single-writer local generation/fence primitive that
  validates exact revisions, shares request pins through
  `Arc<DocumentRevisionRef>`, avoids generation bumps for duplicate snapshots,
  detects conflicting reuse of the currently active revision ID, and makes
  supersession observable.

`LatestRevision` deliberately contains no lock, queue, executor, editor, or
storage dependency. A future session actor owns it and uses an advanced
generation to signal cancellation. Its `is_current` check is a fast guard; the
adapter still performs the complete exact-revision check before presentation
and application.

The future session coordinator owns document-slot assignment and the durable
materializer owns historical identity-conflict detection. `LatestRevision`
retains only the current value, so it does not prove that an identifier was
never used by an older, no-longer-current record.

The opportunity FSM, bounded context selector, trace queue, causal replay
planner, metric structures, IPC, and durable stores remain unimplemented. Their
presence in this document is an implementation direction, not evidence that
the product runs.

## Profiling and rollback rules

Replace one of these defaults only when a pinned benchmark or adversarial
fixture shows a concrete failure, and preserve the external contracts while
doing so. Examples include:

- actor mailbox or worker handoff dominates measured p90/p99;
- bounded greedy context selection materially loses surviving edit value to a
  tested alternative within the same latency/privacy budget;
- candidate edit counts make repeated range queries dominate normalization;
- a document mirror demonstrably lowers end-to-end cost enough to justify its
  synchronization and memory burden;
- histogram error is insufficient for a declared metric; or
- the selected store cannot meet concurrency, migration, or physical-erasure
  gates.

Every comparison pins configuration, policy, protocol, adapter, runtime,
hardware, corpus, limits, and raw evidence as required by
[`EVALUATION.md`](EVALUATION.md). Architectural novelty alone is not a reason
to replace a simpler bounded structure.
