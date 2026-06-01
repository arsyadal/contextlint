import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'ContextLint — Lint AI agent context before it wastes tokens',
  description:
    'ContextLint scans CLAUDE.md, AGENTS.md, .cursorrules, README, and docs to find duplicate, outdated, risky, and token-wasting AI agent context.',
  metadataBase: new URL('https://contextlint.dev'),
  openGraph: {
    title: 'ContextLint',
    description: 'Lint, score, and compress your AI agent context files.',
    type: 'website',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'ContextLint',
    description: 'Lint, score, and compress your AI agent context files.',
  },
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
