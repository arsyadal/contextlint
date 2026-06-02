# Changelog

## Unreleased

- Add `contextlint diff <commit>` to compare current context against a git commit or branch.
- Show token delta, score delta, new issues, and resolved issues between commits.

## 0.1.3

- Add demo fixture for messy AI-agent context.
- Add README example output using the demo fixture.
- Document noisy, duplicate, risky, outdated, missing file, and missing command examples.

## 0.1.2

- Add composite GitHub Action via `uses: arsyadal/contextlint@v0.1.2`.
- Add release workflow for Linux, macOS, and Windows binaries.
- Add CLI and configuration docs.
- Add security policy, contributing guide, and issue templates.
- Expand README with GitHub Action usage and docs links.

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
