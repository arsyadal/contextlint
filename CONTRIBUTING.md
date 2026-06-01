# Contributing

Thanks for helping improve ContextLint.

## Development

```bash
cargo fmt --check
cargo check --locked
cargo clippy --locked -- -D warnings
cargo test --locked
```

## Local scan

```bash
cargo run -- scan
cargo run -- scan --json
```

## Pull requests

Please include:

- What changed
- Why it changed
- Tests for rule behavior when possible
- Before/after CLI output for user-facing changes

## Rule changes

Rules should prefer deterministic behavior and low false positives.

For new rules, include:

- Stable rule ID
- Severity
- Suggestion text
- Tests
- README or docs update if user-facing
