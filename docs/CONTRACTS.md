# Prediction contracts

Status: **normative foundation; wire format not yet implemented**

Last reviewed: **2026-08-23**

This document defines the semantic contract. Concrete JSON Schema, API, and
database representations must preserve these meanings and invariants, but are
not chosen here.

## Lifecycle

```text
EditorSnapshot
      -> PredictionOpportunity
      -> DispatchDecision
      -> ExecutionGrant
      -> PredictionRequest
      -> PredictionCandidate
      -> PredictionDecision
      -> PresentationAttempt
      -> ApplicationAttempt
      -> SurvivalObservation
```

The lifecycle is not a guarantee that every object produces the next object.
An opportunity may be ineligible. A request may be cancelled or expire. A
model may return no valid candidate. A valid candidate may be suppressed. A
presentation may fail. A shown suggestion may receive no attributable human
action, and a user gesture may fail its final conditional application fence.

## Common envelope

Every durable object has:

| Field | Meaning |
| --- | --- |
| `schema_version` | Version of the object's serialized contract |
| `id` | Immutable identifier unique within its declared purpose and trust domain |
| `occurred_at_wall` | Producer wall-clock observation for human correlation, never sole event ordering |
| `producer_monotonic_tick` | Producer-local monotonic tick used for durations and ordering on one clock |
| `monotonic_clock_id` | Identity of the monotonic clock/process epoch |
| `producer_instance_id` | Process/adapter instance that emitted the object |
| `producer_sequence` | Strictly increasing sequence within one producer instance |
| `ingested_at_wall` | Receiver wall-clock time, when the object crosses a process boundary |
| `caused_by` | Zero or more causal parent identifiers |
| `supersedes` | Prior record replaced by this record, when applicable |
| `idempotency_key` | Producer-scoped duplicate-suppression key for retried delivery |
| `session_id` | Purpose-scoped local editing-session correlation identifier |
| `repository_id` | Purpose-scoped pseudonymous repository identifier |
| `provenance` | Producer, producer revision, and capture mode |

Wall-clock time supports cross-process evidence but does not establish causal
order. Monotonic ticks are comparable only inside one `monotonic_clock_id`.
Cross-process deadlines are transmitted as remaining duration budgets and
joined through causal identifiers; receivers do not subtract unrelated clocks.
Purpose-scoped identifiers must not become a covert cross-repository or
cross-product identity graph.

## `ConfigurationSnapshot`

The immutable, locally validated configuration identity used by Core for one
or more prediction lifecycles. Standalone local configuration is the initial
provider. A future Anvil Events reconciler may propose a replacement, but the
hot path consumes only an already-active snapshot.

Required semantics:

- configuration provider and provider revision;
- snapshot identifier, immutable revision, and canonical digest;
- activation attempt identifier and optional previous snapshot identifier;
- standalone or managed mode;
- optional Events desired event, authority, resource, generation, revision,
  adapter, and artifact digest, kept distinct from the active snapshot identity;
- immutable prediction, context, display, routing, authorization, prompt-
  protocol, capability-pack, and normalization component identities;
- effective local policy digest and whether an external proposal was narrowed;
- activation and verification observations on one declared clock; and
- compatibility result, validity limit when any, and lifecycle observation.

Lifecycle observations distinguish `staged`, `active`, `superseded`,
`rejected`, `rolled_back`, and `indeterminate`; they do not mutate the
snapshot's immutable identity. Desired, received, staged, active, used by a
request, deployed at an executor, and promoted are separate evidence states. A
request or dispatch pins the active snapshot it used; later activation does
not rewrite prior evidence.

## `ConfigurationReconciliationObservation`

A source-free local record joining an external desired revision to Core's
configuration activation boundary without treating either side as the other's
authority.

Required semantics:

- desired event, correlation, reconciliation operation, and local activation
  attempt identifiers;
- authority/resource/adapter binding and target result;
- generation, revision, artifact digest, bundle-contract version, and adapter
  revision;
- prior and proposed `ConfigurationSnapshot` identifiers;
- receive, stage, activate, verify, rollback, and terminal observations when
  they occurred;
- schema, compatibility, artifact, local-policy intersection, activation, and
  verification results; and
- terminal outcome, exact effective `ConfigurationSnapshot` identifier and
  digest when activated, whether the external proposal was narrowed, and
  bounded source-free reason codes.

Exactly one locally configured authority owns a managed resource on a node at a
time. Changing that owner requires a separate local rebind operation. Generation
is monotonic per authority/resource binding, and reusing one generation with a
different revision, digest, artifact reference, adapter, or bundle schema is an
integrity conflict. An authority-requested rollback is a new higher generation
referring to previously accepted immutable bytes; generation never moves
backward.

## `DocumentRevision`

A portable identity for the exact editor buffer state used by a snapshot,
candidate, presentation, or application transaction.

Required semantics:

- adapter type, adapter instance, and workspace instance;
- logical URI and URI scheme without assuming a local filesystem path;
- document incarnation identifier that changes across close/reopen or logical
  replacement when the editor cannot prove continuity;
- editor-native version value and version namespace;
- position encoding, including UTF-8/UTF-16/code-point semantics;
- line-ending and terminal-newline representation;
- range end semantics and text canonicalization rules;
- full-buffer byte length and digest over the declared canonicalization; and
- source persistence class.

A document name, path, or editor version number alone is not a revision. Text
edits declare whether ranges are half-open and how multiple edits are ordered.
The foundational contract permits one-document conditional application. Any
future multi-document transaction must either provide atomic compare-and-apply
across all target revisions or expose an explicit per-document review fallback;
it must never imply atomicity the editor cannot provide.

## `EditorSnapshot`

An immutable description of the editor state from which a prediction may be
constructed.

Required semantics:

- active `DocumentRevision`;
- cursor and selection ranges;
- language identifier;
- visible ranges;
- ordered recent edit references;
- open or recently visited logical document references, when permitted;
- diagnostics or semantic references, when available and permitted;
- content digest for every source-bearing input; and
- capture and persistence class.

The hot path may hold content in memory. Durable metadata must refer to
source-bearing content through a governed content reference rather than
silently duplicating source text into logs or event rows.

An editor snapshot is not a complete repository snapshot. The context compiler
records which bounded portions it selected later.

## `PredictionOpportunity`

An immutable observation that the current state may warrant a prediction.

Required semantics:

- source `snapshot_id`;
- trigger kind, such as `typing_pause`, `edit_applied`, `cursor_jump`,
  `diagnostic_change`, or `explicit_reveal`;
- trigger timestamp and render deadline;
- eligibility decision and reason;
- superseded opportunity identifier, when any; and
- cheap policy signals used by the gate.

An opportunity is not synonymous with a keystroke. Adapters may observe many
editor events without emitting an opportunity.

## `ContextPack`

A request input compiled from one snapshot under an explicit context policy.
It is named separately because context choice must be evaluated independently
from model choice.

Required semantics:

- source snapshot and `DocumentRevision` values;
- context-policy identifier and immutable revision;
- ordered context items;
- for each item, source kind, content digest, inclusion reason, token count,
  persistence class, source revision, and freshness role;
- total token and byte counts;
- redaction or omission results; and
- optional task-context digest and bounded selected fields.

A context pack must not contain an unrecorded repository retrieval result.
Task context is optional, read-only input. A task identifier alone is not
permission to persist or export source.

Every context item declares one freshness role:

- `application_critical` — a changed target revision makes the candidate stale;
- `display_critical` — a changed dependency suppresses presentation but does
  not claim that an already reviewed target transaction changed; or
- `advisory` — a change is recorded as drift and may reduce confidence without
  silently rewriting the candidate.

The policy records how each role behaves. It may choose a stricter result, but
must not silently downgrade an application-critical dependency.

## `DispatchDecision`

The policy decision to attempt inference, distinct from the inference request
and from the later candidate-display decision.

Required semantics:

- source opportunity identifier;
- active `ConfigurationSnapshot` identifier;
- prediction, context, protocol, and routing policy revisions;
- decision: `abstain`, `dispatch`, `shadow_dispatch`, or `defer`;
- selected explicit capability or standalone executor;
- purpose and visible/shadow mode;
- relative queue, inference, and render budgets;
- reason codes and experiment assignment; and
- `attempt_group_id`, attempt ordinal, relation to prior attempts, and retry,
  race, fallback, or escalation reason when applicable.

One common opportunity may yield several dispatch decisions for replay or
shadow comparison. Each is observable. A fallback is never a mutation of a
prior request.

## `ExecutionGrant`

A finite, pre-dispatch authorization compiled before source-bearing content is
serialized or sent across a process or network boundary.

Required semantics:

- source dispatch decision and context-pack identifiers;
- destination and operator trust domain;
- explicit capability and protocol revision;
- purpose and visible/shadow mode;
- allowed content classes and protected-content result;
- independent grants for runtime read, executor dispatch, persistence, replay,
  export, training, task context, and outcome correlation;
- effective policy digest, grant issuer, issued time, and expiry or one-shot
  consumption rule; and
- decision and denial reason codes.

An alias, configured endpoint, task identifier, fleet policy, or previous grant
is not authorization. Unknown policy input fails closed. The resolved
model/runtime identity may arrive later as `ServingObservation`, but the
destination trust domain and permitted content must be known before dispatch.
A local runtime-read grant precedes context access; a content-bound dispatch
grant binds the selected digests/classes before serialization. They are
separate immutable authorizations even when one policy engine issues both.

## `PredictionRequest`

A single, explicit inference attempt.

Required semantics:

- source opportunity and context-pack identifiers;
- active `ConfigurationSnapshot` identifier inherited from the dispatch;
- prediction-policy identifier and immutable revision;
- protocol identifier and immutable revision;
- explicit capability alias or standalone endpoint identity;
- output token or edit budget;
- deterministic generation settings where supported;
- source `DispatchDecision` and consumed `ExecutionGrant` identifiers;
- relative deadline budgets, dispatch timestamp, and cancellation identifier;
- attempt group, ordinal, relation, and reason; and
- experiment assignment, if the request is shadowed or part of an A/B test.

One request names one capability. If policy chooses another capability after a
failure or abstention, that is another request linked by an explicit policy
decision. Silent substitution is forbidden.

## `ServingObservation`

Evidence returned by or joined from the inference executor.

Required semantics when available:

- request correlation identifier;
- resolved model repository and immutable revision;
- tokenizer and prompt/template identity;
- quantization or conversion identity;
- runtime image/revision and relevant runtime flags;
- hardware class and executor identity at the permitted disclosure level;
- queue, prefill/TTFT, decode, and total durations;
- generated token counts and cache state; and
- terminal status such as `completed`, `cancelled`, `deadline_exceeded`,
  `unavailable`, or `failed`.

Missing fields remain explicitly unknown. They are never inferred from an
alias or a healthy endpoint.

## `PredictionCandidate`

A normalized proposed change independent of the model's native output format.

Required semantics:

- source request identifier;
- exact base document identifiers, versions, and content digests;
- one or more non-overlapping normalized text edits;
- proposed next focus location, if any;
- model-native output digest retained for reproducibility when policy permits;
- parse, range, scope, and diagnostic validation results;
- generation completion and normalization timestamps; and
- candidate status.

Each normalized text edit contains:

```text
base_document_revision_id
range_start
range_end
position_encoding
range_end_semantics
replacement_content or governed content_ref
replacement_digest
```

The candidate declares edit ordering and rejects overlap after normalizing all
ranges into the base revision's position encoding. Native model output is
untrusted protocol input: parsers apply byte, nesting, edit-count, replacement,
and time limits. Raw protocol text, control sequences, or markup is never
rendered as editor UI or executed.

Candidates can have these terminal states:

- `valid`
- `empty`
- `invalid_protocol`
- `invalid_range`
- `invalid_syntax`
- `out_of_scope`
- `stale`
- `cancelled`
- `expired`
- `failed`

The validator may report syntax or diagnostic information without claiming
semantic correctness.

## `PredictionDecision`

The policy decision about a candidate, separate from candidate generation.

Required semantics:

- source opportunity and considered candidate identifiers;
- decision-policy identifier and immutable revision;
- decision: `show`, `suppress`, `no_candidate`, `stale`, or `expired`;
- reason codes and recorded scores;
- selected presentation mode;
- decision timestamp and remaining deadline budget; and
- experiment assignment.

Presentation modes are capability declarations, not editor assumptions:

- `inline`
- `local_diff`
- `next_location`
- `cross_file_preview`
- `explicit_reveal`

The editor adapter may decline an unsupported presentation mode. It must not
silently downgrade a cross-file review into an automatic edit.

## `PresentationAttempt`

An append-only record of what the adapter tried to render after a `show`
decision.

Required semantics:

- source decision and candidate identifiers;
- adapter capability and presentation mode requested;
- fresh target and display-critical dependency revisions checked;
- attempt timestamp, render duration, and remaining budget;
- terminal status: `displayed`, `unsupported`, `stale`, `expired`, `failed`, or
  `suppressed_by_adapter`; and
- adapter reason code and rendered-content digest when policy permits.

A `show` decision is not proof that the user saw a suggestion. Funnel metrics
count a display only from a successful presentation attempt.

## `ApplicationAttempt`

The adapter's conditional transaction after an attributable user gesture.

Required semantics:

- source presentation, decision, and candidate identifiers;
- gesture kind and attribution policy;
- expected `DocumentRevision` values for every target;
- transaction mode: `single_document_conditional`, future
  `multi_document_atomic`, or explicit `per_document_review`;
- attempted edit ordering and digests;
- post-application document revisions; and
- terminal status: `applied`, `partially_applied`, `stale`, `denied`,
  `unsupported`, or `failed`.

The v0 supports only `single_document_conditional`. A stale or mismatched
revision terminates the attempt; the adapter does not offset-repair it.

## `SurvivalObservation`

An outcome checkpoint attached to an applied edit without mutating the original
application evidence.

Required semantics:

- application and candidate identifiers;
- checkpoint definition and scheduled time;
- observation time and censoring status;
- retained-content digest/distance when permitted;
- observed save or commit correlation with its ambiguity; and
- status such as `survived`, `rewritten`, `removed`, `right_censored`, or
  `attribution_lost`.

## `ObservedOutcome`

An append-only human/editor observation linked to a presentation or application
attempt. Outcomes may arrive over time; one mutable final row is insufficient
for audit and survival analysis. Specialized presentation, application, and
survival records remain distinct so a policy decision cannot be mistaken for a
render, a user gesture, or durable utility.

Outcome kinds include:

- `accepted`
- `partially_accepted`
- `dismissed`
- `ignored_until_superseded`
- `explicitly_rejected`
- `undone`
- `rewritten`
- `saved`
- `commit_correlated`
- `attribution_lost`

Required semantics:

- presentation/application attempt and candidate identifiers, when applicable;
- editor `DocumentRevision` before and after attributable action;
- outcome timestamp and attribution window/policy;
- accepted or retained edit digest and distance metrics, when permitted;
- confidence or ambiguity of attribution.

`ignored` is time- and policy-dependent. Lack of acceptance is not automatically
an explicit rejection. Commit correlation is a proxy and may become
unavailable after rebases, squashes, or later rewrites.

## Cross-cutting invariants

### Version fencing

An edit may be presented against or applied to a document only when its base
`DocumentRevision` and the required context-dependency revisions still satisfy
their declared freshness roles. A mismatch terminates or suppresses according
to that role. There is no best-effort offset adjustment in the foundational
contract.

### Authorization before serialization

Core resolves a finite `ExecutionGrant` before constructing a wire payload for
another process or host. Policy resolution is deterministic: deny rules union,
allowlists intersect, the shortest permitted retention wins, a local pause
wins, fleet policy may narrow but never broaden local grants, and unknown input
fails closed. The effective grant and policy digest are evidence.

### User authority

The runtime returns suggestions. The editor owns the user gesture and the
actual application transaction. The v0 does not execute terminal actions or
silently apply cross-file edits.

### Cancellation

A newer relevant snapshot cancels pending work tied to the older version.
Cancellation must propagate to context building and inference when supported.
A late result is still fenced even when upstream cancellation fails.

### Deadlines

Every opportunity and request has an explicit deadline. Work completing after
that deadline may be retained as shadow evidence if policy permits, but it is
not shown as an interactive result.

Across processes, deadline enforcement uses transmitted remaining-duration
budgets. Wall clocks support reporting only. A receiver records exhausted or
negative budgets rather than reconstructing a deadline from an unrelated
clock.

### Causality, retries, and idempotency

Producer sequence and causal links establish lifecycle order. Consumers apply
idempotency keys before materializing a retry, preserve out-of-order evidence,
and do not infer missing intermediate events. A superseding correction points
to the prior immutable record.

### Identity

Model alias is not model identity. Reproducible evidence joins the request to
the resolved immutable model, tokenizer, runtime, protocol, context policy,
prediction policy, and trace manifest.

### No hidden fallback

An unavailable or failed selected capability returns an observable failure.
Any retry, fallback, race, or escalation is an explicit Edit policy action with
a separate request and outcome.

### Configuration activation

External desired state is never consulted synchronously during a prediction.
Core validates, compiles, and atomically activates a complete configuration
snapshot before it becomes eligible for dispatch. Unknown incompatible majors,
same-generation digest conflicts, incomplete bundles, failed preflight, or
local-policy broadening leave the prior verified snapshot active. A crash with
uncertain activation becomes `indeterminate` and is resolved by reading Core's
exact active identity rather than replaying blindly.

### Content minimization

Source-bearing data is not copied into ordinary logs, errors, metrics labels,
or fleet events. Persistence and export follow the classes in
`PRIVACY-AND-TRUST.md`.

Resolved logical URIs, protected paths, selected context, native requests,
model output, and replacement text are untrusted inputs. The pipeline applies
protected-content filtering after URI/path resolution and applies bounded
parsing, secret/protected-content checks, Unicode control/bidirectional policy,
and plain-text editor rendering before a candidate can be shown.

### Immutable evidence and authorized erasure

Requests, serving observations, decisions, and outcomes are immutable evidence
records during their authorized retention. Corrections append a superseding
record. A deletion grant physically erases source-bearing content and linkable
metadata/derived indexes covered by the request, including permitted backup and
export copies. A minimal non-linkable deletion receipt may remain; it must not
retain content digests or stable identifiers that recreate the deleted graph.
Materialized summaries may be rebuilt only from still-authorized evidence.

### Schema evolution

Wire and durable schemas are versioned independently. Readers reject unknown
incompatible major versions. Durable stores require tested forward migration
and a documented export path before a release can change their schema.

## Integration contracts

### Evidence ownership seam

| Evidence | Authority |
| --- | --- |
| Opportunity selection, context/protocol/prediction/display policy, normalization, TTRS, fencing, deadline outcome, and human outcomes | Anvil Edit / Lab |
| Resolved model, tokenizer, runtime, quantization, hardware, queue, prefill, decode, capacity, and executor termination | Executor or Anvil Serving |

The two sides join through one canonical request correlation identifier and an
immutable joined manifest. Missing executor evidence remains missing. When two
authorities disagree, the report retains both observations, names the conflict,
and fails any gate that depends on the disputed field.

### Anvil Serving

Anvil Edit sends an explicit capability alias and correlation identifier.
Serving executes the selected route, rejects an unavailable route rather than
substituting another, and exposes bounded serving identity and timing evidence.
Edit owns semantic selection and outcome interpretation.

Example aliases such as `edit.fast` or `edit.semantic` are documentation
placeholders. Their presence here is not proof that any route is configured,
deployed, healthy, or qualified.

### Anvil State

Edit may read a bounded task packet or digest as context when the developer has
enabled the integration. It does not mutate task state in the hot path and does
not store editor telemetry in State. A later promotion report may be attached
as durable project evidence through a separately authorized workflow.

### Anvil Events

Events may eventually converge one immutable, source-free Anvil Edit
configuration bundle per managed release channel. The initial desired resource
is `edit/config/<channel>` and the reserved adapter name is
`anvil_edit_config`. These are design identifiers, not implemented or deployed
resources.

The desired event names one authority-assigned generation, immutable revision,
artifact digest, and logical artifact reference. It carries no configuration
body, source, prompt containing captured source, endpoint, credential, path,
trace identifier, or behavioral outcome. The complete bundle is the atomic
activation unit until a separately tested multi-resource activation contract
exists.

Core treats the bundle as policy input, not authorization. Local pause and
repository/destination permissions win; fleet policy may narrow but never
widen runtime read, dispatch, persistence, replay, export, training, shadow,
task-context, or outcome-correlation permission. A future adapter may report
`reconcile.applied` only after Core reports the exact desired authority,
resource, generation, revision, artifact digest, and adapter as active together
with the exact effective `ConfigurationSnapshot` identifier and digest.

Events delivery and reconciliation remain outside the prediction hot path.
Per-opportunity, per-request, per-keystroke, source-bearing, and human-outcome
records stay in Edit's local data plane. The full future contract and its
explicit non-implementation status are in
[`integrations/anvil-events.md`](integrations/anvil-events.md).

### Anvil Workbench

Workbench may consume redacted aggregate benchmark and promotion evidence. It
is not required for prediction and receives no raw source-bearing trace by
default.

### Standalone executor

Core and Lab may target a directly configured local executor. Standalone mode
must preserve explicit endpoint/model identity, cancellation, deadlines,
protocol revision, and serving observations to the extent the executor exposes
them. Missing evidence is reported as missing rather than synthesized.
