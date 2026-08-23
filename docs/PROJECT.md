# Project definition

Status: **foundation / proposed product**

Last reviewed: **2026-08-23**

## One-sentence definition

Anvil Edit is a local-first, model-agnostic runtime and evaluation system that
turns developer activity into bounded prediction opportunities and measures
which explicit model and policy combinations produce useful edits under a
deadline.

## User problem

Developers regularly make sequences of related edits but must interrupt their
flow to restate intent to a chat or agent. Existing completion endpoints solve
only part of the problem. A predictive editing product must also reconstruct
recent intent, find the likely next location, decide whether to speak, meet an
interactive latency budget, avoid stale edits, and learn from durable outcomes.

Local-model users face an additional problem: token throughput and general
coding benchmarks do not tell them which next-edit model, context policy, or
presentation policy works on their code and hardware.

## Product promise

Anvil Edit should let a developer:

1. connect a supported editor to a local runtime;
2. choose or permit explicit model capabilities;
3. keep raw editing traces local unless they opt into another destination;
4. receive restrained, version-fenced edit suggestions;
5. compare models and policies on identical, permitted traces; and
6. promote only configurations supported by replay and visible dogfood
   evidence.

The product may improve retrieval, gating, or routing from outcomes. It must
not claim personalization or model learning until a held-out evaluation and
promotion path proves it.

Cross-editor portability is a product hypothesis, not an accepted property.
The first release supports one adapter deeply. A second-editor capability
matrix tests which parts of the Core contract port; executor compatibility
alone does not prove snapshot, cancellation, presentation, application, or
outcome portability.

## Primary users

### Local-first developer

Wants predictive editing without sending proprietary code or behavioral traces
to a hosted coding service.

### Model and inference engineer

Wants reproducible edit-task benchmarks that connect model/runtime performance
to actual developer outcomes rather than decode throughput alone.

### Tool builder

Wants an editor-independent prediction contract and adapters for specialized
next-edit models without rebuilding capture, cancellation, normalization, and
evaluation.

### Private platform operator, later

Wants controlled model and policy revisions, aggregate quality evidence, and
no individual productivity surveillance.

## Initial jobs to be done

- When I continue a local edit sequence, offer the next likely edit without
  making me explain the sequence.
- When I keep typing or change files, cancel obsolete work and never apply it
  to a newer buffer.
- When I evaluate a local model, replay the same opportunities through its
  native protocol and show latency, quality proxies, compute cost, and outcome
  evidence separately.
- When several models are available, make policy choice explicit and
  inspectable rather than silently substituting a different endpoint.
- When the system is uncertain, stay quiet or let me reveal a prediction
  without continuously interrupting the editor.

## Product principles

### Predictive runtime, not model wrapper

Context compilation, cancellation, validation, abstention, and outcome capture
are product behavior. They must not be incidental editor plug-in code.

### Evidence before intelligence claims

A larger or more capable general model is only a candidate. The relevant test
is useful edit outcome under the required latency, interruption, privacy, and
compute constraints.

### Quiet is a valid result

The opportunity gate and display decision are first-class policy. Suggestion
volume is not a success metric.

### Small, permitted context by default

Context is selected for a recorded reason. Larger context must demonstrate
incremental utility worth its latency and privacy cost.

### Explicit capabilities

Anvil Edit may select a fast or semantic capability. The serving layer must
execute that exact selection or return an error. A fallback is a new,
observable Edit policy decision.

### Bounded and version-fenced

Every candidate is bound to explicit document revisions and permitted context
dependencies. The editor, not the model or gateway, owns a conditional
application attempt after the required user gesture. This constrains known
failure modes; it is not a claim of semantic safety.

### Local-first is a data contract

It means local processing and no source-bearing persistence or export without
explicit policy. It does not merely mean that a daemon happens to run on the
same machine.

### Portable public core

Product code, schemas, portable examples, and sanitized evidence may be
public. Host topology, active routes, private source traces, raw evidence, and
operator state stay outside the public repository.

## Initial product shape

### Anvil Edit Core

The hot-path runtime:

- editor adapter boundary;
- opportunity gate;
- context compiler;
- prediction protocol registry;
- explicit capability policy;
- request deadlines and cancellation;
- candidate normalization and validation;
- abstention and presentation decision; and
- local outcome capture.

### Anvil Edit Lab

The evidence path:

- trace capture with consent and provenance;
- deterministic replay manifests;
- native-protocol model comparison;
- latency and resource accounting;
- future-edit similarity and semantic checks;
- shadow and visible-dogfood joins; and
- promotion reports with immutable inputs.

Lab is the primary technical differentiation hypothesis: it can establish
private, reproducible evidence that a complete system bundle works for a
particular developer or team. That does not establish that Lab should be the
first commercial entry point; Phase 0 separately tests whether users will
adopt Flow, Lab, or both and what setup and trace-consent burden they accept.

### First editor experience

One deeply instrumented adapter should prove a full lifecycle. Before calling
the contract portable, the project records a second-editor capability matrix
covering document identity/versioning, position encoding, cancellation,
presentation, conditional application, and outcome attribution. A provider
compatibility path proves only executor compatibility unless it exposes those
same lifecycle semantics.

Failure to support a second editor narrows the initial distribution claim to
one supported editor. It does not by itself invalidate Core or Lab.

## Deferred product hypotheses

| Hypothesis | Why it is deferred |
| --- | --- |
| Personal policy or weight tuning | Requires sufficient consented traces, time-split holdout, and rollback |
| Multi-file change propagation | Requires stronger location, validation, and review contracts |
| Live steering of active agents | Requires explicit scope, user review, and run correlation |
| Team/fleet product | Requires governance and aggregation that cannot become productivity scoring |
| Custom next-edit model | Premature until repeated, measured failure modes justify training |
| Full editor or editor fork | Distribution and UX cost would obscure the Core/Lab validation question |

## Success definition

The first product decision is positive only if evidence shows that an
open/local configuration can:

- stay within a calibrated interactive latency envelope;
- apply zero candidates across a buffer-version mismatch;
- keep stale work bounded through cancellation;
- provide meaningful surviving edits per active developer hour;
- avoid unacceptable interruption and immediate undo rates;
- preserve the trace and export privacy contract; and
- reproduce a comparison from pinned inputs.

Phase 0 also requires evidence of a real initial user or buyer, their current
workaround, tolerated setup cost, privacy threshold, willingness to opt into
trace capture, and whether the immediate pull is the editing experience or the
evaluation system. Technical feasibility without a credible adoption path is
an inconclusive result, not automatic continuation.

Initial numeric latency goals are hypotheses in the roadmap, not universal
truth. Baseline dogfood must calibrate the utility and interruption thresholds.

## Non-goals

The v0 does not:

- execute terminal commands;
- autonomously apply cross-file edits;
- own canonical project plans or task acceptance;
- own model process lifecycle or GPU placement;
- use a generic fleet event bus for high-rate editor activity;
- upload raw source or editing history by default;
- evaluate developer productivity; or
- promise that one named model is the permanent default.
