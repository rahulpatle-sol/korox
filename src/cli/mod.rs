use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cargo-korox", version, about = "Rust-native developer quality toolkit")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub json: bool,

    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[arg(long, global = true)]
    pub strict: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Check(CheckArgs),
    Fix(FixArgs),
    Explain(ExplainArgs),
    Watch(WatchArgs),
    Init,
    Config,
}

#[derive(Parser)]
pub struct CheckArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long, short = 'c')]
    pub changed: bool,
}

#[derive(Parser)]
pub struct FixArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long, short = 'c')]
    pub changed: bool,
}

#[derive(Parser)]
pub struct WatchArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long, short = 'c')]
    pub changed: bool,

    #[arg(long, default_value = "500")]
    pub debounce: u64,
}

#[derive(Parser)]
pub struct ExplainArgs {
    pub rule: String,
}