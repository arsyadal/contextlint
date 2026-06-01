# Changelog

## 0.1.1

- Add inline ignore support with `contextlint-ignore` and `contextlint-ignore-next-line`.
- Add config ignore patterns by rule, path glob, or `rule-id:path/glob/**`.
- Add shell completions via `contextlint completions <shell>`.
- Add missing command/script detection for npm, pnpm, yarn, cargo, make, and just.
- Reduce false positives for outdated marker detection.
- Harden CI with fmt, check, clippy, tests, and package verification.

## 0.1.0

- Initial crates.io release.
- Add scan, JSON output, report, init, and generate agents commands.
- Add context file discovery, token estimation, scoring, and core lint rules.
