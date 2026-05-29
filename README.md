# ContextLint

Lint, score, and compress your AI agent context files.

ContextLint scans `CLAUDE.md`, `AGENTS.md`, `.cursorrules`, Cursor rules, `README.md`, and `docs/**/*.md` to find duplicate, outdated, risky, and token-wasting context before Claude, Cursor, Codex, Gemini, or another AI coding agent reads it.

## Status

Early MVP in Rust.

Implemented:

- `contextlint scan`
- `contextlint scan --json`
- `contextlint scan --fail-under <score>`
- `contextlint report --format markdown`
- `contextlint report --format json`
- `contextlint init`
- `contextlint generate agents`
- File discovery for common AI context files
- Approximate token estimation
- Duplicate instruction detection
- Noisy section detection
- Risky phrase detection
- Outdated note detection
- Missing backtick file reference detection
- Basic dependency/technology mismatch detection

## Install from source

```bash
cargo install --path .
```

## Usage

```bash
contextlint scan
contextlint scan --json
contextlint scan --path ./my-project --fail-under 70
contextlint report --output contextlint-report.md
contextlint init
contextlint generate agents --output AGENTS.generated.md
```

## Default scanned files

```txt
CLAUDE.md
AGENTS.md
.cursorrules
.cursor/rules/*
README.md
docs/**/*.md
.github/copilot-instructions.md
```

## Product principle

The best AI context is not the longest context. It is the clearest, safest, and most relevant context.
# contextlint
