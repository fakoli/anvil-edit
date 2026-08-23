# Privacy and trust

Status: **normative foundation; implementation threat model pending**

Last reviewed: **2026-08-23**

Editor traces are unusually sensitive. They can contain proprietary source,
secrets, customer data, internal URLs, abandoned experiments, and behavior that
never enters version control. “Local” is not sufficient unless collection,
persistence, access, export, and deletion are explicit.

## Trust goals

Anvil Edit should be:

- private by default;
- quiet by default;
- inspectable about context and model choice;
- designed to reject stale edits through exact revision fencing;
- reversible at the user interaction layer;
- honest about evidence and uncertainty; and
- unsuitable for covert individual productivity scoring by design.

## Data classes

| Class | Examples | Default handling |
| --- | --- | --- |
| P0 configuration | Policy revision, protocol identifier, non-secret limits | Local persistence allowed |
| P1 operational metadata | Timing, status, token count, pseudonymous IDs, digests | Local persistence allowed; bounded retention |
| P2 derived edit metadata | Ranges, edit size, validation result, outcome category | Local persistence allowed when it cannot reconstruct source |
| P3 source-bearing content | Buffer text, snippets, diffs, prompts, model output, symbols, paths | Memory-only unless repository and capture are explicitly enabled |
| P4 highly sensitive content | Detected secrets, credentials, private keys, protected files | Deny persistence and export; surface a local warning when safe |

Hashes are identifiers, not anonymization. Small or known source fragments may
be recoverable by guessing. They remain governed metadata.

## Default policy

### Runtime processing

Core may process the active, allowed buffer and bounded context in memory to
produce a prediction. No remote endpoint is used unless the developer has
configured and enabled it for the repository or workspace.

### Metadata capture

Local operational and outcome metadata may be recorded to make latency,
cancellation, and utility visible. It must avoid source text, raw prompts,
document paths, secrets, and unbounded model output.

### Source-bearing trace capture

Disabled until all of these are true:

1. the repository or workspace is explicitly allowlisted;
2. the user has enabled a named capture purpose;
3. retention and storage location are visible;
4. ignore and protected-path rules have been evaluated; and
5. the UI exposes pause and deletion controls.

Enabling predictions is not consent to persist training or replay data.

### Export

All export is off by default. Export requires a destination-specific action,
preview, declared content classes, redaction result, provenance manifest, and
user confirmation. A team setting or cloud teacher must not inherit local raw
trace permission implicitly.

### Training

Captured traces are not training data merely because they exist. Weight tuning
requires a separate consent and provenance purpose, owned or permissively
licensed inputs, a deletion policy, a time-split holdout, and a promotion gate.

## Authorization dimensions

Permission is not one `enabled` bit. The policy compiles independent grants for:

| Grant | Authorizes |
| --- | --- |
| Runtime read | Core may read selected source in memory for a local operation |
| Executor dispatch | A named destination/trust domain may receive declared content classes |
| Persistence | A named store may retain declared classes for a finite period |
| Replay | Lab may reconstruct a named permitted corpus for a declared purpose |
| Export | A named external destination may receive a previewed package |
| Training | Declared inputs may be used to change weights/adapters |
| Shadow | A non-visible candidate may receive context and produce evidence |
| Task context | Bounded task fields may enter context |
| Outcome correlation | Session outcomes may be joined across declared checkpoints |

The effective policy is finite and deterministic: deny rules union, allowlists
intersect, minimum retention wins, the local pause switch wins, fleet policy
may narrow but never expand local permission, and unknown values fail closed.
The effective digest and grant are recorded. A prior grant, repository
allowlist, configured endpoint, or task identifier grants no adjacent purpose.

Before any source-bearing value is serialized for another process or host,
Core constructs an `ExecutionGrant` naming destination/operator trust domain,
capability and protocol revision, purpose, visible/shadow mode, allowed content
classes, policy digest, and expiry. Denial abstains before network dispatch.
Context selection requires its own already-effective local runtime-read grant;
the later content-bound dispatch grant does not retroactively authorize reads.

## Repository policy

Each repository can declare:

- prediction enabled or disabled;
- permitted document globs and languages;
- protected/ignored globs;
- metadata retention;
- source-bearing capture mode and retention;
- permitted executors/destinations;
- whether task context may be read;
- whether shadow models may receive context;
- whether save/commit correlation is allowed; and
- whether export is allowed at all.

The most restrictive applicable workspace, repository, file, and destination
rule wins. Policy denial causes abstention or smaller context, not a hidden
override.

## Protected content

At minimum, default protected rules should cover common credential files,
private keys, environment files, VCS internals, editor secret stores, package
credentials, and user-configured sensitive paths.

Secret scanning is defense in depth, not the primary permission boundary. It
can miss secrets and must not justify broad capture. Scanner output must not
echo the secret into logs.

Untitled, temporary, generated, vendored, and very large files require explicit
policy rather than accidental inclusion.

## Threat boundaries and key lifecycle

The implementation inventory names the editor UI host, extension/adapter host,
Core host, metadata/content-store host, Lab host, executor host, operator, user
principal, and destination trust domain. Processes on one machine are not
automatically one principal.

Before source persistence or remote dispatch, the threat model specifies:

- IPC peer authentication and authorization;
- transport encryption and endpoint identity across host/trust boundaries;
- encryption-key creation, OS-backed storage, rotation, recovery, revocation,
  and deletion behavior;
- effects of lock screen, logout, multi-user machines, malware, and stolen
  disks;
- crash dump, swap/pagefile, temporary file, clipboard, telemetry, log, and
  diagnostic-bundle exposure; and
- backup, sync, export, and disaster-recovery copies.

Key loss, key compromise, and deletion-key destruction are distinct incidents.
The product must not claim encrypted deletion unless all retained ciphertext,
keys, backups, and linkable metadata satisfy the declared erasure contract.

Resolved URIs and paths, repository files, task text, prompts, and model output
are untrusted. Protected rules are evaluated after path/URI resolution. Output
parsing is bounded, plain-text only at the editor boundary, and checks secrets,
protected content, and dangerous Unicode control/bidirectional characters
without echoing matched bytes into evidence.

## Storage controls

The implementation threat model must specify protection against other local
users, accidental backup/sync, malware, and stolen disks. Foundation
requirements are:

- logical separation of metadata and source-bearing content;
- encryption at rest appropriate to the platform and threat model;
- no credential values in the trace store;
- finite, visible retention for source-bearing content;
- content-addressed integrity without treating hashes as anonymous;
- purpose- and repository-scoped content addressing; no global cross-repository
  deduplication by default;
- physical deletion by repository, session, time range, purpose, and all data;
- erasure of source blobs, linkable metadata, derived indexes, declared export
  packages, and governed backup copies;
- minimal non-linkable deletion receipts that retain neither content digests
  nor stable repository/session identifiers;
- explicit partial-failure reporting and retry for deletion targets;
- bounded logs and metrics with no source text; and
- backup/sync behavior that is explicit rather than inherited accidentally.

The exact retention duration is an open product decision. “Forever” is not an
acceptable default.

Evidence is immutable only during its authorized retention. Append-only event
design does not override deletion. If SQLite/WAL is selected, deletion tests
must cover free pages, WAL/checkpoint behavior, `secure_delete`, vacuuming,
temporary files, filesystem snapshots, and backups rather than equating SQL
row deletion with byte erasure.

## User controls

The editor experience must make these controls accessible:

- predictions on/off;
- subtle or explicit-reveal presentation;
- session recording indicator;
- pause recording;
- current repository policy;
- current model capability and local/remote destination;
- inspect why a context item was selected;
- inspect why a candidate was shown or suppressed when available;
- delete current session or repository traces; and
- export a manifest or permitted trace with preview.

Controls that materially change collection or destination must not be hidden in
an unrelated settings page.

## Presentation trust ladder

| Prediction | Default interaction |
| --- | --- |
| Obvious one-line local completion | Inline preview and explicit accept |
| Multi-line local rewrite | Diff preview |
| Another region in the same file | Location indicator, then preview |
| Another file | Explicit navigation and diff review |
| Multi-file propagation | One-by-one or grouped review; no silent apply |
| Terminal or test action | Suggest only in v0; never execute |
| Agent steering | Deferred; show the inferred bounded intent before use |

The system may compute a prediction without immediately rendering it. Subtle
or explicit-reveal modes let model availability and UI interruption remain
separate choices.

## Team and fleet governance

The default team view is aggregate product quality:

- model and policy latency;
- error, stale, and cancellation rates;
- aggregate display and outcome funnels;
- privacy/control failures; and
- promotion evidence.

It must not expose raw individual replay, keystroke cadence, hours worked, or
individual rankings by default. Any broader administrative access needs a
documented purpose, notice, access audit, retention policy, and legal review.

Team reports are built from local aggregation by default. They exclude stable
individual, session, and repository identifiers; exact timestamps; event
cadence; free text; paths; and source digests. Publication requires a declared
minimum cohort and dimensionality threshold, purpose-scoped rotated report
identifiers, and suppression of sparse cells. Data classification follows what
a report can reveal after joining with other available data, not the apparent
innocence of each column in isolation.

Anvil Edit is not designed as employee monitoring software.

## Incident expectations

The implementation must define response paths for:

- source sent to an unapproved destination;
- secret or protected-file capture;
- trace access by an unauthorized principal;
- deletion or retention failure;
- stale or wrong-buffer edit presentation/application;
- unrecorded model substitution; and
- metrics or logs containing source.

The local runtime should be able to disable prediction and export independently
so investigation does not require destroying evidence or shutting down the
editor.

## Governance references

[NIST AI RMF](https://www.nist.gov/itl/ai-risk-management-framework), its
[Generative AI Profile](https://www.nist.gov/publications/artificial-intelligence-risk-management-framework-generative-artificial-intelligence),
and the [Secure Software Development Framework](https://csrc.nist.gov/Projects/ssdf)
are useful voluntary references for risk, provenance, evaluation, incident,
and supply-chain controls. Referencing them is not a compliance or
certification claim.

Employment, privacy, export, and cross-jurisdiction requirements must be
reviewed for the actual deployment at launch; they are not frozen by this
foundation document.
