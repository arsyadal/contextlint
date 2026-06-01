# Product Requirements Document

# ContextLint — Context Waste Detector for AI Coding Agents

## 1. Ringkasan Produk

**ContextLint** adalah tool CLI yang membantu developer mendeteksi pemborosan context pada file-file yang biasa digunakan AI coding agent, seperti `CLAUDE.md`, `AGENTS.md`, `.cursorrules`, `README.md`, folder `docs/`, dan memory file lainnya.

Tool ini melakukan audit terhadap instruksi yang duplikat, catatan arsitektur yang outdated, informasi yang terlalu panjang, noisy, tidak relevan, atau berisiko membuat AI agent salah arah. ContextLint juga memberikan skor kualitas context, estimasi biaya token per session, dan dapat menghasilkan versi `AGENTS.md` yang lebih compact.

## 2. Problem Statement

Developer modern mulai banyak menggunakan AI coding agent seperti Claude Code, Cursor, Codex, Gemini CLI, Cline, dan tool sejenis. Agar agent bekerja efektif, developer biasanya membuat file instruksi dan dokumentasi project, misalnya:

* `CLAUDE.md`
* `AGENTS.md`
* `.cursorrules`
* `README.md`
* `docs/architecture.md`
* `docs/setup.md`
* memory atau note hasil kerja sebelumnya

Masalahnya, file-file ini sering menjadi terlalu panjang, duplikatif, outdated, dan tidak terstruktur. Akibatnya:

1. AI agent membuang token untuk membaca context yang tidak penting.
2. Agent mengikuti instruksi lama yang sudah tidak relevan.
3. Dokumentasi project membingungkan karena ada konflik antarfile.
4. Biaya token meningkat.
5. Output agent menjadi kurang akurat.
6. Developer tidak tahu bagian mana dari context yang benar-benar useful atau hanya noise.

Belum banyak tool yang fokus pada audit kualitas context untuk AI agent. Kebanyakan tool hanya membantu kompres token atau retrieval memory, tetapi belum membantu developer membersihkan, menilai, dan merapikan context project secara sistematis.

## 3. Tujuan Produk

Tujuan utama ContextLint adalah membantu developer menjaga context AI agent tetap ringkas, relevan, akurat, dan hemat token.

Secara spesifik, ContextLint bertujuan untuk:

1. Mendeteksi instruksi yang duplikat di banyak file.
2. Mendeteksi catatan arsitektur yang outdated atau berpotensi konflik.
3. Memberikan skor kualitas context per file dan per project.
4. Mengklasifikasikan context menjadi useful, noisy, risky, atau outdated.
5. Mengestimasi token cost dari context yang dikirim ke AI agent.
6. Menghasilkan rekomendasi perbaikan context.
7. Menghasilkan versi compact `AGENTS.md` untuk digunakan lintas AI agent.

## 4. Target User

### 4.1 Primary User

**Developer yang menggunakan AI coding agent setiap hari.**

Contoh:

* Fullstack developer
* Indie hacker
* Open-source maintainer
* AI-assisted developer
* Developer yang menggunakan Claude Code, Cursor, Codex, Gemini CLI, Cline, atau Copilot

### 4.2 Secondary User

* Engineering team yang ingin membuat standar AI agent context di repo internal
* Tech lead yang ingin menjaga dokumentasi agent tetap bersih
* Developer tool enthusiast
* Maintainer template/starter kit

## 5. User Persona

### Persona 1: Solo Developer / Indie Hacker

**Nama:** Raka
**Kebutuhan:** Menggunakan Claude Code dan Cursor untuk membangun SaaS kecil.
**Masalah:** File `CLAUDE.md` makin panjang karena sering ditambah instruksi baru. Banyak aturan lama yang sudah tidak dipakai.
**Tujuan:** Ingin context agent tetap pendek, jelas, dan hemat token.

### Persona 2: Team Developer

**Nama:** Dina
**Kebutuhan:** Bekerja di team dengan repo besar dan dokumentasi banyak.
**Masalah:** AI agent sering salah karena membaca dokumentasi lama.
**Tujuan:** Ingin tahu file mana yang outdated dan instruksi mana yang saling konflik.

### Persona 3: Open-source Maintainer

**Nama:** Bayu
**Kebutuhan:** Memiliki project open-source dengan banyak contributor.
**Masalah:** Contributor menggunakan agent berbeda-beda, sehingga perlu file instruksi universal.
**Tujuan:** Ingin auto-generate `AGENTS.md` yang clean dan ringkas.

## 6. Value Proposition

ContextLint membantu developer menjawab pertanyaan:

> “Apakah context project saya membantu AI agent, atau justru membuatnya boros dan bingung?”

Nilai utama produk:

1. **Hemat token** — mengurangi context yang tidak perlu.
2. **Lebih akurat** — mengurangi instruksi konflik dan outdated.
3. **Lebih rapi** — membuat context file lebih terstruktur.
4. **Agent-agnostic** — dapat digunakan untuk Claude, Cursor, Codex, Gemini, dan agent lainnya.
5. **Developer-friendly** — berjalan sebagai CLI dan bisa masuk CI/CD.

## 7. Scope Produk

### 7.1 In Scope untuk MVP

MVP ContextLint akan fokus pada:

1. Scan file context umum:

   * `CLAUDE.md`
   * `AGENTS.md`
   * `.cursorrules`
   * `README.md`
   * `docs/**/*.md`
   * `.github/copilot-instructions.md`

2. Deteksi duplicate instruction.

3. Deteksi terlalu panjang atau noisy section.

4. Estimasi token cost.

5. Context quality score.

6. Rekomendasi perbaikan.

7. Generate compact `AGENTS.md`.

8. Output dalam format terminal dan JSON.

### 7.2 Out of Scope untuk MVP

Fitur berikut tidak masuk MVP:

1. GUI desktop.
2. Web dashboard.
3. Integrasi langsung dengan semua AI provider.
4. Semantic analysis berbasis embedding cloud.
5. Auto-edit file tanpa approval user.
6. Team collaboration.
7. Paid SaaS billing.
8. Real-time monitoring session agent.

## 8. Fitur Utama

## 8.1 Scan Context Files

User dapat menjalankan:

```bash
contextlint scan
```

ContextLint akan mencari file-file context di root project dan folder dokumentasi.

Default file yang discan:

```txt
CLAUDE.md
AGENTS.md
.cursorrules
.cursor/rules/*
README.md
docs/**/*.md
.github/copilot-instructions.md
```

Output contoh:

```txt
ContextLint Report

Files scanned: 8
Estimated total tokens: 18,420
Context score: 64/100

Issues found:
- 6 duplicate instructions
- 3 outdated architecture notes
- 4 noisy sections
- 2 risky/conflicting rules
```

## 8.2 Duplicate Instruction Detector

ContextLint mendeteksi instruksi yang muncul berulang di banyak file.

Contoh duplikasi:

```txt
Use TypeScript for all frontend code.
```

Muncul di:

```txt
CLAUDE.md
AGENTS.md
.cursorrules
docs/frontend.md
```

Output:

```txt
Duplicate Instruction
Rule: "Use TypeScript for all frontend code"
Found in:
- CLAUDE.md:12
- AGENTS.md:8
- .cursorrules:3

Suggestion:
Keep this instruction only in AGENTS.md and remove duplicates from other files.
```

## 8.3 Outdated Architecture Note Detector

ContextLint mendeteksi potensi catatan arsitektur yang sudah usang.

Indikator outdated:

1. Menyebut teknologi yang tidak lagi ada di dependency.
2. Menyebut folder/file yang sudah tidak ada.
3. Menyebut command yang gagal ditemukan di `package.json`, `Makefile`, atau script config.
4. Menyebut struktur lama yang tidak sesuai dengan project saat ini.
5. Terdapat kata seperti:

   * deprecated
   * old
   * legacy
   * temporary
   * TODO later
   * previous architecture
   * no longer used

Contoh output:

```txt
Outdated Note
File: docs/architecture.md:42

Text:
"This project uses Express.js for backend API."

Detected:
No Express dependency found in package.json.
NestJS dependency found instead.

Suggestion:
Update architecture note to reflect current backend framework.
```

## 8.4 Useful / Noisy / Risky Classification

Setiap bagian context akan diklasifikasikan menjadi:

### Useful

Context yang jelas membantu AI agent.

Contoh:

* coding convention
* architecture overview
* test command
* build command
* folder structure
* API rule
* database migration rule

### Noisy

Context yang terlalu panjang, berulang, atau tidak memberi instruksi langsung.

Contoh:

* penjelasan historis terlalu panjang
* changelog lama
* narasi project yang tidak relevan untuk coding
* dokumentasi marketing di file agent context

### Risky

Context yang berpotensi membuat agent salah.

Contoh:

* instruksi konflik
* command yang sudah tidak valid
* secret/token tidak sengaja masuk docs
* instruksi terlalu agresif seperti “delete all files before rebuild”
* referensi database production tanpa warning

### Outdated

Context yang kemungkinan sudah tidak sesuai dengan project saat ini.

Contoh:

* menyebut framework lama
* menyebut route lama
* menyebut folder yang sudah tidak ada
* menyebut dependency yang sudah dihapus

## 8.5 Context Score

ContextLint memberikan skor 0–100 untuk project.

Formula awal:

```txt
Context Score = 
100
- duplicate_penalty
- noisy_penalty
- outdated_penalty
- risky_penalty
- excessive_length_penalty
```

Contoh:

```txt
Context Score: 64/100

Breakdown:
- Duplicate instructions: -12
- Noisy sections: -8
- Outdated notes: -10
- Risky rules: -6
```

Kategori skor:

```txt
90–100 = Excellent
75–89  = Good
60–74  = Needs Cleanup
40–59  = Risky
0–39   = Very Noisy
```

## 8.6 Estimated Token Cost

ContextLint menghitung estimasi jumlah token dari context file.

Output:

```txt
Estimated token usage:
- CLAUDE.md: 4,200 tokens
- AGENTS.md: 2,100 tokens
- README.md: 6,800 tokens
- docs/architecture.md: 3,900 tokens

Total: 17,000 tokens
Estimated waste: 5,400 tokens
Potential reduction: 31%
```

Untuk MVP, token estimation dapat menggunakan pendekatan kasar:

```txt
estimated_tokens = character_count / 4
```

Versi berikutnya dapat menggunakan tokenizer provider-specific.

## 8.7 Auto-generate Compact AGENTS.md

User dapat menjalankan:

```bash
contextlint generate agents
```

ContextLint akan menghasilkan file:

```txt
AGENTS.generated.md
```

Isi file akan berisi context ringkas:

```md
# Agent Instructions

## Project Overview
This project is a fullstack web application using Next.js, TypeScript, and PostgreSQL.

## Tech Stack
- Frontend: Next.js + TypeScript
- Backend: NestJS
- Database: PostgreSQL
- Styling: Tailwind CSS

## Development Rules
- Use TypeScript for all new files.
- Follow existing folder structure.
- Do not modify database schema without migration.
- Run tests before final response.

## Commands
- Install: npm install
- Dev: npm run dev
- Test: npm run test
- Build: npm run build

## Important Notes
- Do not edit production config.
- Ask before deleting files.
```

User dapat membandingkan file lama dan file generated secara manual. Command `contextlint diff` masuk future feature, bukan MVP.

## 8.8 JSON Output

Untuk integrasi CI/CD atau tool lain:

```bash
contextlint scan --json
```

Output:

```json
{
  "score": 64,
  "files_scanned": 8,
  "total_estimated_tokens": 18420,
  "estimated_waste_tokens": 5400,
  "issues": [
    {
      "id": "duplicate-instruction-1",
      "rule_id": "duplicate-instruction",
      "severity": "medium",
      "file_path": "CLAUDE.md",
      "start_line": 12,
      "end_line": 12,
      "message": "Duplicate TypeScript instruction found in 3 files.",
      "suggestion": "Keep this rule only in AGENTS.md.",
      "confidence": 0.94
    }
  ]
}
```

## 9. User Flow

## 9.1 Basic CLI Flow

1. User masuk ke root project.
2. User menjalankan:

```bash
contextlint scan
```

3. ContextLint mencari file context.
4. ContextLint menganalisis isi file.
5. ContextLint menampilkan skor dan issue.
6. User menjalankan:

```bash
contextlint generate agents
```

7. ContextLint membuat `AGENTS.generated.md`.
8. User review hasilnya.
9. User rename/manual copy ke `AGENTS.md`.

## 9.2 CI Flow

1. Developer menambahkan ContextLint ke workflow CI.
2. CI menjalankan:

```bash
contextlint scan --fail-under 70
```

3. Jika score di bawah 70, CI gagal.
4. Developer harus membersihkan context file sebelum merge.

## 10. CLI Command Design

## 10.1 Scan

```bash
contextlint scan
```

Options:

```bash
contextlint scan --json
contextlint scan --path ./my-project
contextlint scan --include "docs/**/*.md"
contextlint scan --exclude "docs/archive/**"
contextlint scan --fail-under 70
```

## 10.2 Report

```bash
contextlint report
```

Menghasilkan report lebih detail.

Options:

```bash
contextlint report --format markdown
contextlint report --output contextlint-report.md
```

## 10.3 Generate Compact Agent File

```bash
contextlint generate agents
```

Options:

```bash
contextlint generate agents --output AGENTS.generated.md
contextlint generate agents --from CLAUDE.md,README.md,docs/architecture.md
```

## 10.4 Config Initialization

```bash
contextlint init
```

Menghasilkan:

```txt
.contextlintrc.json
```

Contoh config:

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
  "rules": {
    "duplicateInstruction": true,
    "outdatedArchitecture": true,
    "riskyInstruction": true,
    "noisySection": true
  }
}
```

## 11. Functional Requirements

## 11.1 File Discovery

ContextLint harus dapat:

1. Membaca file Markdown dan text-based config.
2. Melakukan recursive scan di folder `docs/`.
3. Mengabaikan folder seperti:

   * `node_modules`
   * `.git`
   * `dist`
   * `build`
   * `.next`
   * `coverage`

## 11.2 Parsing

ContextLint harus dapat:

1. Membaca heading Markdown.
2. Memecah file menjadi section.
3. Menghitung panjang section.
4. Menyimpan metadata:

   * file path
   * line number
   * heading
   * content
   * token estimation

## 11.3 Duplicate Detection

ContextLint harus dapat:

1. Mendeteksi exact duplicate.
2. Mendeteksi near-duplicate sederhana.
3. Mengelompokkan instruksi yang mirip.
4. Memberikan rekomendasi file utama untuk menyimpan instruksi.

Untuk MVP:

* Exact match
* Normalized text similarity
* Jaccard similarity sederhana

## 11.4 Outdated Detection

ContextLint harus dapat:

1. Membaca dependency dari:

   * `package.json`
   * `pnpm-lock.yaml`
   * `yarn.lock`
   * `requirements.txt`
   * `go.mod`
   * `Cargo.toml`
2. Membandingkan teknologi yang disebut di docs dengan dependency aktual.
3. Mendeteksi referensi file/folder yang tidak ada.
4. Mendeteksi command yang tidak tersedia di script config.

## 11.5 Risky Rule Detection

ContextLint harus dapat mendeteksi pola instruksi berisiko, misalnya:

```txt
delete all
remove database
drop table
use production database
disable auth
ignore tests
skip validation
hardcode token
```

Output harus menyertakan severity:

```txt
low
medium
high
critical
```

## 11.6 Token Estimation

ContextLint harus dapat:

1. Menghitung estimasi token per file.
2. Menghitung total token project context.
3. Menghitung estimasi token yang bisa dihemat.
4. Menampilkan file paling boros.

## 11.7 Report Generation

ContextLint harus dapat menghasilkan report:

1. Terminal output.
2. JSON output.
3. Markdown report.

## 12. Non-Functional Requirements

## 12.1 Performance

Untuk MVP:

* Project kecil: selesai kurang dari 2 detik.
* Project menengah dengan 100 file Markdown: selesai kurang dari 10 detik.
* Tidak perlu koneksi internet.
* Tidak perlu API key.

## 12.2 Privacy

ContextLint harus bersifat local-first.

Ketentuan:

1. Tidak mengirim isi file ke server eksternal secara default.
2. Tidak membaca file di luar project tanpa izin.
3. Tidak menyimpan isi file ke remote.
4. Tidak mengirim telemetry pada MVP.

## 12.3 Compatibility

Minimal support:

* macOS
* Linux
* Windows

Target distribusi MVP:

* Rust CLI binary bernama `contextlint`
* Install awal via `cargo install --path .`
* Distribusi berikutnya via crates.io, GitHub Releases, dan Homebrew
* Tidak membutuhkan Node.js, Python, API key, atau koneksi internet

## 12.4 Reliability

ContextLint tidak boleh langsung mengubah file asli tanpa flag eksplisit.

Default behavior:

```txt
read-only scan
```

Untuk perubahan file:

```bash
contextlint fix --write
```

Fitur `fix --write` tidak masuk MVP awal.

## 13. Rule System

ContextLint menggunakan rule-based engine.

Contoh rule:

```txt
duplicate-instruction
outdated-architecture-note
missing-command
missing-file-reference
noisy-section
risky-instruction
excessive-token-cost
conflicting-instruction
```

Setiap rule memiliki:

```json
{
  "id": "duplicate-instruction",
  "severity": "medium",
  "description": "Detect duplicate instructions across context files",
  "enabled": true
}
```

## 14. Scoring System

Severity weight:

```txt
critical = -20
high     = -12
medium   = -6
low      = -2
```

Contoh calculation:

```txt
Base score: 100

Issues:
- 1 high risky instruction: -12
- 3 medium duplicate instructions: -18
- 2 low noisy sections: -4

Final score: 66
```

Score tidak boleh kurang dari 0.

## 15. MVP Specification

## 15.1 MVP Features

MVP wajib memiliki:

1. `contextlint scan`
2. Auto-discovery file:

   * `CLAUDE.md`
   * `AGENTS.md`
   * `.cursorrules`
   * `README.md`
   * `docs/**/*.md`
   * `.cursor/rules/*`
   * `.github/copilot-instructions.md`
3. Estimated token count.
4. Duplicate instruction detection.
5. Noisy section detection.
6. Risky phrase detection.
7. Basic outdated detection berdasarkan dependency dan missing file reference.
8. Context score.
9. Terminal report.
10. JSON output.
11. Generate `AGENTS.generated.md`.

## 15.2 MVP Success Criteria

MVP dianggap berhasil jika:

1. Bisa dijalankan di minimal 5 repo berbeda.
2. Bisa menemukan duplicate instruction dengan cukup akurat.
3. Bisa menunjukkan estimasi token per file.
4. Bisa menghasilkan `AGENTS.generated.md` yang lebih pendek dari context awal.
5. Bisa berjalan tanpa API key.
6. Bisa dipahami developer dalam kurang dari 5 menit.

## 15.3 MVP Acceptance Criteria

Command wajib:

1. `contextlint scan`
   * Exit code `0` jika scan berhasil.
   * Threshold hanya memengaruhi exit code jika user memakai `--fail-under` atau config `scoreThreshold`.
   * Menampilkan file scanned, token summary, score, dan issues.
2. `contextlint scan --json`
   * Output valid JSON tanpa text tambahan.
   * Field wajib: `score`, `files_scanned`, `total_estimated_tokens`, `estimated_waste_tokens`, `issues`.
3. `contextlint scan --fail-under 70`
   * Exit code `1` jika score di bawah `70`.
   * Exit code `0` jika score `>= 70`.
4. `contextlint report --format markdown`
   * Menghasilkan report Markdown ke stdout atau file jika `--output` diberikan.
5. `contextlint init`
   * Membuat `.contextlintrc.json` jika belum ada.
   * Tidak overwrite config existing tanpa konfirmasi/flag.
6. `contextlint generate agents`
   * Membuat `AGENTS.generated.md` secara default.
   * Mendukung `--output <path>`.

## 15.4 MVP Rule Thresholds

Rule awal memakai threshold deterministic agar mudah dites.

1. Token estimation:
   * `estimated_tokens = ceil(character_count / 4)`.
2. Noisy section:
   * `medium` jika section `> 1,000` estimated tokens.
   * `low` jika file `> 4,000` estimated tokens.
3. Duplicate instruction:
   * Normalize: lowercase, trim whitespace, hapus punctuation dasar.
   * Exact duplicate: similarity `1.0`.
   * Near duplicate: Jaccard similarity `>= 0.82` untuk kalimat 8–80 kata.
4. Risky instruction:
   * `critical`: `delete all`, `drop database`, `drop table`, `wipe production`.
   * `high`: `disable auth`, `use production database`, `hardcode token`.
   * `medium`: `ignore tests`, `skip validation`, `bypass security`.
5. Outdated note:
   * `medium` jika backticked file/folder path tidak ada.
   * `medium` jika docs menyebut dependency yang tidak ditemukan di manifest.
   * `low` jika mengandung marker `legacy`, `deprecated`, `old`, `temporary`, `TODO later`.

## 15.5 JSON Output Contract

Contoh shape final MVP:

```json
{
  "score": 68,
  "files_scanned": 5,
  "total_estimated_tokens": 15820,
  "estimated_waste_tokens": 4600,
  "issues": [
    {
      "id": "duplicate-instruction-1",
      "rule_id": "duplicate-instruction",
      "severity": "medium",
      "file_path": "CLAUDE.md",
      "start_line": 12,
      "end_line": 12,
      "message": "Duplicate instruction found in 3 files.",
      "suggestion": "Keep this instruction in AGENTS.md and remove duplicates.",
      "confidence": 0.94
    }
  ]
}
```

## 16. Future Features

## 16.1 Provider-specific Tokenizer

Support tokenizer untuk:

* Claude
* GPT
* Gemini
* local model

## 16.2 AI-assisted Rewrite

ContextLint dapat menggunakan LLM untuk rewrite context agar lebih compact.

Command:

```bash
contextlint rewrite --provider openai
contextlint rewrite --provider anthropic
contextlint rewrite --local ollama
```

## 16.3 MCP Server

ContextLint dapat berjalan sebagai MCP server.

Use case:

* Claude Code bisa bertanya ke ContextLint:

  * “What is the cleanest project context?”
  * “Which docs are outdated?”
  * “Which instructions should I follow?”

## 16.4 GitHub Action

Contoh workflow:

```yaml
name: ContextLint

on:
  pull_request:

jobs:
  contextlint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: contextlint/contextlint-action@v1
      - run: contextlint scan --fail-under 70
```

## 16.5 VS Code Extension

Menampilkan warning langsung di editor:

* Duplicate instruction
* Noisy section
* Risky instruction
* Estimated token cost per file

## 16.6 Context Diff

Membandingkan perubahan context antar commit.

```bash
contextlint diff main
```

Output:

```txt
Context token increased by 31%
New risky instruction added in CLAUDE.md
Duplicate rule added in .cursorrules
```

## 17. Tech Stack Recommendation

## 17.1 Keputusan Stack MVP

MVP menggunakan **Rust**.

Alasan:

* Cocok untuk CLI linting yang cepat dan local-first.
* Bisa didistribusikan sebagai single binary.
* Mudah dipakai di CI tanpa runtime tambahan.
* Aman untuk file scanning dan rule engine.
* Selaras dengan README project: “Early MVP in Rust.”

Recommended stack:

```txt
Language: Rust 2021
CLI Framework: clap
Serialization: serde, serde_json
Error Handling: anyhow, thiserror
File Walk: ignore atau walkdir
Glob Matching: globset
Markdown Parsing: pulldown-cmark atau parser heading sederhana untuk MVP
Config: serde_json
Testing: cargo test, assert_cmd, predicates, tempfile
Distribution: cargo install, crates.io, GitHub Releases, Homebrew later
```

## 17.2 Crate Layout

```txt
Cargo.toml
src/
  main.rs
  cli.rs
  config.rs
  discovery.rs
  parser.rs
  token.rs
  rules/
    mod.rs
    duplicate.rs
    noisy.rs
    risky.rs
    outdated.rs
  score.rs
  report.rs
  generate.rs
  model.rs
```

## 17.3 MVP Dependency Policy

Gunakan dependency sedikit dan stabil.

Wajib:

* `clap` untuk command dan flags.
* `serde` + `serde_json` untuk JSON output/config.
* `anyhow` untuk error handling aplikasi.
* `globset` atau `ignore` untuk discovery.

Opsional jika dibutuhkan:

* `pulldown-cmark` untuk parsing Markdown.
* `similar` untuk near-duplicate detection.

Tidak masuk MVP:

* LLM API.
* Embedding.
* Database.
* Telemetry.
* Background daemon.

## 18. Data Structure Internal

Contoh struktur internal:

```rust
#[derive(Debug, Clone, Serialize)]
pub struct ContextFile {
    pub path: String,
    pub file_type: ContextFileType,
    pub content: String,
    pub estimated_tokens: usize,
    pub sections: Vec<ContextSection>,
}

#[derive(Debug, Clone, Serialize)]
pub enum ContextFileType {
    Claude,
    Agents,
    Cursor,
    Readme,
    Docs,
    Copilot,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContextSection {
    pub id: String,
    pub file_path: String,
    pub heading: Option<String>,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub estimated_tokens: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    pub id: String,
    pub rule_id: String,
    pub severity: Severity,
    pub file_path: String,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    pub message: String,
    pub suggestion: Option<String>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub score: u8,
    pub files_scanned: usize,
    pub total_estimated_tokens: usize,
    pub estimated_waste_tokens: usize,
    pub issues: Vec<Issue>,
}
```

## 19. Example Output

```txt
ContextLint v0.1.0

Project: my-next-app

Files scanned:
✓ CLAUDE.md
✓ AGENTS.md
✓ .cursorrules
✓ README.md
✓ docs/architecture.md

Score: 68/100
Status: Needs Cleanup

Token Summary:
- Total estimated tokens: 15,820
- Estimated waste: 4,600
- Potential reduction: 29%

Top Issues:

[HIGH] Risky instruction
File: CLAUDE.md:44
"Ignore all test failures during refactor."

Suggestion:
Remove this instruction or replace with safer guidance.

[MEDIUM] Duplicate instruction
Found in:
- CLAUDE.md:12
- AGENTS.md:9
- .cursorrules:4

Instruction:
"Use TypeScript for all new files."

Suggestion:
Keep this rule only in AGENTS.md.

[MEDIUM] Outdated architecture note
File: docs/architecture.md:21
"This project uses Express.js."

Detected:
package.json contains NestJS, but Express is not listed directly.

Suggestion:
Update architecture documentation.
```

## 20. Monetization Possibilities

## 20.1 Open-source First

ContextLint sebaiknya dimulai sebagai open-source CLI.

Tujuan:

* Membangun trust.
* Mendapat GitHub stars.
* Mendapat feedback dari developer.
* Menjadi bagian dari workflow AI coding.

## 20.2 Paid Features Later

Jika traction bagus, monetisasi bisa melalui:

1. Team dashboard.
2. GitHub App.
3. Private repo analysis.
4. Context quality monitoring across multiple repos.
5. AI-assisted rewrite.
6. Team policy rules.
7. Context history and regression tracking.

Contoh pricing:

```txt
Free:
- Local CLI
- Basic scan
- JSON report
- Generate AGENTS.md

Pro:
- AI-assisted rewrite
- Provider-specific token estimation
- VS Code extension advanced mode

Team:
- GitHub App
- Repo-level context policy
- Team dashboard
- CI reporting
```

## 21. Go-to-Market Strategy

## 21.1 Positioning

Positioning utama:

> ContextLint helps developers clean, score, and compress AI agent context files before they waste tokens.

Alternatif tagline:

> Stop feeding your AI agent noisy context.

> Lint your AI context before Claude, Cursor, or Codex reads it.

> ESLint for AI agent context.

## 21.2 Target Channel

Channel awal:

1. GitHub
2. X/Twitter developer community
3. Reddit:

   * r/ClaudeAI
   * r/Cursor
   * r/LocalLLaMA
   * r/programming
   * r/webdev
4. Hacker News
5. Product Hunt
6. Dev.to
7. LinkedIn developer content

## 21.3 Launch Angle

Launch post:

```txt
I built ContextLint — a CLI that scans CLAUDE.md, AGENTS.md, .cursorrules, README, and docs to find duplicate, outdated, risky, and token-wasting context before your AI coding agent reads it.
```

Demo sederhana:

```txt
Before:
18,420 estimated tokens
12 duplicate rules
3 outdated architecture notes

After:
9,800 estimated tokens
Generated compact AGENTS.md
47% context reduction
```

## 22. Metrics

## 22.1 Product Metrics

1. Number of scans run.
2. Average context score.
3. Average token reduction.
4. Number of generated `AGENTS.md`.
5. Number of issues detected per repo.
6. Average project setup time.

## 22.2 Open-source Metrics

1. GitHub stars.
2. Crates.io downloads.
3. Issues opened.
4. PRs from contributors.
5. Forks.
6. Mentions on X/Reddit/Hacker News.
7. Crates.io downloads setelah publish.

## 22.3 Success Metrics for First 30 Days

Target realistis:

```txt
GitHub stars: 100+
Crates.io downloads: 500+
Useful issues from users: 10+
External contributors: 1–3
```

Target ambisius:

```txt
GitHub stars: 500+
Crates.io downloads: 2,000+
Mentioned in AI coding community


## 23. Risks

## 23.1 False Positive

ContextLint bisa salah menandai context sebagai duplicate/outdated padahal masih relevan.

Mitigasi:

* Jangan auto-delete.
* Beri confidence score.
* User tetap review manual.
* Tambahkan config ignore.

## 23.2 Too Generic

Jika output terlalu umum, developer tidak merasa terbantu.

Mitigasi:

* Beri line number.
* Beri suggestion konkret.
* Fokus pada file AI agent populer.
* Beri before/after token estimation.

## 23.3 Existing Tools Bisa Meniru

Tool besar seperti Cursor, Claude Code, atau Codex bisa menambahkan fitur serupa.

Mitigasi:

* Jadilah agent-agnostic.
* Fokus open-source.
* Support banyak format.
* Buat rule system yang extensible.

## 23.4 Hard to Measure Token Accurately

Token tiap model berbeda.

Mitigasi:

* MVP gunakan estimasi kasar.
* Label sebagai “estimated”.
* Future support provider-specific tokenizer.

## 24. Competitive Differentiation

ContextLint berbeda dari:

1. **Token compressor**

   * Token compressor hanya memendekkan text.
   * ContextLint menganalisis kualitas, risiko, dan relevansi context.

2. **Memory retrieval tool**

   * Memory retrieval mengambil informasi yang relevan.
   * ContextLint membersihkan dan menilai file context yang sudah ada.

3. **Documentation linter**

   * Documentation linter fokus grammar/format.
   * ContextLint fokus pada efektivitas context untuk AI coding agent.

4. **Static analyzer**

   * Static analyzer menganalisis code.
   * ContextLint menganalisis instruction dan documentation yang dikonsumsi AI agent.

## 25. Roadmap

## Phase 1 — MVP CLI

Durasi target: 2–4 minggu

Fitur:

* Scan file context
* Token estimation
* Duplicate detection
* Noisy section detection
* Risky phrase detection
* Context score
* JSON output
* Generate `AGENTS.generated.md`

## Phase 2 — Better Rules

Durasi target: 1–2 bulan

Fitur:

* Outdated dependency detection
* Missing file reference detection
* Command validation
* Config file
* Markdown report
* CI fail threshold

## Phase 3 — Ecosystem Integration

Durasi target: 2–3 bulan

Fitur:

* GitHub Action
* MCP server
* VS Code extension
* Provider-specific tokenizer
* Context diff

## Phase 4 — Pro/Team

Durasi target: setelah ada traction

Fitur:

* GitHub App
* Team dashboard
* AI-assisted rewrite
* Multi-repo monitoring
* Context quality trend

## 26. MVP Development Task Breakdown

## Week 1 — Rust CLI Foundation

1. Create `Cargo.toml` and `src/` layout.
2. Implement `clap` command structure.
3. Implement `contextlint scan`.
4. Implement file discovery for default context files.
5. Implement Markdown/text section parser.
6. Implement token estimator: `ceil(character_count / 4)`.
7. Print basic terminal report.

## Week 2 — Core Rules

1. Implement duplicate instruction detection.
2. Implement noisy section detection.
3. Implement risky phrase detection.
4. Implement scoring system.
5. Add `contextlint scan --json`.
6. Add unit tests for each rule.

## Week 3 — Outdated + Reports

1. Implement basic outdated detection.
2. Read `package.json`, `Cargo.toml`, `go.mod`, and `requirements.txt` when present.
3. Detect missing command references.
4. Detect missing file references from backticked paths.
5. Implement `contextlint report --format markdown`.
6. Implement `contextlint scan --fail-under <score>` exit code behavior.

## Week 4 — Generate + Release Prep

1. Implement `contextlint generate agents`.
2. Implement `contextlint init` for `.contextlintrc.json`.
3. Add integration tests with sample repos.
4. Update README usage examples.
5. Prepare demo repo/output snapshot.
6. Prepare crates.io metadata and GitHub release notes.

## 27. Example README Headline

```md
# ContextLint

Lint, score, and compress your AI agent context files.

ContextLint scans CLAUDE.md, AGENTS.md, .cursorrules, README, and docs to find duplicate, outdated, risky, and token-wasting context before Claude, Cursor, Codex, or Gemini reads it.
```

## 28. Final MVP Definition

ContextLint v0.1 dikatakan selesai jika user bisa menjalankan:

```bash
contextlint scan
```

dan mendapatkan:

1. daftar file context yang discan,
2. estimasi token per file,
3. skor context project,
4. daftar duplicate instruction,
5. daftar noisy/risky/outdated issue,
6. rekomendasi perbaikan,
7. opsi generate `AGENTS.generated.md`.

## 29. Core Product Principle

ContextLint tidak bertujuan menggantikan dokumentasi project.

ContextLint bertujuan memastikan dokumentasi dan instruksi yang dibaca AI agent tetap:

```txt
short
accurate
relevant
safe
useful
```

Prinsip produk:

> The best AI context is not the longest context. It is the clearest, safest, and most relevant context.
