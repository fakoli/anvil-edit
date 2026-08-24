# Project: Anvil Edit Core v0

## Summary

For local-first tool builders integrating a supported editor, build the control
plane that turns editor state into permission-bounded prediction requests and
validated, version-fenced, single-document application attempts while
preserving explicit executor identity, cancellation, privacy, causality, and
replayable evidence.

## Goals

- Define and implement the versioned narrow-waist lifecycle shared by editors,
  executors, and Lab.
- Deny unapproved source dispatch before serialization and make every retry,
  fallback, race, or escalation observable.
- Apply zero edits across a document-revision mismatch.
- Record enough source-minimized evidence to replay lifecycle and latency
  behavior without treating ordinary metadata capture as source consent.
- Support one full editor adapter and expose a capability contract for testing a
  second editor without claiming portability in advance.

## Non-Goals

- Train or fine-tune a predictive model.
- Implement semantic routing, multi-model racing, or hidden fallback.
- Apply a multi-document edit or execute terminal/test actions.
- Persist source-bearing traces, dispatch to a remote executor, or export data
  before the corresponding privacy and threat gates pass.
- Own model lifecycle, GPU placement, Anvil task state, fleet convergence, or a
  Workbench UI.
- Implement Anvil Events managed configuration; v0 preserves only a future
  source-free, asynchronous activation boundary and remains standalone.
- Build or fork a complete editor.

## Requirements

- R001: Core shall version every wire and durable lifecycle schema and reject unknown incompatible major versions.
- R002: Core shall represent editor state with a `DocumentRevision` containing adapter/workspace instance, document incarnation, logical URI and scheme, editor version namespace, position encoding, line-ending/canonicalization rules, range semantics, and full-buffer digest.
- R003: Durable events shall carry producer instance and sequence, producer monotonic clock identity/tick, wall and ingestion observations, causal parents, supersession, and idempotency keys sufficient to preserve retries and out-of-order delivery.
- R004: Prediction policy shall emit a `DispatchDecision` with purpose, visible/shadow mode, explicit capability/protocol revision, relative budgets, reason codes, experiment assignment, and attempt-group relation.
- R005: Core shall compile an `ExecutionGrant` before serializing source for another process or trust domain, independently governing runtime read, dispatch, persistence, replay, export, training, shadow, task context, and outcome correlation.
- R006: Effective authorization shall union denies, intersect allowlists, select minimum retention, honor local pause over all other policy, permit fleet policy only to narrow local permission, and fail closed on unknown input.
- R007: Every context item shall record inclusion reason, digest, token/byte cost, permission class, source revision, and application-critical, display-critical, or advisory freshness role.
- R008: One inference request shall name one explicit capability and native protocol; an unavailable route shall fail observably and any retry, race, fallback, or escalation shall create a separately linked attempt.
- R009: Cancellation shall propagate on a newer relevant revision, and cross-process deadlines shall use relative duration budgets rather than subtraction of unrelated wall clocks.
- R010: Protocol/model output shall be treated as hostile input with byte, nesting, edit-count, replacement-size, and time bounds plus protected-content, secret, and unsafe Unicode controls before plain normalized edits reach the editor.
- R011: Dispatch, serving observation, candidate, display decision, presentation attempt, application attempt, human outcome, and survival observation shall remain distinct causal records.
- R012: The v0 editor boundary shall provide a single-document conditional compare-and-apply transaction against the exact `DocumentRevision`; mismatch shall terminate stale with no offset repair.
- R013: Core shall join Edit-owned lifecycle evidence to executor-owned model/runtime/hardware and queue/prefill/decode evidence through one canonical request correlation ID, preserving missing or conflicting claims.
- R014: Metadata-only recording shall be the default; source-bearing persistence, replay, export, training, remote dispatch, task context, and outcome correlation shall require their own grants.
- R015: Authorized deletion shall physically erase covered content, linkable metadata, derived indexes, and governed copies while retaining at most a non-linkable receipt and reporting partial failure.
- R016: Editor adapters shall declare capabilities and shall not synthesize unsupported version, presentation, conditional-application, or outcome semantics.
- R017: Core shall consume one immutable, digest-bound `ConfigurationSnapshot` through a local provider interface and pin it into every dispatch; standalone local configuration is the initial provider, and a future Events adapter may atomically replace only a source-free, locally authorized snapshot through the separate contract in `docs/integrations/anvil-events.md` without becoming a hidden hot-path dependency.

## Acceptance Criteria

- Versioned fixtures round-trip every lifecycle object and reject an unknown
  incompatible major version.
- Document fixtures cover close/reopen, rename, untitled buffers, UTF-16 with
  non-BMP text, line-ending changes, terminal-newline changes, and overlapping
  multi-range edits.
- A denied or expired `ExecutionGrant` produces zero serialized source bytes to
  the executor transport.
- Duplicate and out-of-order event delivery reconstructs the same materialized
  lifecycle without duplicate attempts or outcomes.
- A new application-critical revision cancels pending work, suppresses late
  presentation, and yields zero stale applications.
- An unavailable capability records failure; a configured retry or fallback is
  a separately correlated request with its own decision and grant.
- Hostile or oversized native output is rejected without rendering raw model
  text, control sequences, or protected content.
- Content-disabled mode stores no source text, raw prompt/output, path, secret,
  or reconstructable replacement in the journal, logs, errors, or metrics.
- Deletion fixtures cover journal indexes, content blobs, materialized views,
  export copies, and partial-failure reporting.
- The executor/Edit joined manifest names ownership and blocks a dependent gate
  when required evidence is missing or conflicting.
- Standalone operation needs no Events process, and a dispatch retains the
  configuration snapshot it started with across a concurrent atomic activation.
- Unverified, incompatible, stale, or same-generation/conflicting configuration
  proposals never become active.

## Risks

- Editor-native versions may not survive reopen or map cleanly to portable
  document incarnations.
- Logging, errors, tokenizer caches, crash files, or storage-engine remnants can
  violate the source-minimization contract despite a clean logical schema.
- A broad policy abstraction could become difficult to inspect; the v0 finite
  grant model must remain smaller than a general policy language.
- IPC and persistence choices can add enough tail latency to distort the
  interactive baseline.
- A first adapter can accidentally hard-code one editor's UTF-16/range behavior
  into otherwise portable contracts.

## Open Questions

- Which concrete schema, local IPC, generated-binding, and peer-identity choices
  best realize D017's Rust/polyglot process split without weakening latency or
  authorization?
- Which first editor exposes the strongest complete lifecycle, not merely the
  easiest model-provider endpoint?
- Which local IPC peer-authentication mechanism is portable enough for v0?
- Which metadata store can meet concurrency and physical-erasure tests without
  obscuring the evidence model?

## Assumptions

### A001: Core v0 supports one deeply instrumented editor.

**Rationale:** One full lifecycle is sufficient to validate the contract; cross-editor portability remains a separately measured hypothesis.

**Requirements:** R002, R012, R016

### A002: Source-bearing persistence and remote dispatch start disabled.

**Rationale:** Metadata-only local execution is the reversible baseline while authorization, threat, key, and erasure controls are implemented and tested.

**Requirements:** R005, R006, R014, R015

### A003: Multi-document application remains outside the v0 transaction.

**Rationale:** Editors vary in atomic compare-and-apply support; pretending atomicity would weaken the core stale-edit invariant.

**Requirements:** R012, R016

## Features

### F001: Versioned lifecycle and event kernel

Defines portable document, causal envelope, attempt, and outcome schemas plus a
deterministic local materialization model.

**Requirements:** R001, R002, R003, R011

### F002: Context and authorization plane

Compiles freshness-aware context and a finite pre-serialization grant from
local repository, destination, purpose, and session controls.

**Requirements:** R005, R006, R007, R014, R015

### F003: Explicit executor and candidate boundary

Dispatches one explicit capability/protocol under relative budgets, joins
executor evidence, and bounds/normalizes hostile output.

**Requirements:** R004, R008, R009, R010, R013, R017

### F004: Adapter presentation and application boundary

Declares editor capabilities and implements distinct presentation, conditional
single-document application, human outcome, and survival records.

**Requirements:** R002, R011, R012, R016

## Tasks

### T001: Publish v1 draft schemas on the selected Core stack

**Feature:** F001
**Priority:** critical
**Likely files:** docs/DECISIONS.md, docs/DATA-MODEL.md, schemas/v1,
crates/anvil-edit-contracts, crates/anvil-edit-core,
tests/fixtures/contracts

D017 selects the Rust hot path and polyglot process shape. Complete the still-
open schema and IPC decision under O003, then implement versioned schemas for
`ConfigurationSnapshot`, `DocumentRevision`, lifecycle records, causal
envelopes, attempt relations, and survival observations with generated
fixtures. The snapshot schema preserves future managed provenance without
implementing an Events adapter in v0.

Current implementation note (2026-08-24): D018 and
`anvil-edit-contracts` now define the I/O-free semantic record set and exercise
one complete in-process lifecycle plus critical structural failures. This is a
prerequisite, not completion of T001: O003 is still open, no language-neutral
machine-readable schema or cross-language fixture exists, and the Core behavior
tasks remain unimplemented.

**Acceptance criteria:**

- D017 records the stack alternatives, latency/privacy tradeoffs, and rollback
  path; O003 records the selected schema and transport.
- Every R001-R004 and R011 object has a machine-readable v1 draft schema.
- Standalone `ConfigurationSnapshot` fixtures pin immutable component and
  effective-policy identity; optional desired-state provenance does not imply
  activation or use.
- Unknown incompatible major versions fail closed.

**Verification:**

- `cargo test -p anvil-edit-contracts`
- `cargo test -p anvil-edit-contracts --test v1_schema_fixtures`
- `cargo xtask check`

### T002: Prove document revision, ordering, and fencing semantics

**Feature:** F001
**Priority:** critical
**Likely files:** crates/anvil-edit-contracts/src/document.rs,
crates/anvil-edit-core/src/documents.rs, crates/anvil-edit-core/src/events.rs,
crates/anvil-edit-core/tests
**Dependencies:** T001

Implement portable revision comparison, range normalization, causal ordering,
idempotent materialization, and stale invalidation fixtures.

**Acceptance criteria:**

- Reopen/rename/untitled/UTF-16/non-BMP/EOL fixtures have unambiguous outcomes.
- Duplicate and out-of-order events materialize deterministically.
- A revision mismatch cannot enter the applied state.

**Verification:**

- `cargo test -p anvil-edit-contracts document_revision`
- `cargo test -p anvil-edit-core event_ordering`
- `cargo test -p anvil-edit-core fencing`

### T003: Implement finite policy resolution and ExecutionGrant

**Feature:** F002
**Priority:** critical
**Likely files:** crates/anvil-edit-contracts/src/execution_grant.rs,
crates/anvil-edit-core/src/policy.rs,
schemas/v1/execution-grant.schema.json, crates/anvil-edit-core/tests
**Dependencies:** T001

Implement independent grants and deterministic precedence with a one-shot or
expiring grant consumed before protocol serialization.

**Acceptance criteria:**

- Deny union, allowlist intersection, minimum retention, pause-wins,
  fleet-narrows, and unknown-fails-closed fixtures pass.
- Denied or expired grants expose no source-bearing transport payload.
- The effective policy digest and denial reason are recorded without source.

**Verification:**

- `cargo test -p anvil-edit-core policy`
- `cargo test -p anvil-edit-core execution_grant`
- `cargo test -p anvil-edit-core no_serialization_before_grant`

### T004: Build the metadata journal and authorized erasure path

**Feature:** F002
**Priority:** high
**Likely files:** crates/anvil-edit-core/src/store,
crates/anvil-edit-core/src/erase.rs, crates/anvil-edit-core/tests
**Dependencies:** T002, T003

Implement metadata-only persistence, bounded asynchronous writes, rebuildable
views, purpose-scoped identifiers, and deletion across all initial stores.

**Acceptance criteria:**

- Content-disabled fixtures find no source-bearing bytes in DB, logs, errors,
  metrics, or temporary artifacts.
- Erasure removes linkable rows/indexes/views and reports each failed target.
- Trace backpressure does not block editor operation.

**Verification:**

- `cargo test -p anvil-edit-core metadata_only`
- `cargo test -p anvil-edit-core erasure`
- `cargo test -p anvil-edit-core trace_backpressure`

### T005: Implement freshness-aware context compilation

**Feature:** F002
**Priority:** high
**Likely files:** crates/anvil-edit-core/src/context,
crates/anvil-edit-core/tests/context_policy.rs
**Dependencies:** T002, T003

Build bounded context packs with inclusion reasons, permission classes, costs,
source revisions, and dependency freshness roles.

**Acceptance criteria:**

- No retrieval result enters a request without a recorded reason and grant.
- Application-critical drift marks stale; display-critical drift suppresses;
  advisory drift is recorded without silent mutation.
- Protected paths are checked after URI/path resolution.

**Verification:**

- `cargo test -p anvil-edit-core context_policy`
- `cargo test -p anvil-edit-core protected_paths`

### T006: Implement the explicit executor and evidence seam

**Feature:** F003
**Priority:** high
**Likely files:** crates/anvil-edit-core/src/executors,
crates/anvil-edit-core/src/protocols, crates/anvil-edit-core/tests,
crates/anvil-editd/tests
**Dependencies:** T003, T005

Implement a standalone executor interface plus the optional Anvil Serving
adapter, relative budgets, cancellation, explicit attempts, and joined evidence.

**Acceptance criteria:**

- One request names one capability and protocol revision.
- Retries/fallbacks create new linked attempts; no adapter silently substitutes.
- Missing or conflicting executor evidence remains explicit and blocks gates.

**Verification:**

- `cargo test -p anvil-edit-core executor_contract`
- `cargo test -p anvil-edit-core evidence_join`
- `cargo test -p anvil-editd standalone_executor`

### T007: Bound, normalize, present, and conditionally apply candidates

**Feature:** F003
**Priority:** critical
**Likely files:** crates/anvil-edit-core/src/normalize,
crates/anvil-edit-core/src/candidates, crates/anvil-edit-core/tests
**Dependencies:** T002, T006

Parse native output within limits, normalize non-overlapping edits, validate
scope/freshness, and expose separate presentation and conditional application
transactions.

**Acceptance criteria:**

- Oversized, deeply nested, overlapping, protected, or unsafe-control output is
  rejected without raw rendering.
- Application succeeds only against the exact expected revision.
- Unsupported multi-document edits become explicit non-applicable candidates.

**Verification:**

- `cargo test -p anvil-edit-core protocol_output`
- `cargo test -p anvil-edit-core application`
- `cargo test -p anvil-edit-core candidate_normalization`

### T008: Publish the adapter SDK and full-lifecycle reference harness

**Feature:** F004
**Priority:** high
**Likely files:** schemas/adapter/v1, crates/anvil-edit-adapter-contract,
examples/reference-adapter, tests/fixtures/adapter-contract
**Dependencies:** T002, T003, T007

Define adapter capability discovery and a reference harness that exercises
snapshot, cancellation, presentation, conditional application, outcome, and
survival without claiming a second production editor.

**Acceptance criteria:**

- Missing capabilities are reported rather than synthesized or downgraded.
- The reference harness produces a causally joined lifecycle for one document.
- The same matrix can be run against a second-editor spike.

**Verification:**

- `cargo test -p anvil-edit-adapter-contract`
- `cargo run -p xtask -- adapter-contract examples/reference-adapter`
