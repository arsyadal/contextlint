mod cli;
mod config;
mod discovery;
mod generate;
mod model;
mod parser;
mod report;
mod rules;
mod score;
mod token;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::{Cli, Commands, GenerateCommands, ReportFormat};

fn main() {
    if let Err(error) = run() {
        eprintln!("contextlint: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            path,
            json,
            fail_under,
            include,
            exclude,
        } => {
            let config = config::load_config(&path)?;
            let result = rules::scan_project(&path, &config, &include, &exclude)?;

            if json {
                report::print_json(&result)?;
            } else {
                report::print_terminal(&path, &result);
            }

            let threshold = fail_under.or(config.score_threshold);
            if let Some(threshold) = threshold {
                if result.score < threshold {
                    std::process::exit(1);
                }
            }
        }
        Commands::Completions { shell } => {
            let mut command = Cli::command();
            let name = command.get_name().to_string();
            clap_complete::generate(shell, &mut command, name, &mut std::io::stdout());
        }
        Commands::Report {
            path,
            format,
            output,
        } => {
            let config = config::load_config(&path)?;
            let result = rules::scan_project(&path, &config, &[], &[])?;
            let rendered = match format {
                ReportFormat::Markdown => report::render_markdown(&path, &result),
                ReportFormat::Json => serde_json::to_string_pretty(&result)?,
            };

            if let Some(output) = output {
                std::fs::write(output, rendered)?;
            } else {
                println!("{rendered}");
            }
        }
        Commands::Init { path, force } => {
            config::init_config(&path, force)?;
        }
        Commands::Generate { command } => match command {
            GenerateCommands::Agents { path, output, from } => {
                generate::generate_agents(&path, &output, from.as_deref())?;
                println!("Generated {}", output.display());
            }
        },
    }

    Ok(())
}
