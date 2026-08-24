---
name: review-prediction-contracts
description: Adversarially review Anvil Edit lifecycle, wire, persistence, and integration contracts. Use when schemas, candidate application, routing, retries, evidence ownership, or product boundaries change before implementation or promotion.
---

# Review Prediction Contracts

## Review order

1. Read `docs/PROJECT.md`, `docs/CONTRACTS.md`, `docs/ARCHITECTURE.md`,
   `docs/DATA-MODEL.md`, `docs/PRIVACY-AND-TRUST.md`,
   `docs/EVALUATION.md`, and `docs/DECISIONS.md`.
2. Trace one opportunity from snapshot through dispatch, grant, request,
   candidate, decision, presentation, application, and survival.
3. Attack each boundary with stale state, duplicate/out-of-order delivery,
   retry/fallback, deadline expiry, missing evidence, hostile protocol output,
   partial application, deletion, and schema-upgrade cases.
4. Separate a missing contract from an implementation defect and from an
   unproven product hypothesis.
5. Return findings ordered by severity with evidence, affected invariant, and
   the smallest acceptance test that would close each gap.

## Load-bearing checks

- `DocumentRevision` includes incarnation, URI scheme, version namespace,
  encoding/canonicalization, range semantics, and full-buffer digest.
- Downstream `DocumentRevisionRef` values repeat those source-free fencing
  semantics; out-of-order delivery does not depend on a hidden database join.
- Every durable lifecycle role remains a distinct `LifecycleRecord` variant
  with the common causal envelope; a generic event payload does not erase
  dispatch, serving, presentation, application, outcome, or survival meaning.
- Durable semantic records carry purpose-scoped `ContentReference` handles,
  not source, URI/path, prompt, model-output, or replacement bytes; opaque IDs
  and reason codes are not a loophole for source-bearing strings.
- `ExecutionGrant` is resolved before serialization and names destination,
  purpose, mode, content classes, policy digest, and expiry.
- `RuntimeReadGrant` is a distinct pre-context record, while the later
  `ExecutionGrant` binds the exact selected content handles and cannot
  retroactively authorize source reads.
- Attempts carry causal parents, producer sequence/clock, idempotency, and
  explicit retry/race/fallback relations.
- Context dependencies distinguish application-critical, display-critical,
  and advisory freshness.
- Raw model output is bounded, parsed as untrusted input, and never rendered or
  executed directly.
- Serving and Edit evidence have one correlation seam, explicit ownership, and
  conflict behavior.
- Fleet configuration is source-free policy input, never an `ExecutionGrant`;
  it may narrow but never widen local permission and remains outside the hot
  path.
- A convergence adapter reports applied only after Core verifies the exact
  active resource, generation, revision, and digest; indeterminate activation
  is recovered by observation rather than silent replay.
- Immutable evidence remains subject to authorized physical erasure.
- Multi-document atomicity is never implied when an editor cannot provide it.
- Semantic, wire, IPC, and durable-store versions remain independent; Rust
  memory layout and enum discriminants never become an implicit polyglot ABI.

Do not approve semantic routing, cross-file application, source persistence,
remote dispatch, training, steering, or fleet scope by adjacency. When an
authorized change moves an invariant or boundary, update the canonical document
and append a decision record.

## Keep this skill current

After an authorized contract or boundary change, use
`$anvil-edit-development:refresh-product-guidance` before handoff. Update this
skill in the same change when its trigger, review order, or load-bearing checks
are stale. On read-only review, report drift without editing repository files.
