use serde::{Deserialize, Serialize};
use colored::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Diagnostic {
    pub rule: String,
    pub severity: Severity,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Suggestion,
}

pub fn print_diagnostics(diagnostics: &[Diagnostic], json: bool, quiet: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(diagnostics).unwrap());
        return;
    }

    if diagnostics.is_empty() {
        if !quiet {
            println!("{}", "✓ No issues found".green());
        }
        return;
    }

    let mut errors = 0;
    let mut warnings = 0;
    let mut suggestions = 0;

    for diag in diagnostics {
        match diag.severity {
            Severity::Error => errors += 1,
            Severity::Warning => warnings += 1,
            Severity::Suggestion => suggestions += 1,
        }
    }

    println!("\n{}", "Diagnostics".bold().underline());
    
    for diag in diagnostics {
        let severity_str = match diag.severity {
            Severity::Error => "✗".red().bold(),
            Severity::Warning => "⚠".yellow().bold(),
            Severity::Suggestion => "💡".cyan().bold(),
        };

        println!("{} {}:{}:{} {}", 
            severity_str,
            diag.file,
            diag.line,
            diag.column,
            diag.message
        );

        if let Some(suggestion) = &diag.suggestion {
            println!("   {} {}", "→".cyan(), suggestion);
        }
    }

    println!("\n{}", "Summary".bold().underline());
    println!("  Errors:      {}", errors.to_string().red());
    println!("  Warnings:    {}", warnings.to_string().yellow());
    println!("  Suggestions: {}", suggestions.to_string().cyan());
}