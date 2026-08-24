# Anvil Events integration contract

Status: **design reference; not implemented**

Last reviewed: **2026-08-23**

This document defines the boundary for a future Anvil Events integration. It
does not claim that an Anvil Edit adapter, event resource, artifact, route,
deployment, or fleet promotion exists. The upstream implementation work is
tracked in [fakoli/anvil-events#13](https://github.com/fakoli/anvil-events/issues/13).

## Purpose

Anvil Events may converge approved, source-free Anvil Edit configuration to
managed developer nodes. It is an asynchronous control plane, never the editor
telemetry plane and never a synchronous dependency of prediction.

The initial integration has one job:

> Deliver one exact configuration bundle to a node, let the node validate and
> activate it under stricter local policy, and report whether that exact bundle
> became active.

## Product boundary

| Authority | Owns |
| --- | --- |
| Anvil Events | Desired-resource envelope, generation ordering, artifact delivery evidence, node reconciliation attempt, and reconciliation outcome |
| Anvil Edit Core | Bundle validation, effective-policy compilation, local activation, active revision, prediction behavior, and activation evidence |
| Local developer policy | Pause, repository permission, destination permission, capture, persistence, replay, export, training, shadow, task-context, and outcome-correlation ceilings |
| Anvil Serving | Resolution and execution of the explicit capability selected by Edit; no hidden routing or substitution |

An Events revision is configuration input. It is not an `ExecutionGrant`, model
qualification, deployment approval, or permission to read or serialize source.
Fleet policy may narrow local permission and budgets; it can never widen them.

## Deployment modes

### Standalone

Standalone is the foundation default. Core loads a local configuration through
its own versioned activation boundary and has no Anvil Events dependency.

### Managed

Managed mode is explicit and opt-in. A background reconciler receives desired
configuration, stages and validates it, asks Core to activate it through an
authenticated local boundary, and verifies the active identity. The prediction
hot path reads only the locally active immutable snapshot.

If managed mode has never activated a compatible configuration, prediction
remains disabled unless the user explicitly switches to standalone mode. If
Events becomes unavailable after activation, Core may continue the last
verified configuration until its declared validity or local policy expires.
The local pause always takes effect independently of Events availability.
Eventual convergence is not an emergency-revocation channel; bundle validity,
offline grace, and fail-closed expiry remain open under O015.

## Desired resource contract

The first integration uses one resource per release channel:

```text
edit/config/<channel>
```

Example channel names are `default`, `preview`, or an organization-defined safe
token. A channel is not a developer identity or repository identifier.
Exactly one locally configured authority owns a channel on a node at a time.
Changing that authority is a separate local rebind operation, not an event-side
generation update.

The initial adapter name is reserved as:

```text
anvil_edit_config
```

The desired event uses the Anvil Events v2 `state.desired` envelope:

| Field | Anvil Edit meaning |
| --- | --- |
| `resource` | `edit/config/<channel>` |
| `generation` | Positive, authority-assigned monotonic generation for that resource |
| `revision` | Immutable configuration-bundle revision |
| `content_sha256` | SHA-256 of the exact artifact bytes |
| `adapter` | `anvil_edit_config` |
| `artifact` | Logical artifact reference resolved only through node configuration |
| `targets` | Optional explicit node tokens; never people, repositories, or editor sessions |

The event contains no configuration body, source content, prompts containing
captured source, endpoint URL, credential, filesystem path, trace identifier,
or behavioral outcome.

## Configuration bundle contract

The artifact is one bounded, deterministic, source-free configuration bundle.
Its future wire schema must preserve these semantics:

- contract name and independently versioned contract major;
- bundle identifier and immutable bundle revision;
- the matching Events resource, generation, revision, and artifact digest;
- minimum and maximum compatible Core contract majors;
- immutable prediction-, context-, display-, routing-, and authorization-policy
  identities and digests;
- immutable prompt-protocol and normalization-policy identities and digests;
- explicit capability aliases with their expected protocol identities;
- deadline, output, context, and resource ceilings;
- optional retention ceilings that can only reduce locally permitted retention;
- component ordering and deterministic canonicalization rules; and
- no secret values, capability-bearing URLs, host paths, source, raw trace,
  individual identifiers, or mutable `latest` references.

The first bundle is the atomic activation unit for one managed resource on one
Core instance. It does not promise cross-process, cross-database, cross-node,
or fleet-wide atomicity. A request retains the immutable snapshot it started
with across a later activation. Separate Events resources for prompt packs,
policy packs, or model packs are deferred until the system has a tested
activation-group contract. Without that contract, independently converged
resources could produce a combination that was never reviewed.

A capability alias in the bundle is not resolved model identity and does not
prove that Anvil Serving has configured, deployed, or qualified the route.

## Activation lifecycle

The future adapter performs these steps outside the prediction hot path:

1. Validate the Events envelope, target, exact authority/resource/adapter
   binding, generation, revision, and digest.
2. Fetch the logical artifact through the node-configured authenticated source.
3. Verify exact revision and artifact SHA-256 before parsing.
4. Parse with byte, depth, component-count, and time bounds; reject an unknown
   incompatible contract major or mutable component identity.
5. Produce a source-free preview of component and limit changes.
6. Evaluate node apply policy and local developer policy. A fleet revision that
   attempts to widen local permission fails or waits until the developer changes
   local policy through a separate local action; approving the Events operation
   cannot itself broaden permission or partially rewrite the bundle.
7. Stage the complete bundle and ask Core to activate it through an authenticated
   local control boundary.
8. Core compiles the effective configuration, atomically swaps the immutable
   active snapshot, and records the Events desired event and operation IDs.
9. Verify Core reports the exact authority, resource, generation, revision,
   digest, and adapter as active, plus the effective `ConfigurationSnapshot`
   identifier and digest and whether local policy narrowed the proposal, before
   emitting `reconcile.applied`.

Writing a file, receiving a broker message, producing a preview, or returning a
successful local control response is not sufficient application evidence.
An operation awaiting approval is durable. Duplicate delivery reuses the same
operation and preview without repeated fetch, preview, prepare, or apply side
effects until the operation is approved, rejected, expired, or superseded.

## Atomicity, crash recovery, and rollback

The active managed identity is the tuple:

```text
(authority, resource, generation, revision, content_sha256, adapter)
```

- A crash before Core's atomic activation leaves the prior revision active.
- A crash after activation but before durable outcome evidence is
  `operation.indeterminate`; recovery queries Core's exact active identity
  before deciding whether to resume, verify, or require review.
- The adapter never silently replays an indeterminate external apply.
- A conclusive activation or read-back mismatch may restore the prior local
  snapshot under the declared rollback policy and must then observe the restored
  identity. Inability to observe is `operation.indeterminate`; it never triggers
  a blind automatic rollback or an applied outcome.
- An authority-requested rollback publishes the previously accepted bytes as a
  new, higher generation. Resource generation never moves backward.

## Evidence and correlation

Events and Edit retain separate authorities and join through:

- Events `event_id`, `correlation_id`, and reconciliation `operation_id`;
- Edit configuration-activation attempt ID;
- desired resource, generation, revision, and content digest; and
- adapter and Core instance revisions at the permitted disclosure level.

Events may receive source-free activation outcomes. It must not receive editor
opportunities, snapshots, requests, candidates, presentation/application
attempts, human outcomes, source digests, paths, prompts, or raw trace data.
Missing or conflicting activation claims remain explicit and block any
dependent fleet-promotion conclusion.

## Failure behavior

| Failure | Required result |
| --- | --- |
| Events or broker unavailable | No hot-path call; retain the permitted last verified snapshot or remain disabled |
| Desired artifact unavailable or digest mismatch | Fail reconciliation; keep the prior active snapshot |
| Unknown bundle major or incompatible Core range | Fail before activation |
| Unauthorized authority/resource/adapter binding | Reject and record source-free denial evidence |
| Fleet bundle widens local permission | Local policy wins; do not activate the broader permission |
| Core unavailable before activation | Fail or await approval without changing active state |
| Crash during activation | Mark indeterminate and verify exact active identity before recovery |
| Core reports a conclusively different active identity | Preserve both claims, mark conflict, and use only the declared, observable rollback policy |
| Core active identity cannot be observed | Remain indeterminate; do not replay activation, auto-rollback, or emit applied |
| Rollback verification fails | Report failure and require explicit recovery |

## Language and process boundary

The integration does not require Anvil Events and Anvil Edit to share a
language or process. One hypothesis under O002 is a Rust Anvil Edit Core and a
separate Anvil Events agent using a versioned local activation protocol or
managed-file handoff with an exact Core acknowledgement.

Anvil Events may remain its current stdlib Python implementation because
configuration convergence is not latency-critical. A later Rust or other
language refactor is permitted only when it preserves the v2 envelope, durable
store migration/export, reconciliation state machine, failure semantics, and
cross-version conformance fixtures. The evidence-grounded invariant is that the
Edit prediction hot path has no synchronous dependency on the Events process,
its language runtime, or its external stores.

## Required conformance evidence before implementation is called integrated

- duplicate, stale, out-of-order, and conflicting generations;
- incompatible and malformed bundles with zero active-state change;
- exact authority/resource/adapter denial cases;
- local-pause and fleet-cannot-widen permission fixtures;
- broker outage and catch-up without prediction-path latency coupling;
- crash injection before activation, after activation, and before outcome;
- exact active-identity verification and higher-generation rollback;
- new-node catch-up after any future snapshot/compaction implementation;
- no source-bearing bytes in events, artifacts, logs, errors, or outcomes; and
- mixed-version Events/Edit compatibility across every supported platform.

No source merge, passing fixture, configured resource, or successful reconcile
constitutes installation, deployment, policy promotion, or live editor
acceptance.

## Dependencies and open work

- Upstream Anvil Events issue:
  [fakoli/anvil-events#13](https://github.com/fakoli/anvil-events/issues/13).
- Anvil Edit O002 must decide Core language and process shape.
- Anvil Edit O003 must select concrete wire and durable schemas.
- Anvil Edit O005 and O013 must define repository-policy and trust-boundary
  implementations before managed configuration can affect source dispatch.
- Anvil Edit O015 must define managed staleness and emergency-revocation
  behavior.
- Anvil Events needs a typed external activation/verification seam and a tested
  latest-generation snapshot/compaction path before broad fleet use.
