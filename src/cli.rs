use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "contextlint")]
#[command(version)]
#[command(about = "Lint, score, and compress AI agent context files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Scan context files and print score/issues.
    Scan {
        /// Project root path.
        #[arg(long, default_value = ".")]
        path: PathBuf,

        /// Print valid JSON only.
        #[arg(long)]
        json: bool,

        /// Exit with code 1 when score is below threshold.
        #[arg(long)]
        fail_under: Option<u8>,

        /// Additional include glob. Repeatable.
        #[arg(long)]
        include: Vec<String>,

        /// Additional exclude glob. Repeatable.
        #[arg(long)]
        exclude: Vec<String>,
    },

    /// Generate detailed report.
    Report {
        /// Project root path.
        #[arg(long, default_value = ".")]
        path: PathBuf,

        /// Report format.
        #[arg(long, value_enum, default_value_t = ReportFormat::Markdown)]
        format: ReportFormat,

        /// Write report to file instead of stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },

    /// Create .contextlintrc.json.
    Init {
        /// Project root path.
        #[arg(long, default_value = ".")]
        path: PathBuf,

        /// Overwrite existing config.
        #[arg(long)]
        force: bool,
    },

    /// Generate compact agent context files.
    Generate {
        #[command(subcommand)]
        command: GenerateCommands,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ReportFormat {
    Markdown,
    Json,
}

#[derive(Debug, Subcommand)]
pub enum GenerateCommands {
    /// Generate compact AGENTS.md candidate.
    Agents {
        /// Project root path.
        #[arg(long, default_value = ".")]
        path: PathBuf,

        /// Output file path.
        #[arg(long, default_value = "AGENTS.generated.md")]
        output: PathBuf,

        /// Comma-separated source files.
        #[arg(long)]
        from: Option<String>,
    },
}
