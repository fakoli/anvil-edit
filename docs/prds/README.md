# Product PRDs

These are the committed, parser-compatible implementation contracts for the
foundation products:

| Anvil PRD ID | Product source | Scope |
| --- | --- | --- |
| `core-v0` | [Core v0](core-v0.md) | Lifecycle, authorization, fencing, executor seam, and local evidence kernel |
| `lab-v0` | [Lab v0](lab-v0.md) | Pinned replay, metrics, experiment design, and promotion reports |
| `flow-v0` | [Flow v0](flow-v0.md) | One supported editor, restrained UX, controls, and visible dogfood |

The files are independently parseable with Anvil's named-PRD support. The local
Anvil workspace keeps its own resolved source copies and event/state evidence;
those runtime files are not committed. When a PRD changes, update the committed
source, resolve the named workspace source with
`anvil prd source-name --prd <id> --json`, keep the content identical, and
re-parse that partition. Do not construct `.anvil/prds/` filenames manually.

The PRDs are authored drafts. Parsing creates draft State; review and permanent
approval remain separate human gates. Cross-PRD sequencing is:

```text
Core contract and privacy gates
        -> Lab deterministic replay
        -> Flow exploratory reveal
        -> Flow confirmatory visible dogfood
        -> later semantic-routing or product-expansion PRD
```

Engineering may overlap after shared schemas are reviewed, but an exposure
gate never inherits authority from another PRD.
