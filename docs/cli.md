# ContextLint CLI Reference

## Scan

```bash
contextlint scan
contextlint scan --json
contextlint scan --path ./my-project
contextlint scan --fail-under 70
contextlint scan --include "docs/**/*.md"
contextlint scan --exclude "docs/archive/**"
```

`scan --json` prints valid JSON only.

## Report

```bash
contextlint report --format markdown
contextlint report --format json
contextlint report --format markdown --output contextlint-report.md
```

## Init

```bash
contextlint init
contextlint init --force
```

Creates `.contextlintrc.json`.

## Generate agents

```bash
contextlint generate agents
contextlint generate agents --output AGENTS.generated.md
contextlint generate agents --from CLAUDE.md,README.md,docs/architecture.md
```

Generates a compact agent instruction candidate. Review before replacing existing files.

## Completions

```bash
contextlint completions bash > contextlint.bash
contextlint completions zsh > _contextlint
contextlint completions fish > contextlint.fish
```

## Exit codes

```txt
0 = command succeeded
1 = command failed or score below threshold when --fail-under is used
```
