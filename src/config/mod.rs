use std::path::PathBuf;
use anyhow::Result;
use colored::Colorize;

pub fn run_init(path: &PathBuf) -> Result<()> {
    let config = r#"[tool.korox]
strict = false

[tool.korox.rules]
unnecessary_clone = "warn"
suspicious_unwrap = "warn"
unused_variable = "suggestion"
"#;
    std::fs::write(path.join("korox.toml"), config)?;
    println!("{} Created korox.toml", "✓".green());
    Ok(())
}

pub fn run_config() -> Result<()> {
    println!("{}", "Korox Configuration".bold().underline());
    println!("\nConfig file: korox.toml (project root)");
    println!("\nAvailable rules:");
    println!("  unnecessary_clone    - Detect avoidable .clone() calls");
    println!("  suspicious_unwrap    - Detect .unwrap()/.expect() that may panic");
    println!("  unused_variable      - Detect unused let bindings");
    println!("\nSeverity levels: deny, warn, suggestion, allow");
    Ok(())
}