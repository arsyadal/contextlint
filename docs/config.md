# ContextLint Configuration

Create config:

```bash
contextlint init
```

Default file: `.contextlintrc.json`.

## Example

```json
{
  "include": [
    "CLAUDE.md",
    "AGENTS.md",
    ".cursorrules",
    "README.md",
    "docs/**/*.md"
  ],
  "exclude": [
    "node_modules/**",
    "dist/**",
    "build/**",
    "docs/archive/**"
  ],
  "scoreThreshold": 70,
  "tokenEstimator": "approximate",
  "ignore": [
    "duplicate-instruction",
    "docs/archive/**",
    "risky-instruction:docs/stale-context/**"
  ],
  "rules": {
    "duplicateInstruction": true,
    "outdatedArchitecture": true,
    "riskyInstruction": true,
    "noisySection": true
  }
}
```

## Include and exclude

`include` and `exclude` accept glob patterns relative to project root.

Default include:

```txt
CLAUDE.md
AGENTS.md
.cursorrules
.cursor/rules/*
README.md
docs/**/*.md
.github/copilot-instructions.md
```

Default exclude:

```txt
node_modules/**
.git/**
dist/**
build/**
.next/**
coverage/**
target/**
```

## Ignore patterns

`ignore` supports three forms:

```txt
rule-id
path/glob/**
rule-id:path/glob/**
```

Examples:

```json
{
  "ignore": [
    "duplicate-instruction",
    "docs/archive/**",
    "risky-instruction:docs/stale-context/**"
  ]
}
```

## Inline ignores

Ignore same line:

```md
Ignore tests during refactor. <!-- contextlint-ignore -->
```

Ignore next line:

```md
<!-- contextlint-ignore-next-line -->
Ignore tests during refactor.
```

## Rule IDs

```txt
duplicate-instruction
noisy-section
risky-instruction
outdated-architecture-note
missing-file-reference
missing-command
```
