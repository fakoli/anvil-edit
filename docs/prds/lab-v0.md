# Project: Anvil Edit Lab v0

## Summary

For model and inference engineers deciding which predictive-editing system to
ship, build a permission-aware record/replay and experiment system that
evaluates complete bundles on identical opportunities, joins executor and human
evidence without inventing missing facts, and produces reproducible reports
that distinguish offline concordance, shadow behavior, visible utility, durable
outcomes, deployment, and promotion.

## Goals

- Replay byte-identifiable permitted opportunities through pinned native model
  and policy bundles.
- Let an engineer run a locked manifest and receive a report whose inputs,
  denominators, evidence class, missingness, and gate outcome are inspectable.
- Make latency, candidate quality signals, human outcomes, interruption,
  censoring, and compute cost separately inspectable.
- Compare at least two specialized next-edit candidates and one control without
  calling a protocol/runtime bundle a model-only result.
- Support exploratory, confirmatory, and inconclusive experiment outcomes.
- Produce immutable promotion manifests that can be joined to executor evidence
  and independently reproduced at the permitted disclosure level.

## Non-Goals

- Prove human utility from replay, exact match, or an LLM judge.
- Collect raw source by default or turn existing traces into training data.
- Own visible editor presentation or conditionally apply edits.
- Automatically deploy or promote a model, route, or policy.
- Establish a universal public ranking across different protocols, hardware,
  corpora, editors, or privacy populations.
- Train a custom model in v0.

## Requirements

- R001: Every replay and experiment shall pin corpus provenance/permission, sample selection/exclusions, editor adapter and revision contract, active `ConfigurationSnapshot`, any externally desired configuration revision as separate evidence, complete model/tokenizer/protocol/context/runtime/quantization/validation bundle, hardware/runtime state, policy revisions, and raw artifact digests.
- R002: Lab shall ingest only traces and source-bearing content covered by an unexpired replay grant and shall preserve purpose-scoped identifiers and deletion lineage.
- R003: Replay shall reconstruct the same locked opportunity inputs and report nondeterminism or missing content rather than silently substituting current repository state.
- R004: E3 replay shall report candidate concordance and fixed-opportunity policy simulation and shall not be labeled counterfactual human utility.
- R005: Lab shall join Edit-owned opportunity/context/policy/normalization/TTRS/outcome evidence with executor-owned identity/queue/prefill/decode/termination evidence through one canonical correlation ID while preserving missing and conflicting claims.
- R006: Funnel reports shall retain O/E/R/C/S/A/V/K denominators plus renderable coverage, no-candidate, failure, deadline, stale, cancellation, and attribution-loss counts.
- R007: Latency reports shall separate trigger, capture, context, queue, TTFT/prefill, decode, validation, render, decision, TTRS, and post-hoc TTUS, and shall not gate on p99 below the declared sample rule.
- R008: Native comparisons shall be labeled system-bundle comparisons unless every non-model factor is held fixed.
- R009: Live experiments shall assign from a common opportunity stream before policy-specific gating, preserve intent-to-treat analysis, and record assignment unit, randomization, developer/repository/session clusters, exclusions, and stopping rules before exposure.
- R010: Exploratory explicit-reveal dogfood and confirmatory randomized visible comparison shall use temporally separate data, with metrics and thresholds frozen between them.
- R011: Survival shall be observed at declared checkpoints with right-censoring and attribution loss; save and commit correlation shall be labeled according to their ambiguity.
- R012: LLM-judge signals shall remain secondary, use pinned judge identity with blinded/randomized order, and publish calibration against a declared human sample.
- R013: Reports shall label their highest E0-E7 evidence class, distinguish discovery/local replay/shadow/visible/deployment/promotion, and permit an inconclusive result.
- R014: Promotion reports shall require complete manifests, contract/privacy tests, local resource evidence, locked replay, live shadow, a predeclared visible comparison, censor-aware durable outcomes, tested rollback/migration behavior, and human approval of exact scope.
- R015: Lab shall support authorized physical erasure of source content, linkable metadata, derived indexes/reports, and governed export copies without leaving a cross-repository correlation graph.
- R016: Public reports shall contain only portable sanitized evidence; private traces, topology, active routes, and raw operator evidence shall remain outside the public repository.
- R017: Reports shall distinguish configuration desired, received, staged, active, used by a request, executor-deployed, and policy-promoted states and shall not infer one from another.
- R018: Replay shall apply producer-scoped idempotency and stable causal topological traversal, preserve independent branches without claiming global order, and report missing parents, producer-sequence gaps, cycles, duplicate identities, and conflicting evidence explicitly.
- R019: Live bounded-memory latency summaries may use HDR histograms and online moments, but authoritative Lab quantiles shall use permitted raw or lossless observations; checkpoint survival shall retain right-censoring and use declared Kaplan-Meier risk sets for descriptive survival.

## Acceptance Criteria

- A locked fixture replay produces the same canonical request/candidate inputs
  from byte-identifiable artifacts or fails with a named missing/nondeterministic
  cause.
- Reordering delivery of the same causal fixture produces the same materialized
  output, while an independent-branch tie-breaker is labeled deterministic
  ordering rather than inferred causality.
- A three-bundle campaign records two native next-edit candidates and one
  control with exact immutable identities and protocol differences visible.
- Metric fixtures prove every funnel denominator, deadline/cancellation path,
  renderable coverage, cluster assignment, and right-censor outcome.
- Reports refuse to call E3 replay a human counterfactual and refuse to use an
  undersampled p99 as a promotion gate.
- A retry, missing Serving observation, or conflicting timing/identity claim is
  preserved in the joined manifest and cannot be silently repaired.
- A desired Events generation that was never activated or used cannot appear as
  the configuration identity of a replay, benchmark, or promotion report.
- LLM-judge output is blinded/order-randomized, secondary, and accompanied by a
  reproducible human-calibration sample when used.
- Exploratory and confirmatory datasets cannot overlap under the manifest's
  time/population split.
- An inconclusive visible experiment produces a complete report without being
  coerced into pass or fail.
- Deletion of an authorized trace invalidates dependent replays/reports or
  rebuilds them from still-authorized aggregate evidence, with failures named.

## Risks

- Model-native protocols can dominate comparisons and encourage misleading
  model-only headlines.
- Replay can overfit to one developer, repository, or time period while
  presenting precise but non-general causal-looking metrics.
- Low display rate, deadline failure, or censoring can disappear from
  conditional latency/utility distributions.
- Judge models can reward verbosity, style, or self-similarity rather than
  developer utility.
- Trace permissions and deletion can make historical reports unreproducible;
  the correct outcome is explicit evidence loss, not retained unauthorized data.
- Small dogfood populations may require N-of-1 crossover designs and may still
  yield an inconclusive result.

## Open Questions

- Which first public/synthetic controls and consented trace partitions provide
  useful language/edit diversity without mixing disclosure classes?
- Which semantic equivalence checks are affordable and sufficiently calibrated
  to complement exact/delta/location metrics?
- What initial cluster-aware sample size and stopping rules are feasible for a
  one-developer dogfood period?
- Which report artifact format best supports local inspection and optional
  future Workbench display without making Workbench a dependency?

## Assumptions

### A001: The first Lab campaign compares complete native bundles.

**Rationale:** Preserving each candidate's intended protocol is more useful than forcing artificial prompt uniformity, provided the report names every changed component.

**Requirements:** R001, R003, R008

### A002: Real source-bearing traces are optional and locally controlled.

**Rationale:** Contract fixtures and permissive controls can build the runner; real traces enter only when replay permission, retention, and deletion are explicit.

**Requirements:** R002, R015, R016

### A003: Commit correlation is exploratory in v0.

**Rationale:** Rebases, squashes, later rewrites, and observation-window endings can break attribution; fixed earlier survival checkpoints are more defensible.

**Requirements:** R011, R013

## Features

### F001: Pinned corpus and replay kernel

Creates permission-aware manifests and deterministic replay across complete
native system bundles.

**Requirements:** R001, R002, R003, R004, R008, R015, R016, R018

### F002: Metric and evidence-join engine

Computes complete funnels, latency/resource distributions, candidate signals,
and joined executor evidence without hiding missingness.

**Requirements:** R005, R006, R007, R011, R012, R019

### F003: Staged experiment protocol

Implements common-stream assignment, exploratory/confirmatory separation,
cluster-aware intent-to-treat analysis, censoring, and inconclusive outcomes.

**Requirements:** R009, R010, R011, R013, R019

### F004: Reproducible reports and promotion gates

Produces immutable, disclosure-aware benchmark and promotion artifacts with no
deployment side effect.

**Requirements:** R013, R014, R015, R016

## Tasks

### T001: Define the Lab manifest and corpus permission schemas

**Feature:** F001
**Priority:** critical
**Likely files:** schemas/lab/v1, src/lab/manifests, tests/lab/test_manifest.py

Implement schemas for trace provenance/permission, locked sample selection,
complete system bundles, configuration snapshots and separate desired-state
provenance, editor revision contracts, assignments, raw artifacts, and deletion
dependencies.

**Acceptance criteria:**

- Every R001 identity is required or explicitly unknown where allowed.
- A replay grant is checked before any source-bearing corpus is opened.
- Manifest digests change on any material system-bundle or sample change.

**Verification:**

- `pytest tests/lab/test_manifest.py tests/privacy/test_replay_grant.py -q`
- `python -m anvil_edit.lab manifest validate tests/fixtures/lab`

### T002: Build deterministic locked replay

**Feature:** F001
**Priority:** critical
**Likely files:** src/lab/replay, tests/lab/test_replay.py
**Dependencies:** T001

Reconstruct context and protocol inputs from permitted artifacts, apply
idempotency and stable causal topological materialization, invoke a pinned
executor adapter, and retain deterministic/non-deterministic causes. Stable
tie-breakers must not be reported as causal edges.

**Acceptance criteria:**

- Identical inputs produce byte-identifiable request and candidate records.
- Missing parents, producer gaps, cycles, duplicate identities, and conflicts
  are named rather than repaired from wall time or current repository state.
- Missing content, policy, tokenizer, or runtime identity fails explicitly.
- E3 outputs are labeled concordance/simulation, never human counterfactuals.

**Verification:**

- `pytest tests/lab/test_replay.py -q`
- `python -m anvil_edit.lab replay tests/fixtures/lab/locked-manifest.json`

### T003: Join executor and Edit evidence without synthesis

**Feature:** F002
**Priority:** high
**Likely files:** src/lab/evidence, tests/lab/test_evidence_join.py
**Dependencies:** T001

Implement the canonical correlation join, ownership map, missing-field states,
conflict records, retry groups, and dependent-gate failure behavior.

**Acceptance criteria:**

- Edit and executor authorities remain distinguishable in the output.
- Missing identity/timing remains unknown rather than inferred from an alias.
- Conflicting required evidence blocks the corresponding conclusion.

**Verification:**

- `pytest tests/lab/test_evidence_join.py -q`

### T004: Implement funnel, latency, utility, and cost metrics

**Feature:** F002
**Priority:** critical
**Likely files:** src/lab/metrics, tests/lab/test_metrics.py
**Dependencies:** T001, T003

Implement O/E/R/C/S/A/V/K funnels, TTRS/TTUS, coverage/failure/cancellation,
candidate signals, resource cost, and raw-distribution outputs. HDR histograms
may support live diagnostics, but report quantiles remain reproducible from
permitted raw or lossless observations.

**Acceptance criteria:**

- Every rate reports numerator and denominator.
- TTRS is paired with renderable coverage and all terminal non-render paths.
- p99 is ineligible as a gate below 2,000 observations per stratum.

**Verification:**

- `pytest tests/lab/test_metrics.py -q`
- `pytest tests/lab/test_latency_quantiles.py -q`

### T005: Implement permission-aware corpus splits and erasure

**Feature:** F001
**Priority:** high
**Likely files:** src/lab/corpus, src/lab/erase, tests/privacy/test_lab_erasure.py
**Dependencies:** T001

Create time/population splits, disclosure partitions, purpose-scoped IDs, and
dependency-aware deletion across inputs, indexes, reports, and exports.

**Acceptance criteria:**

- Public, private, and source-free corpora cannot be silently co-reported.
- Exploratory and confirmatory windows cannot overlap.
- Erasure removes linkable derived state or marks dependent artifacts unusable.

**Verification:**

- `pytest tests/lab/test_corpus_splits.py tests/privacy/test_lab_erasure.py -q`

### T006: Add secondary semantic and judge evaluation

**Feature:** F002
**Priority:** medium
**Likely files:** src/lab/judges, tests/lab/test_judge_protocol.py
**Dependencies:** T002, T005

Implement optional syntax/semantic checks and a pinned, blinded,
order-randomized judge protocol with human calibration artifacts.

**Acceptance criteria:**

- Judge scores are never primary ground truth or a direct promotion gate.
- Candidate/system identity is hidden and order is reproducibly randomized.
- Reports include calibration sample size and agreement/bias measures.

**Verification:**

- `pytest tests/lab/test_judge_protocol.py -q`

### T007: Implement staged experiment assignment and survival analysis

**Feature:** F003
**Priority:** critical
**Likely files:** src/lab/experiments, tests/lab/test_assignment.py, tests/lab/test_survival.py
**Dependencies:** T004, T005

Assign before policy gating from a common stream, preserve intent-to-treat,
cluster observations, freeze analysis plans, and represent right-censoring and
attribution loss with declared Kaplan-Meier risk sets for descriptive survival.

**Acceptance criteria:**

- Assignment and cluster fields are emitted at exposure, not reconstructed.
- Exploratory data cannot satisfy the confirmatory gate.
- Missing checkpoints remain censored/lost observations rather than deletions.

**Verification:**

- `pytest tests/lab/test_assignment.py tests/lab/test_survival.py -q`

### T008: Produce benchmark and promotion reports

**Feature:** F004
**Priority:** high
**Likely files:** src/lab/reports, tests/lab/test_reports.py
**Dependencies:** T003, T004, T006, T007

Render machine-readable and human-readable reports with evidence class,
complete bundles, uncertainty, disclosure class, gate outcomes, and no deploy
side effect.

**Acceptance criteria:**

- Reports support pass, fail, and inconclusive outcomes.
- Promotion reports require every R014 gate and exact human-approved scope.
- Public output fixtures contain no private topology, source, route, path, or
  stable individual/repository identifier.

**Verification:**

- `pytest tests/lab/test_reports.py tests/privacy/test_public_report.py -q`
- `python -m anvil_edit.lab report tests/fixtures/lab/complete-run.json`
