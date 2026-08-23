---
name: audit-editor-adapter
description: Audit an editor or extension API against Anvil Edit's complete adapter contract. Use for first-editor selection, second-editor portability matrices, provider compatibility spikes, or reviews of snapshot, cancellation, presentation, conditional application, and outcome support.
---

# Audit Editor Adapter

## Workflow

1. Read `docs/CONTRACTS.md`, `docs/ARCHITECTURE.md`,
   `docs/PRIVACY-AND-TRUST.md`, and `docs/EVALUATION.md` from the target Anvil
   Edit checkout.
2. Inspect primary editor documentation and, when available, the adapter code
   or a minimal probe. Label documentation claims separately from executed
   evidence.
3. Build a capability matrix with `supported`, `partial`, `missing`, or
   `unknown`, an evidence pointer, the consequence, and the next proving test.
4. Classify the result as full lifecycle adapter compatibility or executor-only
   compatibility. Do not collapse the two.
5. Recommend `support`, `narrow`, `spike`, or `block` and state the exact claim
   the evidence permits.

## Required matrix

Cover:

- workspace instance, document incarnation, logical URI, editor version, full
  digest, line endings, terminal newline, and position encoding;
- event sequence, duplicate delivery, file switch/reopen/rename, and
  cancellation;
- inline, local diff, explicit reveal, next-location, and unsupported-mode
  reporting;
- single-document conditional compare-and-apply and multi-range ordering;
- presentation, user-gesture, application, undo/rewrite, and survival
  attribution;
- extension/Core IPC peer identity and destination authorization; and
- timing boundaries and relative deadline propagation.

Probe UTF-16 and non-BMP text where relevant. Never infer continuity across a
document reopen or synthesize outcome evidence the editor does not expose.

## Keep this skill current

After an authorized change alters the adapter contract or evidence standard,
use `$refresh-product-guidance` before handoff. Update this skill in the same
change when its trigger, matrix, gate, or evidence wording is stale. On a
read-only audit, report drift without editing repository files.

## Portability gate

One full adapter proves only that editor. Require a second-editor capability
matrix before claiming cross-editor portability. A provider endpoint proves
executor compatibility only. A second-editor failure narrows the supported
surface; it does not automatically reject Core or Lab.
