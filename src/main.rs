use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod action;
mod git;
mod parser;
mod workflow;

use workflow::WorkflowProcessor;

/// Pin GitHub Actions to specific commit SHAs for improved security
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the workflows directory (defaults to .github/workflows)
    #[arg(short, long, default_value = ".github/workflows")]
    workflows_dir: PathBuf,

    /// Perform a dry run without modifying files
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Create backup files before modifying
    #[arg(short, long)]
    backup: bool,

    /// Number of concurrent requests for resolving SHAs
    #[arg(short = 'j', long, default_value = "10")]
    jobs: usize,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Skip actions that are already pinned
    #[arg(long, default_value = "true")]
    skip_pinned: bool,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    format: OutputFormat,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    let log_level = if args.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .without_time()
                .with_level(true),
        )
        .with(tracing_subscriber::EnvFilter::from_default_env().add_directive(log_level.into()))
        .init();

    // Validate workflows directory exists
    if !args.workflows_dir.exists() {
        anyhow::bail!(
            "Workflows directory not found: {}",
            args.workflows_dir.display()
        );
    }

    if !args.workflows_dir.is_dir() {
        anyhow::bail!("Not a directory: {}", args.workflows_dir.display());
    }

    // Create processor
    let processor = WorkflowProcessor::new(
        args.workflows_dir.clone(),
        args.dry_run,
        args.backup,
        args.skip_pinned,
        args.jobs,
    );

    // Process workflows
    info!(
        "{}",
        format!("🔍 Scanning workflows in {}", args.workflows_dir.display()).cyan()
    );

    let results = processor.process().await?;

    // Display results
    match args.format {
        OutputFormat::Text => display_text_results(&results, args.dry_run),
        OutputFormat::Json => display_json_results(&results)?,
    }

    if results.errors > 0 {
        warn!("⚠️  Completed with {} errors", results.errors);
        std::process::exit(1);
    }

    Ok(())
}

fn display_text_results(results: &workflow::ProcessResults, dry_run: bool) {
    println!();
    println!("{}", "📊 Summary".bold().cyan());
    println!("{}", "─".repeat(50).cyan());
    println!("  Files processed:  {}", results.files_processed);
    println!("  Actions found:    {}", results.actions_found);
    println!(
        "  Actions pinned:   {}",
        results.actions_pinned.to_string().green()
    );
    println!("  Already pinned:   {}", results.already_pinned);
    println!(
        "  Errors:           {}",
        if results.errors > 0 {
            results.errors.to_string().red()
        } else {
            results.errors.to_string().green()
        }
    );
    println!("{}", "─".repeat(50).cyan());

    if dry_run {
        println!("\n{}", "ℹ️  Dry run mode - no files were modified".yellow());
    } else if results.actions_pinned > 0 {
        println!(
            "\n{}",
            "✅ All unpinned actions have been pinned to commit SHAs".green()
        );
    } else {
        println!("\n{}", "✨ No actions needed pinning".green());
    }
}

fn display_json_results(results: &workflow::ProcessResults) -> Result<()> {
    let json = serde_json::to_string_pretty(&results)?;
    println!("{}", json);
    Ok(())
}
