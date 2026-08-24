# Anvil Edit

> A local-first predictive coding assistant that tries to offer the next useful
> edit before you have to ask.

Developers often make a series of related changes: update a function, fix the
places that call it, then adjust a test. Anvil Edit is being built to notice
that direction and offer the next helpful change while the developer stays in
control.

## Current status

Anvil Edit is at the foundation stage. This repository now contains an initial
Rust workspace alongside the product definition, safety and privacy rules,
evaluation plan, product requirements, and development skills. The code proves
one narrow building block: work already underway keeps the configuration
identity it began with even if a newer identity is selected.

It is not yet a released application. It has no editor integration, model
connection, trace database, or proven latency and usefulness results.

Anvil Edit is not a new editor or coding model. It is intended to connect
editors, explicitly chosen model capabilities, and evidence about what actually
helps.

## What we plan to build

- **Anvil Flow v0** will be the first supported-editor experience. It will offer
  restrained, reviewable suggestions and stay quiet when it is unsure.
- **Anvil Edit Core** will be the safety and coordination layer. It will choose
  a permitted model capability, enforce deadlines, cancel outdated work, and
  bind every suggestion to the exact file version it came from. The editor will
  refuse to apply it if that file has changed.
- **Anvil Edit Lab** will compare models and policies on the same permitted
  editing moments. It will measure whether suggestions were timely, useful,
  and still present later.

Together, these parts are intended to let people choose among supported model
capabilities, run them on local or explicitly permitted infrastructure, and
improve the experience from evidence rather than model hype.

## What makes it different

Anvil Edit is meant to compare explicit model and policy combinations on the
same permitted editing moments, then require evidence from a visible developer
experience before recommending one. That connects private model choice to what
actually happened in the editor.

- **Local first.** Code and editing history stay local unless the developer
  explicitly permits another destination.
- **Quiet by design.** Choosing not to interrupt is a successful outcome.
- **Model independent.** No single model or inference provider owns the
  experience.
- **Fenced against stale files.** The system is designed to reject old
  suggestions when the file has moved on.
- **Measured honestly.** A fast response or impressive benchmark is not enough;
  a suggestion must show useful, retained outcomes in visible dogfood rather
  than merely score well offline.

The first goal is intentionally small: prove one useful, private, low-
interruption editing experience in one well-instrumented editor, then decide
what deserves to grow.

## Learn more

- [What the product should become](docs/PROJECT.md)
- [Core, Lab, and Flow product requirements](docs/prds/README.md)
- [How we will judge whether it works](docs/EVALUATION.md)
- [How privacy and developer control work](docs/PRIVACY-AND-TRUST.md)
- [The validation roadmap](docs/ROADMAP.md)
- [How the codebase is organized and checked](docs/DEVELOPMENT.md)

Building with us? Start with [the contributor guidance](AGENTS.md) and run
`cargo xtask check` before handing off a change. The
[development skills](plugins/anvil-edit-development/skills) help contributors
review safety, privacy, experiments, model evidence, and documentation as the
product grows. They are checked into this repository but are not proof that a
plugin has been installed or published.
