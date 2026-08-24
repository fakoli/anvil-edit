# Validation roadmap

Status: **proposed**

Planning window: **2026-08-24 through 2026-11-16**

Last reviewed: **2026-08-24**

This roadmap optimizes for learning rate. Dates and candidate models are
planning hypotheses, not deployment commitments. Each phase ends with evidence
and a decision; implementation does not continue merely because the preceding
code exists.

## Three-month objective

By the end of the validation window, Anvil Edit should be able to:

- capture and deterministically replay permitted edit opportunities;
- compare at least two specialized next-edit models and one control through
  pinned native protocols;
- serve one visible fast lane in one instrumented editor;
- run at least one alternative or semantic capability in shadow;
- report end-to-end latency, stale/cancelled work, display, acceptance, undo,
  survival, and compute per surviving edit; and
- make an evidence-backed decision to continue Core/Flow, emphasize Lab, pursue
  a narrower propagation wedge, or stop.

Private hardware assignment and active model routes belong in an operator plan,
not this public roadmap.

## Phase 0 — Foundation

Target: **2026-08-24 to 2026-09-02**

Implementation note (2026-08-24): D017 and the initial Rust workspace resolve
the language/process-shape slice. D018 adds the I/O-free semantic lifecycle
model, a complete structural `ConfigurationSnapshot`, source-free content
handles, and critical causal/document/grant/candidate/application fixtures.
D019 records the initial single-writer, bounded-selection, exact-fence, and
causal-replay algorithm defaults. Core now proves atomic configuration-identity
replacement, request-local identity pinning, and one actor-owned local
revision-generation primitive; it still has no session actor or cancellation
transport. The repository has not selected O003's wire, IPC, or durable schemas
and does not yet satisfy the Phase 0 privacy, editor, adopter,
language-neutral-fixture, or executable-behavior exit gates.

Deliverables:

- review and accept the product boundary;
- validate an initial user/buyer, current workaround, setup tolerance, privacy
  threshold, trace-consent posture, and Flow-versus-Lab demand;
- turn `CONTRACTS.md` into versioned draft schemas and fixtures, including
  `DocumentRevision`, `DispatchDecision`, `ExecutionGrant`, presentation,
  application, survival, causality, and dependency freshness;
- preserve the D017 Rust/polyglot process boundary while selecting the concrete
  schema and IPC contracts under O003;
- define the finite repository/destination authorization policy and threat
  model;
- choose the first editor adapter and complete a second-editor capability
  matrix;
- define a replay manifest and deterministic fixture format; and
- review the Core, Lab, and Flow implementation PRDs.

Exit gate:

- every lifecycle object has a reviewed schema;
- stale-buffer, cancellation, pre-serialization authorization, source-
  persistence/erasure, event-order/idempotency, and explicit-capability
  invariants have executable fixture cases;
- one first adopter and a bounded adoption hypothesis are recorded;
- portability claims match the adapter capability evidence;
- unresolved privacy or first-editor decisions are visible; and
- no model or host route is presented as deployed.

Kill or revise Flow if the editor cannot provide stable document revisions,
outcome observations, or a conditional presentation/apply boundary. Narrow to
one editor when a second adapter is insufficient; do not discard Lab solely for
that reason.

## Phase 1 — Recorder and replay kernel

Target: **2026-09-03 to 2026-09-20**

Deliverables:

- local daemon skeleton and editor connection;
- bounded single-writer session coordination with request-local configuration
  and exact revision pins;
- snapshot, opportunity, decision, and outcome capture;
- metadata/content-store separation;
- repository allowlist, protected paths, pause, retention, and deletion;
- `ExecutionGrant` enforcement before serialization or remote dispatch;
- version-fenced normalized edit application fixtures;
- deterministic replay of captured opportunities without model inference;
- explicit replay diagnostics for idempotency, missing causal parents,
  producer-sequence gaps, and cycles; and
- timing instrumentation from editor opportunity through render.

Exit gate:

- a captured permitted opportunity replays from byte-identifiable inputs;
- content-disabled capture stores no source-bearing text in DB, logs, or
  metrics;
- deletion and retention behavior is testable;
- rapid typing and file switches produce cancellations and zero stale applies;
- trace-write degradation cannot block normal editing.

Kill or revise if privacy controls make real trace collection too risky or the
adapter cannot attribute user outcomes with useful confidence.

Source-bearing persistence, remote inference, and multi-document application
remain blocked until the authorization/deletion, threat-boundary, and document-
transaction contracts respectively pass their fixtures.

## Phase 2 — Fast-model baseline and Lab

Target: **2026-09-14 to 2026-10-04**

Deliverables:

- native protocol adapters for two specialized next-edit candidates;
- at least one FIM or general-code control;
- direct local executor and optional Anvil Serving adapter;
- fixed-context/output/cache benchmark matrix;
- replay quality, location, latency, and cost metrics;
- immutable manifests and raw-result digests; and
- one selected visible fast-path candidate.

Candidate set from research, subject to revision/license/runtime verification:

- Zeta 2.1;
- Sweep Next-Edit 1.5B;
- Continue Instinct or another open next-edit control; and
- one FIM control such as Mellum or Seed-Coder.

Exit gate:

- at least three candidates run through pinned, named protocols on identical
  permitted corpus partitions;
- TTRS components and p50/p90/p95/p99 are reported with sample counts;
- each comparison names the full model/tokenizer/protocol/context/runtime/
  quantization/validation system bundle;
- a fast candidate meets the initial latency envelope and parser/fencing gates;
- the choice is based on local evidence, not reputation.

Kill or narrow to Lab if no candidate meets the interactive envelope with
enough valid edit signal to justify visible dogfood.

## Phase 3 — Restrained visible dogfood

Target: **2026-09-28 to 2026-10-25**

Deliverables:

- one visible fast capability;
- inline/local-diff presentation and explicit reveal/subtle mode;
- display-policy reason codes;
- partial acceptance, dismissal, undo, rewrite, and survival observations;
- a minimum viable user control and trace-inspection surface; and
- an explicit-reveal exploratory pilot;
- predeclared metrics, thresholds, exclusions, assignment, clustering,
  censoring, and stopping rules frozen after that pilot; and
- a later randomized visible comparison with immutable reports.

Initial operational hypotheses:

- TTRS p50 at or below 200 ms;
- TTRS p90 at or below 500 ms;
- zero stale application;
- no hidden executor/model substitution;
- source-bearing export disabled by default.

Exit gate:

- sufficient real opportunities exist to stabilize core funnel and latency
  estimates;
- accepted content shows non-trivial five-minute survival;
- undo, rapid rewrite, visible duration, and feature-disable evidence do not
  indicate an annoying product;
- the confirmatory window is temporally separate from threshold calibration;
- results may be declared inconclusive rather than forced into pass/fail.

Kill or pivot to explicit-reveal-only if the offline winner is too interruptive
when shown.

## Phase 4 — Semantic shadow and policy experiment

Target: **2026-10-12 to 2026-11-08**

Deliverables:

- one general coding model benchmarked at 2K, 4K, and 8K realistic contexts
  with short outputs;
- shadow requests only, with explicit deadlines and cost accounting;
- fast/semantic disagreement corpus paired with future developer action;
- declared hard-opportunity heuristics;
- fast-only versus gated-fast/semantic replay and shadow comparison; and
- optional bounded task-intent A/B after the base policy is stable.

The semantic candidate is not placed in the visible path until local
short-context TTFT/TTRS is known and shadow evidence nominates a gated policy
for a separately predeclared visible experiment.
A very large teacher/critic model remains offline or shadow analysis initially.

Exit gate:

- the gated policy earns nomination for visible comparison through declared
  replay/shadow signals; shadow alone is not durable utility proof;
- escalation frequency and failure behavior are bounded;
- no task or repository context crosses an unapproved destination.

Kill the router if fast-only is operationally equivalent. Architectural novelty
is not a reason to retain it.

## Phase 5 — Executor compatibility and product decision

Target: **2026-10-26 to 2026-11-16**

Deliverables:

- one executor/provider-compatibility spike in an editor other than the first
  adapter, explicitly distinguished from full Core portability;
- a sanitized reproducible benchmark package;
- comparison of Core/Flow, Lab, and a narrower propagation product wedge;
- recorded go/no-go decision and next PRD; and
- explicit deferred list for personalization, training, steering, and fleet.

Portability means more than an endpoint connection. The spike records which
snapshot, cancellation, presentation, and outcome semantics the editor surface
cannot expose.

Go when evidence supports a useful, restrained local experience and a
repeatable Lab advantage. Narrow when Lab is valuable but predictive UX is not.
Stop when utility cannot justify interruption, privacy burden, or compute cost.

## Workstream dependencies

```text
contracts and privacy
        -> recorder and replay
        -> fast candidate bakeoff
        -> visible dogfood
        -> semantic shadow
        -> gated policy
        -> product decision
```

Lab implementation may proceed alongside Core after the shared trace contract
is accepted. UI polish, multiple editors, personalization, and fleet work do
not precede visible dogfood evidence.

Engineering workstreams may overlap, but exposure gates do not: remote/source-
bearing capture waits for privacy and authorization evidence; visible dogfood
waits for contract and latency evidence; promotion waits for the temporally
separate visible and durable-outcome gates.

## Explicitly deferred

- Custom model training.
- Personal LoRA or team tuning.
- Automatic multi-file application.
- Agent steering from raw keystrokes.
- Central raw-trace collection.
- Individual developer productivity dashboards.
- General smart routing inside Anvil Serving.
- A full editor fork.
- Anvil Events managed configuration. The non-implemented contract is recorded
  in [`integrations/anvil-events.md`](integrations/anvil-events.md); delivery
  waits for the upstream convergence issue, Core's local activation contract,
  and standalone product evidence.

Each deferred item needs a new decision record and its own acceptance/privacy
gate before entering a delivery plan.
