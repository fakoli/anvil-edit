# Foundation decisions

Status: **working decision log**

Last reviewed: **2026-08-23**

Accepted entries constrain the initial design. Open entries must be resolved
before the phase that depends on them. Changes should append a superseding
entry rather than rewriting history without explanation.

## Accepted

### D001 — Product narrow waist

**Decision:** Anvil Edit owns the translation from live developer activity to
prediction opportunities, explicit inference workloads, presentation
decisions, and outcome evidence.

**Why:** This is the missing layer between editor interaction and model serving.
It remains stable while editors, context strategies, model protocols, and
inference engines change.

**Consequence:** Core and Lab share the lifecycle in `CONTRACTS.md`. Editor-only
plug-in behavior that cannot be replayed or measured is insufficient.

### D002 — Core and Lab first

**Decision:** The first product consists of Core, Lab, and one instrumented
editor adapter.

**Why:** The visible UX cannot be selected responsibly before capture/replay and
outcome evidence exist.

**Consequence:** Personal, Ripple, Steer, Fleet, and custom training are
deferred hypotheses, not parallel v0 products.

### D003 — Explicit capability selection above serving

**Decision:** Edit policy names one explicit capability per request. The
executor runs it or returns failure.

**Why:** Semantic difficulty and expected utility are Edit concerns; model
lifecycle and execution are Serving concerns.

**Consequence:** Fallback, race, or escalation creates another recorded Edit
request. Serving aliases never silently substitute models.

### D004 — High-rate traces stay in Edit

**Decision:** Editor opportunities, requests, decisions, and outcomes use an
Edit-owned local data plane.

**Why:** They are high-volume behavioral telemetry, not canonical project state
or desired fleet configuration.

**Consequence:** Anvil Events may converge policy/config revisions. Anvil State
may receive separately approved durable benchmark evidence. Neither stores the
hot-path stream.

### D005 — Offline replay is not human utility proof

**Decision:** Replay selects candidates for shadow and dogfood but cannot alone
promote a visible default.

**Why:** The observed future edit is not a complete counterfactual label, and
offline scoring cannot measure interruption or timing effects.

**Consequence:** Visible dogfood and durable survival are required evidence
classes for product claims.

### D006 — Exact version fencing

**Decision:** A candidate is valid only for the exact document identity,
version, and required content digest from which it was produced.

**Why:** A correct edit against old state is incorrect against current state.

**Consequence:** No best-effort offset repair is allowed in the foundation.
Cancellation saves work, but the final editor fence guarantees correctness.

### D007 — Source-bearing content is a separate permission class

**Decision:** Runtime context, metadata persistence, source-trace persistence,
export, and training are separate permissions.

**Why:** Enabling a prediction does not imply consent to build a dataset.

**Consequence:** Raw/source-bearing trace capture and all export are opt-in with
repository and destination policy. Metadata and content stores are logically
separate.

### D008 — One visible fast lane before routing

**Decision:** The first dogfood policy shows one fast candidate; alternatives
and semantic models begin in replay or shadow.

**Why:** An ensemble would confound model, context, routing, latency, and UX
before a baseline exists.

**Consequence:** A router survives only if a controlled comparison shows
incremental durable value at acceptable latency, interruption, and cost.

### D009 — Public docs remain topology-neutral

**Decision:** Public architecture and roadmap use generic capabilities and
hardware classes.

**Why:** Real host identities, active routes, and raw operational evidence are
private and change independently from portable product design.

**Consequence:** Candidate-to-host assignments and live alias mappings belong in
a private operator plan. A public example cannot be cited as deployed state.

### D010 — Portability remains a tested hypothesis

**Decision:** One adapter proves the first full lifecycle. A second-editor
capability matrix is required before claiming cross-editor portability.

**Why:** Provider/executor compatibility does not expose editor-native snapshot,
cancellation, presentation, application, or outcome semantics.

**Consequence:** A failed second adapter narrows the supported-editor claim; it
does not automatically kill Core or Lab. Missing capabilities remain explicit.

### D011 — Authorization precedes serialization

**Decision:** A finite `ExecutionGrant` is resolved before source-bearing data
is serialized for another process or trust domain.

**Why:** Executor configuration and repository allowlisting do not establish
destination-, purpose-, mode-, or content-specific authorization.

**Consequence:** Unknown policy fails closed. Runtime read, dispatch,
persistence, replay, export, training, shadow use, task context, and outcome
correlation remain independent grants.

### D012 — Portable document revisions and conditional application

**Decision:** Editor state is fenced through a portable `DocumentRevision`
including incarnation, logical URI, version namespace, encoding,
canonicalization, and full-buffer digest.

**Why:** Path plus version is ambiguous across reopen, URI schemes, encodings,
and editor implementations.

**Consequence:** v0 supports one-document compare-and-apply. Multi-document work
requires atomic conditional application or an explicit per-document review
fallback.

### D013 — Policy, presentation, application, and survival are distinct

**Decision:** Dispatch, grant, presentation, application, and survival are
separate append-only lifecycle records with causal and idempotency metadata.

**Why:** A policy decision is not a dispatch, a `show` is not a render, a user
gesture is not a successful apply, and acceptance is not durable survival.

**Consequence:** Funnel and latency metrics use observed attempts rather than
inferred state. Cross-process deadlines use relative budgets, not unrelated
wall clocks.

### D014 — Retention-bounded immutability permits physical erasure

**Decision:** Evidence is immutable only while authorized to remain. Deletion
physically erases source-bearing content, linkable metadata, derived indexes,
and governed copies.

**Why:** An append-only audit design cannot override a source-data deletion
contract.

**Consequence:** A minimal non-linkable deletion receipt may remain; global
cross-repository content deduplication is off by default. Backup and storage-
engine remnants are part of deletion verification.

### D015 — Lab is the technical differentiator, not a presumed sales lead

**Decision:** Lab is the strongest technical moat hypothesis; Phase 0 separately
tests whether the initial adoption path is Flow, Lab, or their combination.

**Why:** Evaluation infrastructure can be defensible without being the first
thing an individual developer buys or installs.

**Consequence:** A demand milestone precedes expansion, and an inconclusive
commercial result does not get disguised as technical success.

### D016 — Fleet configuration is an asynchronous, source-free bundle

**Decision:** A future Anvil Events integration converges one immutable
`edit/config/<channel>` bundle outside the prediction hot path. Core activates
the bundle only after local compatibility and policy checks and reports the
exact active generation, revision, and digest.

**Why:** One atomic bundle prevents independently delivered prompt, policy,
protocol, and capability revisions from producing an unreviewed combination.
Keeping reconciliation asynchronous preserves latency and lets standalone
Core work without a sibling service.

**Consequence:** Events artifacts and outcomes contain P0 configuration and
source-free operational evidence only. Fleet policy may narrow but never widen
local permission, and an Events desired revision is not an `ExecutionGrant`,
deployment, qualification, or promotion. The identifiers are reserved design
contracts until the upstream work and local conformance gates in
`integrations/anvil-events.md` are implemented.

## Open decisions

### O001 — First editor adapter

Choose between a deeply instrumented VS Code adapter and another editor surface
after checking document-version, cancellation, presentation, and outcome APIs.
Record the second-editor comparison fields at the same time. Provider
compatibility alone is not sufficient if it hides Lab evidence.

Needed by: Phase 0 exit.

### O002 — Implementation language and process shape

Choose the Core daemon, adapter SDK, and Lab implementation stack. The decision
must consider low-latency IPC, parser/LSP ecosystem, portable packaging,
SQLite/concurrency behavior, and protocol adapter reuse.

Needed by: Phase 0 exit.

### O003 — Concrete schema and transport

Choose JSON Schema/Protobuf/etc., local IPC transport, durable schema, and
migration strategy while preserving `CONTRACTS.md` semantics.

Needed by: Phase 0 exit.

### O004 — Raw trace retention default

Choose a finite source-bearing retention window, encryption/key lifecycle,
backup/sync behavior, deletion receipt, and storage-engine erasure procedure
after writing the platform threat model.

Needed by: Phase 1.

### O005 — Repository policy format

Choose location, inheritance, precedence, protected-path defaults, and UI
override behavior. Policy must compile the independent authorization grants and
deterministic precedence rules in `PRIVACY-AND-TRUST.md`.

Needed by: Phase 1.

### O006 — Initial candidate revisions and licenses

Resolve exact immutable model/tokenizer/runtime artifacts, licenses, native
protocol versions, and redistribution constraints. Research names candidates
but does not qualify them locally.

Needed by: Phase 2.

### O007 — First executor boundary

Decide whether phase 2 targets a standalone local endpoint first, Anvil Serving
first, or both behind one executor interface. No choice may weaken explicit
identity, deadline, cancellation, or no-fallback behavior.

Needed by: Phase 2.

### O008 — Calibrated utility and interruption gates

Set display, survival, undo, rewrite, late-result, stale-request, and useful
volume thresholds from the first visible dogfood window rather than declaring
universal values in advance.

Needed by: Phase 3 exit.

### O009 — Optional Anvil task context

Define which task packet fields may enter context, how permission is obtained,
and how the A/B isolates value from extra tokens. Task context is not in the
initial fast baseline.

Needed by: Phase 4.

### O010 — Project license and contribution policy

Choose repository license, contributor terms, model-artifact boundary, and
trace/dataset contribution policy before accepting external code or data.

Needed before: first public release or external contribution.

### O011 — Name clearance

Complete a public-name and trademark review for “Anvil Edit” and adjacent
editor/tool uses before the first public product launch. Internal prototyping
may continue under the working name.

Needed before: public launch or marketplace publication.

### O012 — Initial demand and adoption path

Identify the first user or buyer, current workaround, tolerated setup burden,
privacy threshold, trace-consent posture, and whether the primary pull is Flow
or Lab.

Needed by: Phase 0 exit.

### O013 — Threat model and destination identity

Choose process/host trust boundaries, IPC peer authentication, transport
identity, key lifecycle, crash/swap/temp handling, and remote destination
attestation sufficient to issue `ExecutionGrant` records.

Needed before: source-bearing persistence or remote inference.

### O014 — Anvil Events activation transport and compatibility

Choose the authenticated local activation boundary, exact bundle wire schema,
Core acknowledgement and recovery protocol, cross-version compatibility
matrix, and whether Anvil Events uses a managed-file adapter, external adapter
process, or another narrow reconciler. A language refactor is permitted only
with v2 envelope, durable-store migration/export, and reconciliation behavior
compatibility.

Needed before: implementing managed fleet configuration.

### O015 — Managed configuration staleness and emergency revocation

Choose bundle validity/expiry, offline grace, last-known-good behavior, local
emergency pause, and fail-closed behavior for a node that cannot observe a
revocation. Eventual convergence is not an emergency-revocation channel and
must not be presented as one.

Needed before: implementing managed fleet configuration.
