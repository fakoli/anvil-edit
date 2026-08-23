---
name: design-edit-experiment
description: Design reproducible Anvil Edit replay, shadow, visible-dogfood, and promotion experiments. Use when comparing models, protocols, context or display policies, task context, routing, latency, utility, or durable outcomes.
---

# Design Edit Experiment

## Sequence

Use the smallest applicable evidence ladder:

1. E1 contract, privacy, concurrency, and metric fixtures.
2. E2 pinned local compatibility and latency.
3. E3 locked replay for candidate concordance and fixed-opportunity simulation.
4. E4 live shadow on one common opportunity stream.
5. E5 explicit-reveal exploratory pilot.
6. Freeze metrics, thresholds, exclusions, assignment, analysis, and stopping.
7. E5 randomized visible comparison with intent-to-treat primary analysis.
8. E6 censor-aware survival checkpoints.
9. E7 human-approved deployment and post-promotion observation.

Shadow may nominate a visible experiment; it cannot prove UI utility. Keep the
exploratory and confirmatory windows temporally separate. Permit an
`inconclusive` result.

## Manifest and analysis

Pin the complete system bundle: model, tokenizer, native protocol, context
adapter, runtime, quantization, normalization/validation policy, editor adapter,
hardware, corpus/trace, and policy revisions. Record the assignment unit,
randomization seed, developer/repository/session cluster fields, exclusions,
censoring, and raw artifact digests before exposure.

Assign from the common opportunity stream before policy-specific gating.
Report all funnel denominators, renderable coverage, deadlines, failures,
abstention, cancellation, stale work, latency distributions, interruption,
undo/rewrite, accepted volume, and fixed-checkpoint survival. Do not gate on p99
with fewer than 2,000 observations in the reported stratum unless a stricter
predeclared rule applies.

Treat LLM judges as secondary: pin identity, blind and randomize candidate
order, and calibrate on a declared human sample. Treat commit correlation as an
exploratory proxy when attribution is lost.
