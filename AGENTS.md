# Repository guidance

Anvil Edit is in the foundation stage. Read these documents before changing
product behavior or introducing implementation:

1. `docs/PROJECT.md`
2. `docs/CONTRACTS.md`
3. `docs/ARCHITECTURE.md`
4. `docs/EVALUATION.md`
5. `docs/PRIVACY-AND-TRUST.md`
6. `docs/DECISIONS.md`
7. `docs/ROADMAP.md`

## Living product guidance

- After an authorized change to product behavior, contracts, architecture,
  evaluation, privacy, roadmap, PRDs, or implementation surfaces, use the
  repository plugin's
  `$anvil-edit-development:refresh-product-guidance` workflow before handoff. When
  the plugin is not installed, read and follow
  `plugins/anvil-edit-development/skills/refresh-product-guidance/SKILL.md`
  directly. Do not install or publish the plugin merely to satisfy this gate.
- Map the change to its canonical document first, then update affected PRDs,
  development skills, agent metadata, and the public README in the same PR when
  their guidance changed.
- Implementation authority does not authorize moving a canonical product
  boundary, invariant, metric, promotion gate, or accepted decision. Stop and
  report a conflict unless the current task explicitly authorizes that product
  decision. Explicit task file and scope limits override this refresh workflow;
  report out-of-scope drift without editing it.
- Keep the README short and understandable without requiring editor, model-
  serving, or evaluation expertise. Put normative detail in `docs/`.
- On read-only work, report documentation or skill drift without editing it.
- Do not claim an unattended updater exists. Guidance refreshes are explicit,
  reviewable repository changes backed by current evidence.

## Project development skills

When installed, the `anvil-edit-development` plugin is user-scoped. Every
bundled skill sets `policy.allow_implicit_invocation: false`, so its
instructions are loaded only through an explicit, plugin-qualified
invocation. This repository guidance supplies the project-specific invocation
rules: use these workflows while working here when their scope matches.

- `$anvil-edit-development:audit-editor-adapter` for editor selection and
  compatibility reviews;
- `$anvil-edit-development:design-edit-experiment` for replay, shadow, dogfood,
  or promotion studies;
- `$anvil-edit-development:intake-edit-model` for candidate artifact and
  benchmark admission;
- `$anvil-edit-development:review-prediction-contracts` for lifecycle and
  integration contracts;
- `$anvil-edit-development:review-trace-privacy` for trace, destination,
  retention, or export changes; and
- `$anvil-edit-development:refresh-product-guidance` after authorized product
  or implementation changes.

Do not make these skills implicitly invocable merely for convenience. New
skills in this plugin must use the same explicit-only policy. Outside this
repository, invoke one only when the user explicitly names it.

## Evidence discipline

- Keep discovery, source claims, local replay, live shadowing, visible dogfood,
  deployment, and promotion distinct.
- A vendor benchmark, model card, compatible API, healthy endpoint, or merged
  configuration is not local qualification or deployment proof.
- Pin model, tokenizer, prompt protocol, runtime, policy, hardware, and trace
  identity for every reproducible result.
- Report latency distributions and denominators. Do not promote a model from a
  single throughput number or an unqualified p50.
- Offline similarity is a candidate-quality signal, not proof that a developer
  would accept or retain a displayed suggestion.

## Product boundaries

- Anvil Edit chooses prediction policy and an explicit capability.
- Anvil Serving executes the capability and records serving evidence; it must
  not silently classify, substitute, fall back, or escalate for Edit.
- Anvil State may provide bounded, read-only task context. It is not an editor
  telemetry store.
- Anvil Events may converge policy, prompt, model-pack, and retention
  revisions. It is not the per-keystroke data plane.
- Workbench may display redacted experiment and promotion evidence. It is not
  in the prediction hot path.

## Public repository hygiene

- Keep host identities, private addresses, active route assignments, operator
  overlays, raw traces, credentials, logs, caches, and capability-bearing URLs
  out of this repository.
- Examples must be portable and sanitized. Use generic host and alias names.
- Raw source-bearing traces and exported benchmark corpora require explicit
  provenance and repository-owner permission.

## Contract changes

- Preserve buffer-version fencing: a stale candidate must never be applied.
- Resolve an `ExecutionGrant` before serializing source-bearing content for
  another process or trust domain.
- Preserve portable `DocumentRevision` semantics, causal/idempotency metadata,
  and separate dispatch, presentation, application, and survival evidence.
- Preserve explicit model and protocol identity for replay.
- Preserve opt-in source-bearing trace persistence and export.
- Treat append-only evidence as retention-bounded; authorized deletion must
  erase content and linkable derived state rather than only append a tombstone.
- Version wire and persisted schemas. Add migrations before changing durable
  data.
- Update the canonical documents and decision log when a change moves a
  product boundary, invariant, metric, or promotion gate.

## Evaluation discipline

- Treat model-native comparisons as system bundles, not model-only results.
- Keep exploratory reveal pilots separate from confirmatory randomized visible
  comparisons.
- Shadow evidence may nominate a visible experiment but cannot prove human
  utility or non-interruption.
- Report renderable coverage, failures, cancellation, censoring, and an
  inconclusive outcome alongside latency and utility metrics.
