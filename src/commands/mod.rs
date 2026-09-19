use crate::scanner::{scan_project, get_all_changed_files};
use crate::rules::{all_rules, RuleContext};
use crate::diagnostics::{print_diagnostics, Diagnostic};
use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

pub fn run_check(path: &PathBuf, json: bool, quiet: bool, changed: bool) -> Result<Vec<Diagnostic>> {
    let files = if changed {
        get_all_changed_files(path)?
    } else {
        scan_project(path)?
    };
    
    if files.is_empty() && changed {
        println!("{} No changed Rust files found", "ℹ".cyan());
        print_diagnostics(&[], json, quiet);
        return Ok(Vec::new());
    }

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
    Ok(all_diagnostics)
}

pub fn run_fix(path: &PathBuf, json: bool, quiet: bool) -> Result<()> {
    println!("{} Fix not yet implemented", "→".cyan());
    run_check(path, json, quiet, false)?;
    Ok(())
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
        "unnecessary-allocation" => {
            println!("{}", "unnecessary-allocation".bold().underline());
            println!("\nDetects allocations that can be avoided with collect() or format!().");
            println!("\nExample:");
            println!("  let mut v = Vec::new();");
            println!("  for i in 0..10 {{ v.push(i); }}");
            println!("\nBetter:");
            println!("  let v: Vec<_> = (0..10).collect();");
        }
        "inefficient-loop" => {
            println!("{}", "inefficient-loop".bold().underline());
            println!("\nDetects inefficient loop patterns like cloning in loops or indexing.");
            println!("\nExample:");
            println!("  for i in 0..items.len() {{");
            println!("      let item = items[i].clone();");
            println!("  }}");
            println!("\nBetter:");
            println!("  for item in &items {{ ... }}");
        }
        "error-handling" => {
            println!("{}", "error-handling".bold().underline());
            println!("\nDetects panic!, todo!, unreachable! and expect() that should be proper errors.");
            println!("\nExample:");
            println!("  panic!(\"something went wrong\");");
            println!("  todo!();");
            println!("\nBetter:");
            println!("  return Err(MyError::SomethingWrong);");
        }
        "unnecessary-conversion" => {
            println!("{}", "unnecessary-conversion".bold().underline());
            println!("\nDetects unnecessary type conversions like to_owned() on literals or as casts.");
            println!("\nExample:");
            println!("  let s = \"hello\".to_owned();");
            println!("  let x = 5 as u32;");
            println!("\nBetter:");
            println!("  let s = String::from(\"hello\");");
            println!("  let x: u32 = 5;");
        }
        "collection-in-loop" => {
            println!("{}", "collection-in-loop".bold().underline());
            println!("\nDetects collections (Vec, HashMap, String) created inside loops.");
            println!("\nExample:");
            println!("  for item in items {{");
            println!("      let mut map = HashMap::new();");
            println!("      map.insert(...);");
            println!("  }}");
            println!("\nBetter:");
            println!("  let mut map = HashMap::new();");
            println!("  for item in items {{");
            println!("      map.clear();");
            println!("      map.insert(...);");
            println!("  }}");
        }
        "inefficient-string-concat" => {
            println!("{}", "inefficient-string-concat".bold().underline());
            println!("\nDetects inefficient string concatenation with + operator.");
            println!("\nExample:");
            println!("  let s = a + &b + &c + &d;");
            println!("\nBetter:");
            println!("  let s = format!(\"{{}}{{}}{{}}{{}}\", a, b, c, d);");
        }
        "redundant-clone" => {
            println!("{}", "redundant-clone".bold().underline());
            println!("\nDetects redundant clones like double clone or clone + to_string.");
            println!("\nExample:");
            println!("  let s = data.clone().clone();");
            println!("  let s = data.clone().to_string();");
            println!("\nBetter:");
            println!("  let s = data.clone();");
            println!("  let s = data.to_string();");
        }
        _ => {
            println!("Unknown rule: {}", rule_name);
            println!("Available: unnecessary-clone, suspicious-unwrap, unused-variable, unnecessary-allocation, inefficient-loop, error-handling, unnecessary-conversion, collection-in-loop, inefficient-string-concat, redundant-clone");
        }
    }
    Ok(())
}