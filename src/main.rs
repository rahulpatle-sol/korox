use crate::cli::{Cli, Commands};
use crate::commands::{run_check, run_explain};
use crate::config::{run_init, run_config};
use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::process;

mod cli;
mod scanner;
mod rules;
mod diagnostics;
mod config;
mod commands;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        crate::cli::Commands::Check(args) => {
            let diagnostics = run_check(&args.path, cli.json, cli.quiet, args.changed)?;
            if cli.strict && !diagnostics.is_empty() {
                1
            } else {
                0
            }
        }
        crate::cli::Commands::Fix(args) => {
            println!("{} Fix not yet implemented", "→".cyan());
            let diagnostics = run_check(&args.path, cli.json, cli.quiet, args.changed)?;
            if cli.strict && !diagnostics.is_empty() {
                1
            } else {
                0
            }
        }
        crate::cli::Commands::Explain(args) => {
            run_explain(&args.rule)?;
            0
        }
        crate::cli::Commands::Init => {
            run_init(&std::env::current_dir()?)?;
            0
        }
        crate::cli::Commands::Config => {
            run_config()?;
            0
        }
    };

    process::exit(exit_code);
}