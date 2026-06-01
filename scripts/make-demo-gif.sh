#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$ROOT/assets/contextlint-demo.gif"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

FONT="/System/Library/Fonts/Menlo.ttc"
if [[ ! -f "$FONT" ]]; then
  FONT="/System/Library/Fonts/Monaco.ttf"
fi

cat > "$WORK/frame1.txt" <<'TXT'
$ contextlint scan

Scanning AI context files...

  CLAUDE.md
  AGENTS.md
  .cursorrules
  README.md
  docs/**/*.md
TXT

cat > "$WORK/frame2.txt" <<'TXT'
ContextLint v0.1.2

Files scanned: 8
Estimated total tokens: 18,420
Context score: 64/100
Status: Needs Cleanup

Issues found:
  - 6 duplicate instructions
  - 4 noisy sections
  - 2 risky rules
TXT

cat > "$WORK/frame3.txt" <<'TXT'
Top Issues

[HIGH] Risky instruction
CLAUDE.md:44
"Ignore all test failures during refactor."

[MEDIUM] Duplicate instruction
Use TypeScript for all new files.
Found in: CLAUDE.md, AGENTS.md, .cursorrules
TXT

cat > "$WORK/frame4.txt" <<'TXT'
$ contextlint generate agents

Generated AGENTS.generated.md

Before: 18,420 estimated tokens
After:   9,800 estimated tokens
Saved:   47 percent

Cleaner context. Fewer wasted tokens.
TXT

make_frame() {
  local n="$1"
  local title="$2"
  local subtitle="$3"
  local accent="$4"
  ffmpeg -hide_banner -loglevel error -y \
    -f lavfi -i "color=c=0x07111f:s=1200x720:d=1" \
    -vf "drawbox=x=0:y=0:w=1200:h=720:color=0x07111f:t=fill,drawbox=x=70:y=70:w=1060:h=580:color=0x0d1728:t=fill,drawbox=x=70:y=70:w=1060:h=48:color=0x111d33:t=fill,drawbox=x=92:y=88:w=14:h=14:color=0xff5f56:t=fill,drawbox=x=116:y=88:w=14:h=14:color=0xffbd2e:t=fill,drawbox=x=140:y=88:w=14:h=14:color=0x27c93f:t=fill,drawtext=fontfile='${FONT}':text='${title}':x=86:y=150:fontsize=34:fontcolor=0xffffff,drawtext=fontfile='${FONT}':text='${subtitle}':x=86:y=194:fontsize=22:fontcolor=${accent},drawtext=fontfile='${FONT}':textfile='${WORK}/frame${n}.txt':x=92:y=250:fontsize=25:fontcolor=0xd7e1f2:line_spacing=10" \
    -frames:v 1 "$WORK/frame${n}.png"
}

make_frame 1 "ContextLint" "Lint AI agent context before it wastes tokens" "0x7dd3fc"
make_frame 2 "Score + token cost" "Find noisy, duplicate, risky, outdated context" "0xfacc15"
make_frame 3 "Actionable issues" "Line numbers + suggestions for cleanup" "0xf87171"
make_frame 4 "Generate compact AGENTS.md" "Keep context short, accurate, relevant, safe, useful" "0x86efac"

cat > "$WORK/frames.txt" <<EOF2
file '$WORK/frame1.png'
duration 2.0
file '$WORK/frame2.png'
duration 2.3
file '$WORK/frame3.png'
duration 2.6
file '$WORK/frame4.png'
duration 2.4
file '$WORK/frame4.png'
duration 0.5
EOF2

ffmpeg -hide_banner -loglevel error -y \
  -f concat -safe 0 -i "$WORK/frames.txt" \
  -vf "fps=12,scale=960:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128[p];[s1][p]paletteuse=dither=bayer" \
  -loop 0 "$OUT"

printf 'Wrote %s\n' "$OUT"
