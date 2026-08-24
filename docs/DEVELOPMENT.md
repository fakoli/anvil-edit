# Development guide

Status: **foundation scaffold**

Last reviewed: **2026-08-24**

Anvil Edit is being organized as a large Rust-centered, polyglot system. The
Rust workspace owns the latency-sensitive and safety-critical path. Editor
extensions and analysis tools may use the languages native to their ecosystems,
but they meet Core at explicit, versioned boundaries.

## Repository map

| Path | Owner | Current state |
| --- | --- | --- |
| `crates/anvil-edit-contracts` | Semantic domain types shared by Core surfaces | Implements the source-free foundation lifecycle model and critical structural invariants |
| `crates/anvil-edit-core` | Latency-sensitive policy and coordination | Implements atomic configuration-identity pinning only |
| `crates/anvil-editd` | Future local daemon process | Help/version shell; starts no server |
| `xtask` | Cross-platform developer verification | Runs Rust and product-guidance checks |
| `docs` | Canonical product, contract, architecture, privacy, evaluation, and PRD guidance | Normative until implemented and proven |
| `plugins/anvil-edit-development` | Explicit project development workflows | Repository source; installation is a separate state |

The allowed Rust dependency direction starts as:

```text
anvil-edit-contracts <- anvil-edit-core <- anvil-editd
```

`anvil-edit-contracts` has no I/O and does not choose serialization. Core does
not depend on an editor SDK, vendor model SDK, Anvil Events, or an analysis
runtime. Its future executor seam is an Anvil-owned contract and bounded Rust
transport, not a provider SDK that can choose or silently replace a model. The
daemon hosts Core but does not own domain policy.

## Working on the repository

Use the exact toolchain in `rust-toolchain.toml`. During a change, run the
smallest crate or test filter that covers the work. Before handoff, run:

```text
cargo xtask check
```

That command checks formatting, compilation, Clippy warnings, unit and doc
tests, Rust documentation, development-skill tests, and product-guidance
validation. CI runs the same entry point on Linux, Windows, and macOS.

Useful narrower commands are:

```text
cargo xtask rust
cargo xtask guidance
cargo run -p anvil-editd -- --help
```

## Adding Rust code

- Give each crate one clear owner and dependency direction. Do not introduce a
  general `utils` or `common` crate.
- Keep domain invariants in `anvil-edit-contracts`, behavior in the owning Core
  crate, and process concerns in `anvil-editd`.
- Keep unsafe Rust forbidden unless an accepted decision names the necessity,
  isolation boundary, and verification plan.
- Add dependencies once at the workspace root and use workspace inheritance.
- Pair concurrency behavior with deterministic tests. A throughput claim still
  requires a separately recorded benchmark with percentiles and identity.

## Adding a polyglot component

Use the editor's supported extension language for an adapter and an
analysis-oriented language for Lab only when it owns that work better than
Rust. A non-Rust package must have its own narrow manifest, lockfile, tests, and
owner. It may consume accepted schemas and conformance fixtures; it must not
redefine the canonical lifecycle in language-specific terms.

Do not add in-process FFI, a dynamic library ABI, or source-bearing IPC while
O003 and O013 are open. Once those decisions close, every boundary must version
its schema, authenticate its peer where required, preserve relative deadlines
and cancellation, and reject incompatible majors. An `ExecutionGrant` must be
resolved before source is serialized for another process or trust domain.

## Working with the semantic data model

[`DATA-MODEL.md`](DATA-MODEL.md) maps the Rust modules and aggregates back to
the normative lifecycle in [`CONTRACTS.md`](CONTRACTS.md). Keep foundational
identity, envelope, document, capture, dispatch, candidate, and outcome types
in their owning modules; do not collapse them into a generic event payload or a
shared utility module.

Durable records carry source-free `ContentReference` handles. Runtime source,
logical URI bytes, prompts, native output, and replacement text remain behind
the authorization and governed-content boundaries. Adding serialization,
database derives, transport framing, or generated bindings to the contract
crate requires O003 to be resolved first.

Context compilation consumes a `RuntimeReadGrant`; protocol serialization
consumes a later content-bound `ExecutionGrant`. Do not combine them, infer one
from the other, or reduce either to a configured endpoint or repository flag.

## Evidence and status

Code, docs, configuration, deployment, and product evidence are different
states. The daemon shell proves that the workspace builds; it does not prove a
running service. The semantic lifecycle fixture proves structural joins and
selected invariants, while the configuration test proves stable identity
pinning. Neither proves Events convergence, privacy authorization, denied
serialization behavior, inference, editor usefulness, or latency.

Local architecture output under `graphify-out` is ignored by Git. Public
examples and tests must remain sanitized and topology-neutral.
