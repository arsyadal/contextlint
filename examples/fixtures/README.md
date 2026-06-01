# ContextLint Fixtures

Sample projects used to demo ContextLint behavior.

## Messy context

Path: `examples/fixtures/messy-context`

Contains intentional issues:

- duplicate instruction across `README.md`, `CLAUDE.md`, `AGENTS.md`, and `.cursorrules`
- risky phrases like `Ignore tests` and `Skip validation`
- outdated technology notes (`Express.js` mentioned but dependency missing)
- missing file/folder references
- missing npm scripts
- noisy/stale historical notes

Run:

```bash
contextlint scan --path examples/fixtures/messy-context
contextlint scan --path examples/fixtures/messy-context --json
contextlint generate agents --path examples/fixtures/messy-context --output AGENTS.generated.md
```
