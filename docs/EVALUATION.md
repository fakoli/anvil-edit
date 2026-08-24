# Evaluation and promotion

Status: **normative foundation; thresholds are initial hypotheses**

Last reviewed: **2026-08-23**

Anvil Edit exists to connect model and policy behavior to developer outcomes.
No single offline metric, endpoint health check, throughput number, or vendor
claim is sufficient for promotion.

## Evidence ladder

| Class | Evidence | What it supports |
| --- | --- | --- |
| E0 | Vendor, paper, model-card, or community claim | Discovery and candidate selection only |
| E1 | Contract fixtures and synthetic unit tests | Parser, fencing, cancellation, and metric correctness |
| E2 | Local fixed-corpus inference benchmark | Runtime compatibility, latency, resource use, reproducibility |
| E3 | Replay on permitted real traces | Offline candidate concordance and fixed-opportunity policy simulation; not counterfactual human utility |
| E4 | Live shadow predictions | Latency, disagreement, and future-action comparison without UI influence |
| E5 | Visible dogfood | Acceptance, interruption, partial use, undo, and attribution evidence |
| E6 | Durable outcome observation | Survival through time, save, or commit proxies |
| E7 | Approved deployment and post-promotion observation | Current operational use within the declared scope |

Each report names its highest evidence class. E3 does not become E5 because a
judge model says the candidate was good. E7 is scoped to an exact policy,
model/runtime identity, adapter, population, and time window.

## Evaluation funnel

```text
O  editor opportunities
E  eligible opportunities
R  inference requests
C  valid candidates
S  suggestions shown
A  accepted or partially accepted suggestions
V  accepted content surviving a defined checkpoint
K  accepted content correlated with save or commit
```

Always report denominators. Important ratios include:

| Metric | Definition |
| --- | --- |
| Eligibility rate | `E / O` |
| Request rate | `R / O` and `R / E` |
| Candidate yield | `C / R` |
| Display rate | `S / E` |
| Acceptance rate | accepted decisions / `S` |
| Partial acceptance rate | partially accepted decisions / `S` |
| Checkpoint survival | surviving accepted content / accepted content |
| Stale-request rate | requests invalidated before decision / `R` |
| Late-result rate | completed after render deadline / `R` |
| Immediate undo rate | attributable undo within the declared window / accepted decisions |

A high acceptance rate can be obtained by showing only punctuation. A high
display rate can be obtained by interrupting constantly. Both require volume,
survival, and interruption context.

## Latency metrics

### Server timing

- **Queue time** — executor receipt to execution start.
- **TTFT** — executor dispatch or receipt to first generated token; the chosen
  start point must be explicit.
- **Decode time and rate** — first token to generation termination.

### End-to-end timing

- **TTRS** — opportunity timestamp to first renderable suggestion, regardless
  of later outcome.
- **Time to decision** — opportunity timestamp to `show`, `suppress`, or
  terminal no-candidate decision.
- **TTUS** — TTRS restricted post hoc to suggestions meeting a declared useful
  outcome, reported together with useful-suggestion coverage.

TTUS alone is not a live service SLO because usefulness is learned later. TTRS
is the operational latency metric; TTUS connects that latency to outcome.
Every TTRS distribution is paired with renderable coverage plus expired,
failed, cancelled, stale, and no-candidate rates; a system cannot improve
latency by dropping slow opportunities without exposing the loss.

Report p50, p90, p95, and p99 when sample size supports them. Always retain raw
durations or a lossless permitted artifact so distributions can be recomputed.
The foundation default requires at least 2,000 observations in a reported
stratum before p99 is used as a gate, yielding at least 20 expected tail
observations. A manifest may predeclare a stricter rule. Smaller p99 estimates
are labeled descriptive and insufficient for promotion.

## Utility and trust metrics

Core outcome metrics:

- accepted characters or normalized edit tokens per active developer hour;
- semantic accepted edits per active hour under a documented classifier;
- five-minute, save, and commit-correlated survival;
- post-accept edit distance;
- immediate undo and rapid rewrite rate;
- correct next file/region/location rate;
- syntax and diagnostic delta;
- ignored prediction visible duration;
- explicit-reveal use and subtle-mode preference; and
- reported interruption or disabled-feature rate.

Core efficiency metrics:

- GPU milliseconds per surviving accepted edit;
- context tokens per surviving accepted edit;
- cache and prefix reuse;
- speculative-token acceptance where applicable;
- requests cancelled before execution and during generation; and
- metadata/content bytes retained per active hour.

The north-star family is **surviving useful edit value per active developer
hour**, paired with latency, interruption, privacy, and compute guardrails. The
exact value weighting must be declared and sensitivity-tested; it is not a
universal scalar.

## Offline replay limits

The developer's next observed edit is useful evidence but not complete ground
truth:

- several different edits may be semantically valid;
- a prediction could have changed what the developer did;
- a matching suggestion might still be visually annoying;
- timing can make a correct candidate useless;
- commit history loses intermediate intent, undo, and cursor flow; and
- a judge model can add a signal but cannot simulate human acceptance.

Replay reports therefore separate:

1. exact and normalized text match;
2. edit/delta similarity;
3. location hit;
4. syntax or language-service checks;
5. semantic equivalence signals with judge identity;
6. latency and deadline success; and
7. whether comparable visible dogfood exists.

LLM-judge scores are secondary. Candidate order and system identity are hidden
and randomized, the judge artifact is pinned, and a declared human sample
calibrates agreement and systematic bias before the score influences
selection.

Offline results select candidates for shadowing. They do not directly promote
a visible policy.

## Benchmark manifest

Every reproducible run pins:

```text
benchmark schema and runner revision
trace/corpus identity, digest, provenance, and permission class
sample selection and exclusions
adapter/editor identity, document-revision contract, and position encoding
active ConfigurationSnapshot identity, revision, digest, and provider
externally desired configuration event/resource/generation/revision, if any,
  kept separate from the active and used snapshot
model repository and immutable revision
tokenizer and immutable revision
quantization or conversion artifact digest
runtime image/revision and material flags
prompt protocol and immutable revision
context policy and immutable revision
prediction/display policy and immutable revision
hardware class, driver, and runtime versions
cache state, concurrency, output budget, and generation settings
start/end timestamps and raw result artifact digests
assignment unit, randomization seed, cohort/cluster fields, and censoring rules
```

Aliases and product names are annotations, not immutable identity.

Native-protocol comparisons are reported as **system bundles**: model,
tokenizer, prompt protocol, context adapter, runtime, quantization, and
normalization/validation policy. A report may isolate one factor only when the
other bundle components are held fixed.

## Initial benchmark matrix

The first local campaign should cover:

| Dimension | Initial values |
| --- | --- |
| Context | 1K, 2K, 4K, 8K, and 16K realistic tokens |
| Output cap | 16, 32, 64, 96, and 192 tokens where protocol permits |
| Cache state | cold, warm identical prefix, warm mostly-identical prefix |
| Concurrency | 1; later 2 and 4 for shadow workloads |
| Editor behavior | stationary pause, rapid cancellation, file switch, repeated edit |
| Model class | specialized next-edit fast candidates plus FIM/general controls |
| Measurements | trigger, context, queue, TTFT, decode, validation, render, total |

The corpus should include contract fixtures, permissively sourced public
controls, and private consented traces reported separately. Prompt-native
protocols remain distinct and are always named in comparisons.

## Candidate and policy experiments

Use the smallest sequence that isolates causal value:

1. **E1 contracts/privacy/concurrency:** prove parsing, fencing,
   authorization, deletion, metric, and event-order fixtures.
2. **E2 local compatibility/latency:** run pinned system bundles on a fixed
   public or synthetic corpus without a visible product claim.
3. **E3 locked replay:** compare candidate concordance and fixed-opportunity
   simulations on a frozen permitted trace split.
4. **E4 live shadow:** measure real deadline behavior and disagreements on one
   common opportunity stream without UI influence.
5. **E5 exploratory reveal pilot:** expose candidates only through an explicit
   reveal gesture to discover UX failure modes and calibrate instrumentation.
6. Freeze primary metrics, thresholds, exclusions, assignment unit, analysis,
   and stopping rules; hold out a later time period or population.
7. **E5 randomized visible comparison:** assign from the common opportunity
   stream before policy-specific gating and analyze intent-to-treat first.
8. **E6 durable outcomes:** observe fixed checkpoints with right-censoring;
   treat save and commit correlation as progressively weaker/exploratory when
   attribution is lost.
9. **E7 promotion:** approve and observe the exact selected revision and scope.

Shadow evidence nominates a policy for a visible experiment. It cannot prove
that the UI is useful or non-interruptive. Exploratory dogfood and confirmatory
visible evaluation are temporally separated so thresholds are not calibrated
and declared passed on the same observations.

Assignment and analysis account for repeated observations. The manifest names
the assignment unit and cluster fields—developer, repository, session, or a
declared N-of-1 crossover—and records them from the first schema rather than
attempting to reconstruct them after exposure. Per-protocol missingness,
deadlines, cancellation, and abstention remain in the intent-to-treat result.
An experiment may conclude **inconclusive**; lack of statistical or operational
resolution is not a pass.

Do not begin with an ensemble. If the gated policy does not materially improve
durable utility or reduce interruption at acceptable latency and compute cost,
remove it.

## Initial SLO hypotheses

These values guide measurement; they are not measured claims:

- visible fast-path TTRS p50 at or below 200 ms;
- visible fast-path TTRS p90 at or below 500 ms;
- zero application across document identity/version/digest mismatch;
- all visible requests carry a deadline and cancellation token;
- raw source export disabled by default;
- no unrecorded model or protocol substitution; and
- no semantic lane in the visible path until it beats fast-only in dogfood on
  a declared durable-utility metric without unacceptable interruption.

After the first dogfood window, the project records calibrated thresholds for
display rate, stale/late results, undo, survival, and useful-edit volume.

## Promotion gate

A policy/model configuration may become a visible default only when:

1. its immutable manifest is complete;
2. contract, stale-edit, cancellation, parser, and privacy tests pass;
3. local latency/resource evidence meets its declared SLO;
4. permitted replay shows no material regression on protected cohorts;
5. live shadow confirms deadline behavior and exposes disagreements;
6. an exploratory reveal pilot finds no unresolved interaction or attribution
   failure that invalidates the planned comparison;
7. a later randomized visible comparison demonstrates utility and interruption
   within predeclared gates under intent-to-treat analysis;
8. durable outcomes are reported with censoring and attribution loss rather
   than silently dropping missing checkpoints;
9. rollback configuration and data migration behavior are tested; and
10. a human approves the exact revision and scope.

A source merge, configured route, health response, or successful benchmark does
not perform deployment or promotion. Post-promotion observation is required to
retain the default.

## Reporting rules

- Label E0-E7 evidence explicitly.
- Separate public portable evidence from private traces and operator evidence.
- Publish sample counts, exclusions, time window, confidence intervals or
  uncertainty appropriate to the data, and all relevant denominators.
- Report failures, cancellations, late results, and missing observations.
- Name the full system bundle and state every changed component.
- Separate exploratory from confirmatory results and permit an inconclusive
  decision.
- Distinguish configuration desired, received, staged, active, used by a
  request, executor-deployed, and policy-promoted states.
- Report assignment, clusters, censoring, and attribution loss.
- Label LLM-judge results secondary and publish the blinded calibration sample.
- Preserve immutable raw result artifacts at their permitted disclosure level.
- Never turn team aggregates into individual productivity rankings.
