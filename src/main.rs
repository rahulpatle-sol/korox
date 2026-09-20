//! Cargo-korox CLI entry point

use cargo_korox::cli::{Cli, Commands};
use cargo_korox::commands::{run_check, run_fix, run_explain, run_watch};
use cargo_korox::config::{run_init, run_config};
use anyhow::Result;
use clap::Parser;
use std::process;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Commands::Check(args) => {
            let diagnostics = run_check(&args.path, cli.json, cli.quiet, args.changed)?;
            if cli.strict && !diagnostics.is_empty() {
                1
            } else {
                0
            }
        }
        Commands::Fix(args) => {
            run_fix(&args.path, cli.json, cli.quiet, args.changed)?;
            0
        }
        Commands::Explain(args) => {
            run_explain(&args.rule)?;
            0
        }
        Commands::Watch(args) => {
            run_watch(&args.path, cli.json, cli.quiet, args.changed, args.debounce).await?;
            0
        }
        Commands::Init => {
            run_init(&std::env::current_dir()?)?;
            0
        }
        Commands::Config => {
            run_config()?;
            0
        }
    };

    process::exit(exit_code);
}