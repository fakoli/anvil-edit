---
name: review-trace-privacy
description: Audit Anvil Edit trace collection, executor destinations, retention, deletion, export, team aggregation, and threat boundaries. Use before source persistence, remote inference, replay/export, training, or team telemetry changes.
---

# Review Trace Privacy

## Workflow

1. Read `docs/PRIVACY-AND-TRUST.md`, `docs/CONTRACTS.md`, and
   `docs/ARCHITECTURE.md`.
2. Draw the data path across UI, adapter, Core, stores, Lab, executor hosts,
   users/operators, and destination trust domains.
3. Classify every field by content sensitivity and joinability. Treat hashes,
   stable identifiers, exact timestamps, cadence, paths, and derived indexes as
   potentially identifying.
4. Resolve independent grants for runtime read, dispatch, persistence, replay,
   export, training, shadow, task context, and outcome correlation.
5. Test policy precedence: deny union, allowlist intersection, minimum
   retention, local pause wins, fleet only narrows, unknown fails closed.
6. Trace deletion through source blobs, metadata, indexes, WAL/free pages,
   backups, exports, caches, and failure reporting.
7. Return blockers, bounded remediations, residual risk, and the evidence class.

## Required threat checks

Cover IPC peer authentication, transport identity/encryption, key creation and
rotation, multi-user hosts, stolen disks, malware limits, crash dumps, swap,
temporary files, logs, telemetry, backups, and sync. Evaluate protected paths
after URI/path resolution and treat prompts/model output as hostile.

Team outputs default to local aggregation with no stable individual/session/
repository IDs, exact timestamps, cadence, paths, digests, or free text. Require
minimum cohort/dimensionality thresholds and suppress sparse cells.

Block source-bearing persistence or remote inference until the relevant grant,
threat, and erasure tests exist. Do not make legal or compliance claims from a
technical checklist.

## Keep this skill current

After an authorized change alters data classes, grants, trust boundaries,
retention, erasure, export, or aggregation rules, use
`$refresh-product-guidance` before handoff. Update this skill in the same change
when its trigger or checklist is stale. On read-only review, report drift
without editing repository files.
