use crate::scanner::scan_project;
use crate::rules::{all_rules, RuleContext};
use crate::diagnostics::print_diagnostics;
use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

pub fn run_check(path: &PathBuf, json: bool, quiet: bool) -> Result<()> {
    let files = scan_project(path)?;
    let rules = all_rules();

    let mut all_diagnostics = Vec::new();

    for file in &files {
        let content = std::fs::read_to_string(file)?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        let ctx = RuleContext {
            file_path: file.display().to_string(),
            content: content.clone(),
            lines,
        };

        for rule in &rules {
            let diagnostics = rule.check(&ctx);
            all_diagnostics.extend(diagnostics);
        }
    }

    print_diagnostics(&all_diagnostics, json, quiet);
    Ok(())
}

pub fn run_fix(path: &PathBuf, json: bool, quiet: bool) -> Result<()> {
    println!("{} Fix not yet implemented", "→".cyan());
    run_check(path, json, quiet)
}

pub fn run_explain(rule_name: &str) -> Result<()> {
    match rule_name {
        "unnecessary-clone" => {
            println!("{}", "unnecessary-clone".bold().underline());
            println!("\nDetects .clone() calls that may be avoidable by using references.");
            println!("\nExample:");
            println!("  let data2 = data.clone();");
            println!("  process(data2);");
            println!("\nBetter:");
            println!("  process(&data);");
        }
        "suspicious-unwrap" => {
            println!("{}", "suspicious-unwrap".bold().underline());
            println!("\nDetects .unwrap() and .expect() calls that can panic.");
            println!("\nExample:");
            println!("  let value = result.unwrap();");
            println!("\nBetter:");
            println!("  let value = result?;  // propagate error");
            println!("  // or");
            println!("  let value = result.unwrap_or(default);  // handle gracefully");
        }
        "unused-variable" => {
            println!("{}", "unused-variable".bold().underline());
            println!("\nDetects let bindings that are never used.");
            println!("\nExample:");
            println!("  let unused = 100;");
            println!("\nFix:");
            println!("  let _unused = 100;  // prefix with underscore");
        }
        _ => {
            println!("Unknown rule: {}", rule_name);
            println!("Available: unnecessary-clone, suspicious-unwrap, unused-variable");
        }
    }
    Ok(())
}