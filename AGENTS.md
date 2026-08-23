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
