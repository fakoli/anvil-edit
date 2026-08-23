# Anvil Edit

> A local-first control and evaluation layer for predictive software editing.

Anvil Edit is intended to turn live developer activity into bounded prediction
workloads, normalize model output into validated version-fenced candidates,
decide when a suggestion is worth showing, and measure what happens afterward.

The product is not an editor fork and is not a coding model. Its proposed
narrow waist is:

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

That contract is meant to let editors, context policies, prediction models,
and inference runtimes change independently while preserving replayable
evidence.

## Status

**Foundation / pre-implementation.** This repository currently defines the
product, boundaries, proposed contracts, evidence standard, trust model, and
validation roadmap. It does not yet contain a released runtime, a deployed
model route, a qualified model, or a locally recorded performance result.

All latency numbers, model choices, and launch dates in the roadmap are test
hypotheses until Anvil Edit records the corresponding evidence.

## Product thesis

Predictive editing is a systems problem as much as a model problem. A useful
product must:

- detect a real opportunity without reacting to every keystroke;
- compile the smallest useful and permitted context;
- choose an explicit prediction capability under a deadline;
- cancel work as soon as its source buffer becomes stale;
- normalize and validate the proposed edit before it reaches the editor;
- abstain when expected utility is low; and
- connect latency and compute cost to human outcomes over time.

The initial product is two technical surfaces and one bounded dogfood
experience over one contract:

- **Anvil Edit Core** — the editor-facing predictive runtime.
- **Anvil Edit Lab** — capture, replay, comparison, and promotion evidence.
- **Anvil Flow v0** — the first instrumented supported-editor experience; it
  is a validation surface, not yet a separately proven business.

A single instrumented editor adapter provides the first dogfood experience.
Personalization, multi-file propagation, agent steering, and fleet governance
remain later hypotheses.

## Boundaries

Anvil Edit owns editor state, opportunity detection, context selection,
prediction policy, cancellation, abstention, presentation decisions, and
outcome capture.

It does not take ownership from sibling Anvil products:

| System | Remains authoritative for |
| --- | --- |
| [Anvil](https://github.com/fakoli/anvil) | Project intent, tasks, claims, evidence, and acceptance |
| [Anvil Serving](https://github.com/fakoli/anvil-serving) | Model lifecycle, capacity, explicit capability aliases, and inference execution |
| [Anvil Events](https://github.com/fakoli/anvil-events) | Desired configuration revision and fleet convergence |
| [Anvil Workbench](https://github.com/fakoli/anvil-workbench) | Human supervision, approvals, delivery evidence, and optional experiment views |

Anvil Edit must also run in standalone mode. Integrations with sibling products
are optional adapters, not hidden runtime dependencies.

## Foundational documents

- [Project](docs/PROJECT.md) — users, promise, scope, principles, and product
  hypotheses.
- [Contracts](docs/CONTRACTS.md) — domain objects, lifecycle, and invariants.
- [Architecture](docs/ARCHITECTURE.md) — components, data paths, cancellation,
  and integrations.
- [Evaluation](docs/EVALUATION.md) — evidence classes, benchmark design,
  metrics, and promotion gates.
- [Privacy and trust](docs/PRIVACY-AND-TRUST.md) — source-bearing trace policy,
  consent, export, retention, and UX trust.
- [Roadmap](docs/ROADMAP.md) — a staged validation plan with kill criteria.
- [Decisions](docs/DECISIONS.md) — accepted foundation decisions and open
  questions.
- [Research review](docs/RESEARCH-REVIEW.md) — what the supplied research
  supports, what it does not prove, and primary references.
- [Product PRDs](docs/prds/README.md) — implementation contracts for Core v0,
  Lab v0, and the first Flow adapter.
- [Development plugin](plugins/anvil-edit-development/.codex-plugin/plugin.json)
  — repository-local review skills for the recurring contract, privacy,
  adapter, evaluation, and model-intake work.

## Non-goals for the first release

- Training a proprietary next-edit model.
- Building or forking a full editor.
- Sending every keystroke to a model or general-purpose event bus.
- Hiding model selection, fallback, or cloud escalation behind an alias.
- Applying cross-buffer or terminal actions without explicit review.
- Ranking individual developers from behavioral telemetry.
- Claiming model quality from vendor benchmarks or offline similarity alone.

## Contributor development plugin

The repository-local `anvil-edit-development` plugin contains documentation-
only review workflows and no hooks, MCP servers, credentials, or deployment
authority:

- [`audit-editor-adapter`](plugins/anvil-edit-development/skills/audit-editor-adapter/SKILL.md)
  — distinguish a full lifecycle adapter from executor compatibility.
- [`review-prediction-contracts`](plugins/anvil-edit-development/skills/review-prediction-contracts/SKILL.md)
  — attack lifecycle, fencing, authorization, and evidence seams.
- [`review-trace-privacy`](plugins/anvil-edit-development/skills/review-trace-privacy/SKILL.md)
  — audit grants, threat boundaries, retention, erasure, and team aggregation.
- [`design-edit-experiment`](plugins/anvil-edit-development/skills/design-edit-experiment/SKILL.md)
  — stage replay, shadow, exploratory, confirmatory, and promotion evidence.
- [`intake-edit-model`](plugins/anvil-edit-development/skills/intake-edit-model/SKILL.md)
  — admit immutable model artifacts to discovery or local benchmarking without
  confusing either with qualification.

No marketplace entry is created by this repository. Installation/publication
is a separate operator decision.

## Working promise

> Bring a supported editor and an explicit model capability. If you separately
> opt into trace capture, Anvil Edit measures whether the resulting policy
> produces useful edits under a latency and privacy contract.

The shorter end-user line remains a hypothesis:

> Your models. Your code. Your editing history. The next useful edit before
> you ask.
