//! # cargo-korox
//!
//! A lightweight, **Rust-native** developer quality toolkit for cleaner, safer, and more maintainable Rust code.
//! Runs locally with no cloud, no AI required.
//!
//! ## Overview
//!
//! `cargo-korox` unifies project-level diagnostics into one developer-oriented command:
//!
//! ```bash
//! cargo install cargo-korox
//! cargo korox check
//! ```
//!
//! Instead of running multiple commands:
//! ```bash
//! cargo check
//! cargo clippy
//! cargo fmt --check
//! ```
//!
//! ## Features
//!
//! - **Local-first** — Your code never leaves your machine
//! - **Zero dependencies** — Native Rust binary, no Python/Node/Docker/GPU
//! - **Rust-native** — Understands ownership, borrowing, lifetimes, async, Tokio, unsafe
//! - **Actionable diagnostics** — Explains *what*, *why*, *where*, and *how to fix*
//! - **AI optional** — Core works without LLMs; AI layer is opt-in
//! - **CI/CD ready** — `--strict` for CI, `--json` for editors/automation
//! - **Git-aware** — `--changed` scans only modified files
//!
//! ## Rules
//!
//! | Rule | Severity | Description |
//! |------|----------|-------------|
//! | `unnecessary-clone` | Warning | Detects avoidable `.clone()` calls |
//! | `suspicious-unwrap` | Warning | Detects `.unwrap()`/`.expect()` that can panic |
//! | `unused-variable` | Suggestion | Detects unused `let` bindings |
//! | `unnecessary-allocation` | Warning | Detects `Vec::new()`/`String::new()` filled in loops |
//! | `inefficient-loop` | Warning | Detects indexing, cloning in loops |
//! | `error-handling` | Error/Warning | Detects `panic!`, `todo!`, `unreachable!`, `.expect()` |
//! | `unnecessary-conversion` | Suggestion | Detects `to_owned()` on literals, `as` casts |
//! | `collection-in-loop` | Warning | Detects `Vec`/`HashMap`/`String` created inside loops |
//! | `inefficient-string-concat` | Warning | Detects multiple `+` concatenations |
//! | `redundant-clone` | Warning | Detects double clone, clone + to_string |
//!
//! ## Configuration
//!
//! Create `korox.toml` in your project root:
//!
//! ```toml
//! [tool.korox]
//! strict = false
//!
//! [tool.korox.rules]
//! unnecessary_clone = "warn"
//! suspicious_unwrap = "warn"
//! unused_variable = "suggestion"
//! ```
//!
//! Severity levels: `deny` • `warn` • `suggestion` • `allow`
//!
//! ## Usage as Library
//!
//! You can also use cargo-korox as a library in your own tools:
//!
//! ```rust
//! use cargo_korox::{run_check, Diagnostic};
//! use std::path::PathBuf;
//!
//! let diagnostics = run_check(&PathBuf::from("."), false, false, false)?;
//! for diag in diagnostics {
//!     println!("{}:{} {}", diag.file, diag.line, diag.message);
//! }
//! ```
//!
//! ## License
//!
//! Licensed under either of:
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
//! - MIT license ([LICENSE-MIT](LICENSE-MIT))

pub mod cli;
pub mod commands;
pub mod config;
pub mod diagnostics;
pub mod rules;
pub mod scanner;

pub use commands::run_check;
pub use diagnostics::Diagnostic;
pub use rules::{Rule, RuleContext};
pub use scanner::scan_project;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_scan_project() {
        let files = scan_project(&PathBuf::from(".")).unwrap();
        assert!(!files.is_empty());
    }
}