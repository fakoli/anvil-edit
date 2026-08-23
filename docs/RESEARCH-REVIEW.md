# Research review

Reviewed: **2026-08-23**

Scope: the supplied report, current public Anvil boundaries, and selected
primary competitive sources.

## Verdict

The research supports proceeding to a foundation and validation phase. Its
strongest conclusion is not that one open model is ready to win. It is that
predictive editing needs a model-independent control loop and an outcome-aware
evaluation system:

```text
observe -> select context -> choose explicit capability -> predict
        -> validate -> abstain/show -> observe outcome
```

That loop fits the current Anvil family boundaries. Anvil Edit fills a real
gap without turning Anvil State into telemetry, Anvil Serving into a semantic
router, Anvil Events into a keystroke bus, or Workbench into an editor hot path.

## Findings strong enough to encode

### The product should be Core plus Lab, not a model clone

Editors and named models will change. Snapshot, request, candidate, decision,
and outcome evidence can remain stable. Lab is not secondary analytics; it is
how the product decides which context/model/policy deserves the UI.

Lab is the strongest technical differentiation hypothesis, but the research
does not prove it is the first commercial entry point. Phase 0 must test actual
Flow-versus-Lab demand and tolerated setup/trace-consent burden.

### Abstention and presentation are first-class

Current primary sources support the report's emphasis on suggestion policy.
Cursor reports a Tab model making fewer suggestions with a higher acceptance
rate, while Zed reports production attention to response latency and offers
provider/model evolution around edit prediction. These are vendor-reported
results, but they strongly justify measuring quiet behavior rather than
maximizing suggestion volume.

### Specialized next-edit models deserve the first baseline

Zeta 2.1 and Sweep Next-Edit have public artifacts and purpose-built protocols.
They are credible candidates, not locally qualified defaults. A general coding
model belongs in a control or semantic shadow lane until short-context latency
and incremental utility are measured.

### Existing editor distribution may be available

JetBrains currently documents an OpenAI-compatible AI Completion provider with
edit-prediction prompt schemas including Zeta and Sweep. This supports a
executor/provider-compatibility spike. It does not prove that the surface
exposes the snapshot, cancellation, timing, and outcome evidence required by
Anvil Edit Lab.

### Cross-editor portability remains unproven

A shared inference protocol does not establish portable document revision,
position encoding, cancellation, presentation, conditional application, or
outcome attribution. One full adapter plus a second-editor capability matrix is
the minimum evidence for a portability claim. Failure narrows the supported
editor set rather than automatically invalidating Core or Lab.

### Optional task intent is a plausible Anvil advantage

Anvil State has exact intent, acceptance criteria, and non-goals. Bounded,
read-only task context may improve ambiguous semantic predictions. This remains
an A/B hypothesis; extra intent tokens can also add latency or distract the
model.

## Corrections and tighter framing

### Replay is not ground truth

The observed next edit is a valuable label but not the only valid future.
Offline exact match and judge scores cannot measure UI interruption or how a
displayed suggestion would have changed behavior. The foundation therefore
requires shadow and visible-dogfood evidence before promotion.

### TTUS needs a companion operational metric

“Time to useful suggestion” is known only after usefulness is observed. The
operational SLO should be time to renderable suggestion (TTRS), while TTUS is a
post-hoc conditional distribution reported with useful-suggestion coverage.

### Runtime data and durable traces are different

The model may need source in memory. That does not authorize storing it. The
foundation separates snapshots/context used at runtime, metadata evidence,
source-bearing replay content, export, and training permission.

### The initial product should be narrower than seven names

Core, Lab, and one editor adapter are enough to answer the first question.
Flow is the visible dogfood experience, not necessarily a separate package.
Personal, Ripple, Steer, and Fleet remain decision-gated hypotheses.

### “Learns” is aspirational in v0

The first loop records outcomes and compares versioned policies. It should not
promise online learning, personalization, or weight updates before time-split
holdout, shadowing, rollback, and consent exist.

### Hardware plans are not public product architecture

The supplied report contains useful private hardware hypotheses. Public docs
should describe fast and semantic capability classes. Exact host assignment,
active routes, local benchmarks, and restore transactions belong in private
operator evidence and must be verified at execution time.

## Not proven by the supplied research

- That any candidate meets the proposed latency SLO on local hardware.
- That a general coding model improves next-edit utility over a specialized
  small model.
- That a large teacher model is a reliable semantic judge for human utility.
- That native protocols can be compared as “model-only” results.
- That thousands of traces will be available with adequate consent and
  attribution quality in three months.
- That commit survival is a stable ground-truth metric across repositories.
- That an existing editor provider API exposes enough evidence for Lab.
- That Core is cross-editor portable beyond the first deeply instrumented
  adapter.
- That Flow or Lab has a validated initial buyer/adoption path.
- That local-only processing alone satisfies the required privacy threat model.
- That configured example aliases exist, are deployed, or are healthy.
- That a custom model will become necessary after policy and retrieval tuning.

These are experiments or operational checks, not omissions to fill with
assumptions.

## Evidence handling recommendations

- Maintain a source ledger with URL, publisher, publication/access date, claim,
  evidence class, and any vendor/community qualification.
- Prefer official project docs, model cards, immutable artifact revisions, and
  local raw benchmark manifests.
- Keep general coding quality, next-edit offline quality, inference latency,
  live human outcome, and deployment state in separate tables.
- Never copy opaque citation markers from a research tool into normative docs.
- Archive a sanitized research snapshot if long-term reproducibility matters;
  live web pages and default branches drift.

## Primary sources checked

The following were checked on 2026-08-23. They support product direction, not
local qualification:

### Anvil family boundaries

- [Anvil](https://github.com/fakoli/anvil) — canonical tasks, claims, evidence,
  and acceptance.
- [Anvil Serving](https://github.com/fakoli/anvil-serving) — explicit capability
  aliases and no hidden classifier/fallback/substitution.
- [Anvil Events](https://github.com/fakoli/anvil-events) — immutable desired
  revision, per-node reconciliation, and verified outcomes.
- [Anvil Workbench](https://github.com/fakoli/anvil-workbench) — private human
  supervision, redacted evidence, approvals, and delivery orchestration.

### Predictive-editing direction

- [Zeta 2.1](https://zed.dev/blog/zeta2-1) — Zed-reported Multi-Region format,
  output length, acceptance, rejection, and production response latency.
- [How Zed developed Zeta 2](https://zed.dev/blog/how-we-developed-zeta2) —
  richer edit history and LSP-resolved definition context.
- [Cursor Tab online RL](https://cursor.com/blog/tab-rl) — Cursor-reported
  improvement in acceptance while producing fewer suggestions.
- [Sweep Next-Edit 1.5B](https://huggingface.co/sweepai/sweep-next-edit-1.5B)
  — public model artifact, model card, protocol example, and license metadata.
- [JetBrains next edit suggestions](https://www.jetbrains.com/help/ai-assistant/next-edit-suggestions.html)
  and [custom models](https://www.jetbrains.com/help/ai-assistant/use-custom-models.html)
  — current OpenAI-compatible edit-prediction provider and prompt-schema path.

Other models and benchmarks in the supplied report remain candidate discovery
until exact artifacts, licenses, protocols, runtime compatibility, and local
measurements are pinned in an Anvil Edit benchmark manifest.
