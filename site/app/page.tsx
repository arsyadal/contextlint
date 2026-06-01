import { AlertTriangle, FileSearch, Gauge, GitBranch, type LucideIcon } from 'lucide-react';

import { MotionCard, MotionReveal } from '@/components/motion-reveal';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Tooltip, TooltipProvider } from '@/components/ui/tooltip';

const features = [
  {
    index: '01',
    title: 'Duplicate instructions',
    text: 'Find repeated rules across Claude, Cursor, AGENTS, README, and docs.',
    icon: GitBranch,
  },
  {
    index: '02',
    title: 'Risky guidance',
    text: 'Flag unsafe phrases like ignoring tests, skipping validation, or dropping tables.',
    icon: AlertTriangle,
  },
  {
    index: '03',
    title: 'Outdated references',
    text: 'Detect missing files, stale command snippets, and technology mismatch notes.',
    icon: FileSearch,
  },
  {
    index: '04',
    title: 'Token waste',
    text: 'Estimate context size, score quality, and spotlight long noisy sections.',
    icon: Gauge,
  },
];

const beforeItems = [
  'Express mentioned, dependency missing',
  'Duplicate TypeScript instruction in 4 files',
  'Missing npm run build script',
  '5K-token historical note',
];

const afterItems = [
  'Severity-ranked issues',
  'File and line numbers',
  'Concrete cleanup suggestions',
  'JSON output for CI',
];

const metrics = [
  ['Install', 'brew / cargo', 'Available through Homebrew tap and crates.io.'],
  ['Runtime', 'single Rust CLI', 'Fast local binary. No Node runtime required for users.'],
  ['Privacy', 'no network scan', 'Scans local files only by default. No telemetry.'],
];

export default function Home() {
  return (
    <TooltipProvider>
      <a className="skip-link" href="#main">Skip to content</a>
      <div className="grain" aria-hidden="true" />
      <div className="orb orb-a" aria-hidden="true" />
      <div className="orb orb-b" aria-hidden="true" />

      <header className="site-header" aria-label="Main navigation">
        <a className="brand" href="#top" aria-label="ContextLint home">
          <span className="brand-mark" aria-hidden="true">CL</span>
          <span>ContextLint</span>
        </a>
        <nav className="nav" aria-label="Primary navigation">
          <a href="#features">Features</a>
          <a href="#demo">Demo</a>
          <a href="#install">Install</a>
          <a href="#ci">CI</a>
        </nav>
        <Button asChild variant="secondary" size="sm">
          <a href="https://github.com/arsyadal/contextlint">GitHub</a>
        </Button>
      </header>

      <main id="main">
        <section id="top" className="hero section-shell">
          <MotionReveal>
            <div className="hero-copy">
              <p className="eyebrow">Local-first CLI for AI coding agents</p>
              <h1>Stop feeding your AI agent noisy context.</h1>
              <p className="hero-lede">
                ContextLint audits <code>CLAUDE.md</code>, <code>AGENTS.md</code>,{' '}
                <code>.cursorrules</code>, README, and docs before Claude, Cursor, Codex, or Gemini
                reads stale instructions.
              </p>
              <div className="hero-actions" aria-label="Primary actions">
                <Button asChild>
                  <a href="#install">Install now</a>
                </Button>
                <Button asChild variant="secondary">
                  <a href="https://github.com/arsyadal/contextlint/releases/tag/v0.1.3">View release</a>
                </Button>
              </div>
              <dl className="metric-strip" aria-label="Project highlights">
                {metrics.map(([term, value, tip]) => (
                  <Tooltip key={term} content={tip}>
                    <div tabIndex={0}>
                      <dt>{term}</dt>
                      <dd>{value}</dd>
                    </div>
                  </Tooltip>
                ))}
              </dl>
            </div>
          </MotionReveal>

          <MotionReveal delay={0.15}>
            <Card className="terminal-card" aria-label="ContextLint terminal output demo">
              <div className="terminal-topbar" aria-hidden="true"><span /><span /><span /></div>
              <pre><code>{`$ contextlint scan --path examples/fixtures/messy-context

ContextLint v0.1.3

Files scanned: 6
Estimated total tokens: 5,753
Score: 0/100  Very Noisy

[CRITICAL] Risky instruction contains 'drop table'.
[MEDIUM] Duplicate instruction found in 4 files.
[MEDIUM] \`npm run build\` is not available.
[MEDIUM] Section 'Noisy Historical Context' is long.`}</code></pre>
            </Card>
          </MotionReveal>
        </section>

        <section className="problem-grid section-shell" aria-labelledby="problem-title">
          <div>
            <p className="eyebrow">The hidden tax</p>
            <h2 id="problem-title">Agent context rots faster than code.</h2>
          </div>
          <div className="problem-copy">
            <p>
              Every old command, duplicate rule, missing file reference, and risky instruction
              becomes prompt surface area. Agents waste tokens, follow stale architecture notes,
              and repeat obsolete project assumptions.
            </p>
          </div>
        </section>

        <section id="features" className="features section-shell" aria-labelledby="features-title">
          <p className="eyebrow">What it catches</p>
          <h2 id="features-title">A linter for the files your agent trusts most.</h2>
          <div className="feature-grid">
            {features.map((feature, index) => (
              <MotionCard key={feature.title} delay={index * 0.05}>
                <FeatureCard {...feature} />
              </MotionCard>
            ))}
          </div>
        </section>

        <section id="demo" className="demo section-shell" aria-labelledby="demo-title">
          <div className="section-heading">
            <p className="eyebrow">Demo fixture</p>
            <h2 id="demo-title">Messy context in. Actionable report out.</h2>
          </div>
          <div className="demo-grid">
            <DemoPanel tone="bad" title="Before" items={beforeItems} />
            <DemoPanel tone="good" title="After scan" items={afterItems} />
          </div>
        </section>

        <section id="install" className="install section-shell" aria-labelledby="install-title">
          <div>
            <p className="eyebrow">Install</p>
            <h2 id="install-title">Two commands. No account. No API key.</h2>
          </div>
          <div className="code-stack" aria-label="Installation commands">
            <CodeBlock label="Homebrew" code={`brew tap arsyadal/tap\nbrew install contextlint`} />
            <CodeBlock label="Cargo" code={`cargo install contextlint\ncontextlint scan`} />
          </div>
        </section>

        <section id="ci" className="ci section-shell" aria-labelledby="ci-title">
          <div className="ci-copy">
            <p className="eyebrow">CI-ready</p>
            <h2 id="ci-title">Fail pull requests when context quality drops.</h2>
            <p>Use the composite GitHub Action or run the binary directly in any pipeline.</p>
          </div>
          <CodeBlock
            className="wide"
            label="GitHub Actions"
            code={`jobs:\n  contextlint:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - uses: arsyadal/contextlint@v0.1.3\n        with:\n          fail-under: "70"`}
          />
        </section>
      </main>

      <footer className="site-footer section-shell">
        <p>ContextLint keeps AI context short, accurate, relevant, safe, and useful.</p>
        <div>
          <a href="https://github.com/arsyadal/contextlint">GitHub</a>
          <a href="https://crates.io/crates/contextlint">crates.io</a>
          <a href="https://github.com/arsyadal/homebrew-tap">Homebrew tap</a>
        </div>
      </footer>
    </TooltipProvider>
  );
}

function FeatureCard({
  index,
  title,
  text,
  icon: Icon,
}: {
  index: string;
  title: string;
  text: string;
  icon: LucideIcon;
}) {
  return (
    <Card className="feature-card">
      <CardHeader>
        <span className="feature-index">{index}</span>
        <Icon className="feature-icon" aria-hidden="true" />
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <CardContent>
        <CardDescription>{text}</CardDescription>
      </CardContent>
    </Card>
  );
}

function DemoPanel({ tone, title, items }: { tone: 'bad' | 'good'; title: string; items: string[] }) {
  return (
    <Card className={`demo-panel ${tone}`}>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <CardContent>
        <ul>
          {items.map((item) => <li key={item}>{item}</li>)}
        </ul>
      </CardContent>
    </Card>
  );
}

function CodeBlock({ label, code, className = '' }: { label: string; code: string; className?: string }) {
  return (
    <Card className={`code-block ${className}`}>
      <p>{label}</p>
      <pre><code>{code}</code></pre>
    </Card>
  );
}
