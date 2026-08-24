# Project: Anvil Flow v0

## Summary

For privacy-sensitive developers working through sequences of related edits,
deliver the first restrained predictive-editing experience in one supported
editor, using Anvil Edit Core and Lab to show one explicit fast capability,
conditionally apply only exact-revision local edits, expose privacy and model
controls, and run a temporally separated exploratory and confirmatory dogfood
program.

## Goals

- Keep the developer in flow with bounded inline/local-diff suggestions and an
  explicit-reveal/subtle interaction.
- Produce zero stale applications while making cancellation and unsupported
  editor capabilities observable.
- Make prediction destination, model capability, trace state, pause, deletion,
  and context rationale inspectable at the point of use.
- Calibrate interruption and utility in an exploratory reveal pilot, then test
  a frozen policy in a randomized visible comparison.
- Record a second-editor capability matrix and a concrete initial adoption
  hypothesis without overstating cross-editor portability.

## Non-Goals

- Apply cross-file or multi-document edits.
- Execute terminal commands, tests, or agent actions.
- Show a semantic lane, ensemble, race, or hidden fallback in the initial
  visible path.
- Persist or export source-bearing traces by default.
- Rank developer productivity or expose individual behavioral telemetry to a
  team dashboard.
- Claim support for an editor from provider/executor compatibility alone.
- Train or personalize model weights.

## Requirements

- R001: Flow shall choose its first editor through a capability audit covering `DocumentRevision`, position encoding, event order, cancellation, presentation, conditional application, outcomes, timing, IPC, and authorization rather than distribution alone.
- R002: The adapter shall emit portable document revisions and causal events for snapshot, opportunity, presentation, application, outcome, and survival without synthesizing unavailable semantics.
- R003: The visible baseline shall use one explicit fast capability and native protocol; alternatives shall remain replay or shadow and shall never silently replace the visible capability.
- R004: Every visible request shall carry a relative render budget and cancellation identity, and a newer application-critical revision shall cancel/suppress obsolete work.
- R005: Flow shall offer inline preview, local diff, and explicit reveal/subtle mode when the editor supports them and shall record unsupported or failed presentation attempts.
- R006: A user gesture shall trigger a single-document conditional application against the exact expected revision; mismatch shall apply nothing and record stale.
- R007: Raw protocol/model output shall never be rendered directly; only bounded normalized plain-text edits that pass scope, range, protected-content, and revision checks may reach presentation.
- R008: The in-editor controls shall show prediction on/off, current explicit capability and local/remote destination, recording state, pause, repository policy, and deletion access.
- R009: Runtime prediction shall not imply trace persistence; source-bearing capture shall require an allowlisted repository, named purpose, visible retention/location, protected-path evaluation, and accessible pause/deletion controls.
- R010: Flow shall record distinct decision, presentation, application, undo/rewrite, partial acceptance, attribution-loss, and fixed-checkpoint survival evidence for Lab.
- R011: Operational reports shall include TTRS distribution with renderable coverage, deadline/failure/cancellation/stale rates, display and explicit-reveal rates, visible duration, accepted volume, undo/rewrite, and censor-aware survival.
- R012: The first visible study shall begin with an explicit-reveal exploratory pilot; metrics, thresholds, exclusions, assignment, clustering, and stopping shall then freeze before a temporally separate randomized visible comparison.
- R013: Visible assignment shall occur on the common opportunity stream before policy gating and shall retain intent-to-treat outcomes, including abstention, failure, and no-candidate paths.
- R014: Flow shall support a local emergency pause that disables prediction independently from evidence-preservation and deletion controls.
- R015: Team or exported product-quality evidence shall be aggregate and shall omit stable individual/session/repository IDs, exact timestamps, cadence, paths, digests, and free text while suppressing sparse cohorts.
- R016: A second-editor spike shall report full lifecycle capabilities separately from executor/provider compatibility, and any missing capability shall narrow the supported-editor claim.
- R017: Flow shall not expose a semantic/gated route until shadow evidence nominates it for a separately predeclared visible experiment under a later PRD or amendment.
- R018: Phase 0 and the final product decision shall record the initial user/buyer, current workaround, setup tolerance, privacy threshold, trace-consent posture, and whether demand centers on Flow, Lab, or both.
- R019: The initial opportunity gate shall be a deterministic finite-state policy with bounded debounce/hysteresis, typing-velocity summary, duplicate suppression, and rate/concurrency control; every eligibility, coalescing, or suppression result shall carry source-free reasons and configured limits.
- R020: Adapter-to-Core session coordination shall use bounded handoff and return asynchronous results with their original configuration, exact revision, local generation, cancellation, and remaining-deadline pins; completion order shall not become event order.

## Acceptance Criteria

- The selected editor capability report contains evidence and a proving probe
  for every R001 dimension.
- Rapid typing, file switches, close/reopen, rename, UTF-16/non-BMP text, and
  line-ending changes produce zero wrong-revision application.
- Repeated equivalent editor events are coalesced or suppressed deterministically
  within declared bounds, and mailbox pressure cannot grow without limit or
  silently reorder lifecycle evidence.
- A selected-capability failure produces an observable failure or abstention;
  no alternate model appears under the same request identity.
- Unsupported presentation modes are recorded and never silently converted
  into automatic or broader-scope edits.
- Content-disabled dogfood stores no source text, raw prompt/output, path,
  secret, or reconstructable replacement outside permitted runtime memory.
- Users can see and change prediction/recording state, pause immediately, and
  initiate deletion without leaving the editor's relevant settings surface.
- The exploratory pilot cannot satisfy the confirmatory gate, and the later
  comparison preserves all assigned opportunities in intent-to-treat reports.
- TTRS is always reported with renderable coverage and terminal non-render
  paths; durable outcomes report right-censoring and attribution loss.
- The second-editor report calls an endpoint-only integration executor
  compatibility, not Anvil Edit portability.
- The product decision may be go, narrow, stop, or inconclusive and cites both
  adoption and technical evidence.

## Risks

- Ghost text can be objectively fast and still interrupt the developer enough
  to invalidate offline quality gains.
- Extension API limitations may make presentation or outcome attribution
  incomplete and bias the funnel.
- A one-developer pilot can overfit thresholds, editing style, repository, and
  time period.
- UI controls can imply safety while background processes, remote endpoints, or
  storage destinations remain difficult to inspect.
- Candidate latency can be improved by suppressing slow opportunities; coverage
  and intent-to-treat reporting must expose that tradeoff.
- A provider integration can create premature marketing pressure for an
  unsupported cross-editor claim.

## Open Questions

- Which editor provides the strongest complete v0 lifecycle and acceptable
  extension distribution/setup burden?
- Which exact inline/diff/reveal interaction is least disruptive for the first
  developer population?
- What minimum local indicator communicates prediction versus source recording
  without creating persistent visual noise?
- What initial adoption signal is strong enough to continue when the technical
  experiment is positive but the install or consent burden is high?

## Assumptions

### A001: Flow v0 uses one visible specialized fast bundle.

**Rationale:** A single baseline isolates adapter, latency, candidate, and UX behavior before any routing complexity.

**Requirements:** R003, R004, R017

### A002: Explicit reveal precedes ambient visible dogfood.

**Rationale:** Reveal mode provides a lower-interruption way to find rendering, attribution, and candidate failure modes before freezing a randomized test.

**Requirements:** R005, R010, R012, R013

### A003: The first editor is the only supported production surface in v0.

**Rationale:** A second-editor spike measures portability but does not become a production adapter merely because an executor protocol connects.

**Requirements:** R001, R016

## Features

### F001: Supported editor adapter

Implements the audited document, event, cancellation, presentation,
conditional-application, outcome, IPC, and authorization contract.

**Requirements:** R001, R002, R004, R006, R016, R020

### F002: Restrained prediction experience

Shows one explicit fast bundle through bounded normalized inline/diff/reveal
interactions with no hidden substitution or broader action scope.

**Requirements:** R003, R005, R007, R017, R019

### F003: Local controls and outcome instrumentation

Exposes model/destination and recording controls while emitting complete,
source-minimized lifecycle and latency evidence.

**Requirements:** R008, R009, R010, R011, R014, R015

### F004: Staged dogfood and product validation

Runs separate exploratory and confirmatory studies, records a second-editor
matrix, and makes a technical plus adoption decision.

**Requirements:** R012, R013, R016, R018

## Tasks

### T001: Audit and select the first editor adapter

**Feature:** F001
**Priority:** critical
**Likely files:** docs/adapters, docs/DECISIONS.md, tests/adapter_contract

Run the complete adapter capability matrix against primary APIs and minimal
probes. Record the first editor decision, missing capabilities, and bounded
workarounds without treating provider compatibility as lifecycle support.

**Acceptance criteria:**

- Every R001 capability is supported, partial, missing, or unknown with evidence.
- The selected editor can enforce a single-document conditional application.
- Unsupported capabilities narrow v0 scope and are reflected in product docs.

**Verification:**

- `python -m anvil_edit.adapter_contract docs/adapters/first-editor.yaml`
- `pytest tests/adapter_contract -q`

### T002: Implement snapshot, opportunity, cancellation, and Core transport

**Feature:** F001
**Priority:** critical
**Likely files:** adapters/first-editor, src/adapter_sdk, tests/flow/test_capture.py
**Dependencies:** T001

Connect the editor to Core with portable revisions, causal events, authenticated
local IPC, relative budgets, protected-path checks, and cancellation. Feed the
bounded single-writer session coordinator and implement the deterministic
opportunity FSM without moving model inference into the gate.

**Acceptance criteria:**

- Reopen/rename/untitled/UTF-16/EOL cases emit unambiguous revisions.
- New application-critical revisions cancel or suppress obsolete work.
- Duplicate/debounce/rate-limit behavior is deterministic, bounded, and emits
  source-free policy reasons.
- No source is serialized without a valid destination grant.

**Verification:**

- `pytest tests/flow/test_capture.py tests/flow/test_cancellation.py -q`
- `pytest tests/security/test_adapter_transport.py -q`

### T003: Implement inline, diff, and explicit-reveal presentation

**Feature:** F002
**Priority:** high
**Likely files:** adapters/first-editor/src/presentation, tests/flow/test_presentation.py
**Dependencies:** T002

Render only normalized plain-text candidates through the adapter's declared
inline/local-diff/reveal capabilities and emit presentation attempts.

**Acceptance criteria:**

- Raw native output and unsafe control/bidirectional sequences never render.
- Unsupported and stale presentation attempts are recorded distinctly.
- Reveal mode can run without persistent ambient ghost text.

**Verification:**

- `pytest tests/flow/test_presentation.py tests/security/test_rendering.py -q`

### T004: Implement conditional application and outcome capture

**Feature:** F001
**Priority:** critical
**Likely files:** adapters/first-editor/src/application, tests/flow/test_application.py, tests/flow/test_outcomes.py
**Dependencies:** T002, T003

Fence the user gesture against the exact revision and emit separate application,
partial acceptance, undo/rewrite, attribution, and survival observations.

**Acceptance criteria:**

- A mismatched revision applies nothing and records stale.
- Presentation, user gesture, and successful application cannot be conflated.
- Fixed checkpoints represent survival, censoring, and attribution loss.

**Verification:**

- `pytest tests/flow/test_application.py tests/flow/test_outcomes.py -q`

### T005: Add local prediction and trace controls

**Feature:** F003
**Priority:** critical
**Likely files:** adapters/first-editor/src/controls, src/core/policy, tests/flow/test_controls.py
**Dependencies:** T002

Expose prediction state, explicit capability/destination, recording indicator,
pause, repository policy, deletion, and inspectable context-selection reasons.

**Acceptance criteria:**

- Prediction and source recording are visibly independent controls.
- Local pause takes effect before the next dispatch and cannot be broadened by
  fleet or destination settings.
- Source capture cannot enable without purpose, retention/location, protected
  rules, and deletion access.

**Verification:**

- `pytest tests/flow/test_controls.py tests/privacy/test_flow_capture.py -q`

### T006: Instrument complete latency and interruption evidence

**Feature:** F003
**Priority:** high
**Likely files:** adapters/first-editor/src/telemetry, src/lab/metrics, tests/flow/test_metrics.py
**Dependencies:** T003, T004, T005

Emit timing and funnel observations required for TTRS, coverage, failures,
display/reveal, visible duration, acceptance, undo/rewrite, and survival.

**Acceptance criteria:**

- TTRS decomposes trigger through render without comparing unrelated clocks.
- All terminal non-render paths remain in the opportunity funnel.
- Source-free telemetry contains no raw content, path, or stable team identity.

**Verification:**

- `pytest tests/flow/test_metrics.py tests/privacy/test_flow_telemetry.py -q`

### T007: Run the explicit-reveal exploratory pilot

**Feature:** F004
**Priority:** high
**Likely files:** experiments/flow-v0/exploratory, docs/evidence
**Dependencies:** T004, T005, T006

Run a consented local reveal-only pilot to find interaction, attribution,
latency, privacy, and instrumentation failures, then freeze the confirmatory
protocol without claiming promotion evidence.

**Acceptance criteria:**

- The report is labeled exploratory E5 and cannot satisfy the visible gate.
- Primary metrics, thresholds, exclusions, assignment, clusters, censoring, and
  stopping rules are frozen before the next window.
- Unresolved failure modes block the confirmatory start.

**Verification:**

- `python -m anvil_edit.lab report experiments/flow-v0/exploratory/manifest.json`
- `pytest tests/experiments/test_protocol_freeze.py -q`

### T008: Run confirmatory dogfood and record the product decision

**Feature:** F004
**Priority:** high
**Likely files:** experiments/flow-v0/confirmatory, docs/adapters/second-editor.yaml, docs/DECISIONS.md
**Dependencies:** T007

Run the later randomized visible comparison on the common opportunity stream,
complete censor-aware outcome reporting, audit a second editor, and record a
go/narrow/stop/inconclusive decision with adoption evidence.

**Acceptance criteria:**

- Confirmatory observations do not overlap the exploratory calibration window.
- Intent-to-treat includes abstention, failures, deadlines, and no-candidate.
- The second-editor report distinguishes executor from full lifecycle support.
- The final decision cites user/buyer, workaround, setup/privacy/consent, Flow
  versus Lab pull, and the complete technical evidence class.

**Verification:**

- `python -m anvil_edit.lab report experiments/flow-v0/confirmatory/manifest.json`
- `python -m anvil_edit.adapter_contract docs/adapters/second-editor.yaml`
