---
name: refresh-product-guidance
description: Keep Anvil Edit's README, canonical product documents, PRDs, development skills, and agent metadata aligned as the product evolves. Use after authorized changes to product behavior, contracts, architecture, evaluation metrics or gates, privacy policy, roadmap, PRDs, or implementation surfaces; when review findings suggest documentation drift; and before major PR or release handoff.
---

# Refresh Product Guidance

## Establish the change set

1. Run `git status --short --branch` and preserve unrelated user changes.
2. Record the authorized change summary, base revision or explicit diff/path
   scope, authority limits, pre-existing dirty files, and expected canonical
   owner when known.
3. Enumerate all changed paths before deciding what is affected. Stop on an
   overlapping or ambiguous dirty edit that cannot be separated safely.

Implementation authority does not authorize changing a canonical invariant,
boundary, metric, promotion gate, accepted decision, or open decision. When
implementation conflicts with canonical guidance, stop and report the conflict
unless the current task explicitly authorizes that product decision. Explicit
file and scope limits in the current task win; report other drift without
editing it.

## Canonical ownership

| Source | Owns |
| --- | --- |
| `docs/PROJECT.md` | Product promise, users, scope, principles, and non-goals |
| `docs/CONTRACTS.md` | Semantic objects, lifecycle, and invariants |
| `docs/ARCHITECTURE.md` | Components, runtime shape, data paths, and integrations |
| `docs/EVALUATION.md` | Evidence meanings, metrics, experiments, and promotion gates |
| `docs/PRIVACY-AND-TRUST.md` | Data permissions, trust, retention, deletion, and user control |
| `docs/DECISIONS.md` | Decision rationale and history; never an independent override of current normative text |
| `docs/ROADMAP.md` | Sequence, milestones, dependencies, and exit or kill criteria |
| `docs/prds/` | Downstream implementation requirements |
| Development skills | Downstream procedures and checklists |
| Skill agent metadata and plugin metadata | Discovery and user-interface descriptions |
| `README.md` | Non-normative public summary |
| `AGENTS.md` | Contributor governance |

Stop on an unresolved conflict between canonical sources. Do not select the
source that merely matches the implementation.

## Workflow

1. Read `AGENTS.md`, the seven canonical documents it names, and
   `docs/prds/README.md`. Read affected PRDs and development skills. Read
   `docs/RESEARCH-REVIEW.md` when research, market, or model facts changed.
2. Inspect the authorized change and its evidence. Separate implemented state,
   proposed behavior, local evidence, deployment, and promotion.
3. Build a small impact map: changed fact -> canonical owner -> affected PRD ->
   affected skill -> public README or agent metadata.
4. Update the canonical owner first. Append a decision record when a product
   boundary, invariant, metric, or promotion gate moves.
5. Update affected PRDs and skills in the same change. Remove stale or repeated
   guidance instead of accumulating exceptions.
6. Update `agents/openai.yaml` when a skill's trigger, user-facing purpose, or
   default prompt changes. Update plugin metadata when its scope changes.
7. Update the README only when the public promise, product shape, status, or
   contributor entry point changed. Keep it short and plain-language; link to
   normative detail.
8. Validate every touched skill and the plugin, then review the diff for
   unsupported claims, private operational detail, and contradictions.

One invocation satisfies the upkeep clauses for every derivative skill change
it makes. Never invoke this skill recursively. Recompute the impact map after
editing and finish when no newly affected file appears. Stop and report a cycle
or unresolved new owner.

Review findings trigger a drift audit only. They authorize writes only after
the relevant product owner accepts the finding and the current task authorizes
the repository change.

## Validation

Run:

```text
python -m unittest discover -s plugins/anvil-edit-development/tests -p "test_*.py"
python plugins/anvil-edit-development/scripts/validate_guidance.py
git diff --check
```

Also run an available skill-spec linter and plugin schema validator. If either
is unavailable, report the gap rather than claiming full validation. Compare
the final touched files with the impact map and scan for unsupported claims,
private operational details, and stale agent metadata.

When plugin packaging or distribution metadata changes, test discovery in a
fresh harness. If that test is unavailable or fails, label installation and
publication unproven.

## Guardrails

- Treat this as an explicit, reviewable refresh gate, not an unattended
  self-modifying process.
- On read-only work, report drift and the files that need attention; do not
  write without authorization.
- Do not rewrite accepted history. Supersede decisions and version durable
  contracts when required.
- Do not turn vendor claims, compatible endpoints, source merges, or shadow
  results into local qualification, deployment, or human utility claims.
- Keep private hosts, routes, raw traces, credentials, and operator evidence
  outside the public repository.
- Preserve concise skills. Store normative detail in canonical documents and
  point skills to it.
- A source manifest or version bump does not install, publish, or prove an
  active plugin. Never perform those actions without separate authorization.

## Handoff

Report:

- the canonical sources reviewed;
- the guidance files changed and why;
- canonical documents, PRDs, and skills checked but left unchanged;
- unresolved drift or decisions; and
- validation commands and results.
