use crate::diagnostics::{Diagnostic, Severity};

pub struct RuleContext {
    pub file_path: String,
    pub content: String,
    pub lines: Vec<String>,
}

pub trait Rule {
    fn name(&self) -> &'static str;
    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic>;
}

pub struct UnnecessaryCloneRule;

impl Rule for UnnecessaryCloneRule {
    fn name(&self) -> &'static str {
        "unnecessary-clone"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines.iter().enumerate() {
            if line.contains(".clone()") && !line.trim_start().starts_with("//") {
                let column = line.find(".clone()").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "This clone may be avoidable".to_string(),
                    suggestion: Some("Consider passing a reference instead".to_string()),
                });
            }
        }
        diagnostics
    }
}

pub struct SuspiciousUnwrapRule;

impl Rule for SuspiciousUnwrapRule {
    fn name(&self) -> &'static str {
        "suspicious-unwrap"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines.iter().enumerate() {
            if (line.contains(".unwrap()") || line.contains(".expect(")) && !line.trim_start().starts_with("//") {
                let keyword = if line.contains(".unwrap()") { ".unwrap()" } else { ".expect(" };
                let column = line.find(keyword).unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: format!("This {} can panic when the Result/Option is Err/None", keyword),
                    suggestion: Some("Consider propagating the error with `?` or handling it explicitly".to_string()),
                });
            }
        }
        diagnostics
    }
}

pub struct UnusedVariableRule;

impl Rule for UnusedVariableRule {
    fn name(&self) -> &'static str {
        "unused-variable"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            if (trimmed.starts_with("let ") || trimmed.starts_with("let mut ")) && 
               !trimmed.starts_with("//") &&
               trimmed.contains(" = ") &&
               !trimmed.contains("_") {
                let after_let = if trimmed.starts_with("let mut ") {
                    &trimmed[8..]
                } else {
                    &trimmed[4..]
                };
                let var_name = after_let.split(" = ").next()
                    .and_then(|s| s.split(':').next())
                    .map(|s| s.trim());
                
                if let Some(name) = var_name {
                    if !name.starts_with('_') {
                        let usage_count = ctx.content.matches(name).count();
                        if usage_count == 1 {
                            let column = line.find(name).unwrap_or(0) + 1;
                            diagnostics.push(Diagnostic {
                                rule: self.name().to_string(),
                                severity: Severity::Suggestion,
                                file: ctx.file_path.clone(),
                                line: i + 1,
                                column,
                                message: format!("Unused variable: {}", name),
                                suggestion: Some("Prefix with `_` to silence this warning".to_string()),
                            });
                        }
                    }
                }
            }
        }
        diagnostics
    }
}

pub struct UnnecessaryAllocationRule;

impl Rule for UnnecessaryAllocationRule {
    fn name(&self) -> &'static str {
        "unnecessary-allocation"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("vec![]") || trimmed.starts_with("Vec::new()") {
                if let Some(next_line) = ctx.lines.get(i + 1) {
                    if next_line.trim().starts_with("for ") && next_line.contains(".push(") {
                        let column = line.find("vec![]").or_else(|| line.find("Vec::new()")).unwrap_or(0) + 1;
                        diagnostics.push(Diagnostic {
                            rule: self.name().to_string(),
                            severity: Severity::Warning,
                            file: ctx.file_path.clone(),
                            line: i + 1,
                            column,
                            message: "Collection created and then filled in loop - consider using `collect()`".to_string(),
                            suggestion: Some("Use iterator.collect() instead of creating empty vec and pushing".to_string()),
                        });
                    }
                }
            }

            if trimmed.contains("String::new()") && ctx.content.contains(".push_str(") {
                let column = line.find("String::new()").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "String created and then built up - consider using format! or collect".to_string(),
                    suggestion: Some("Use format!() or iterator.collect::<String>()".to_string()),
                });
            }

            if trimmed.contains("to_string()") && line.contains("format!") {
                let column = line.find("to_string()").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Suggestion,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "Unnecessary to_string() on format! result".to_string(),
                    suggestion: Some("format! already returns String".to_string()),
                });
            }
        }
        diagnostics
    }
}

pub struct InefficientLoopRule;

impl Rule for InefficientLoopRule {
    fn name(&self) -> &'static str {
        "inefficient-loop"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        for (i, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("for ") && trimmed.contains(".len()") {
                let column = line.find(".len()").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "Iterating over range with .len() - consider direct iteration".to_string(),
                    suggestion: Some("Use `for item in &collection` instead of `for i in 0..collection.len()`".to_string()),
                });
            }

            if trimmed.contains(".clone()") && (trimmed.contains("for ") || trimmed.contains("while ")) {
                let column = line.find(".clone()").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "Cloning inside loop - consider cloning outside".to_string(),
                    suggestion: Some("Move .clone() outside the loop".to_string()),
                });
            }

            if trimmed.starts_with("for ") && trimmed.contains("..=") && trimmed.contains(".collect()") {
                let column = line.find("..=").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Suggestion,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "Range with collect - consider using iterator methods".to_string(),
                    suggestion: Some("Use (0..n).map(...).collect() or similar".to_string()),
                });
            }
        }
        
        for (i, line) in ctx.lines.iter().enumerate() {
            if line.trim().contains("String::new()") {
                for j in i+1..ctx.lines.len().min(i+10) {
                    if ctx.lines[j].contains(".push_str(") || ctx.lines[j].contains(".push(") {
                        let column = line.find("String::new()").unwrap_or(0) + 1;
                        diagnostics.push(Diagnostic {
                            rule: self.name().to_string(),
                            severity: Severity::Warning,
                            file: ctx.file_path.clone(),
                            line: i + 1,
                            column,
                            message: "String allocation in loop - consider reusing or using format!".to_string(),
                            suggestion: Some("Build strings outside loops or use format!".to_string()),
                        });
                        break;
                    }
                }
            }
        }
        
        diagnostics
    }
}

pub struct ErrorHandlingRule;

impl Rule for ErrorHandlingRule {
    fn name(&self) -> &'static str {
        "error-handling"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        for (i, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("panic!") && !trimmed.starts_with("//") {
                let column = line.find("panic!").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Error,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "panic!() usage - consider returning Result instead".to_string(),
                    suggestion: Some("Return Result<T, E> and use ? operator".to_string()),
                });
            }
            
            if trimmed.starts_with("todo!") && !trimmed.starts_with("//") {
                let column = line.find("todo!").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "todo!() found - will panic at runtime".to_string(),
                    suggestion: Some("Implement the functionality or return Result with proper error".to_string()),
                });
            }
            
            if trimmed.starts_with("unreachable!") && !trimmed.starts_with("//") {
                let column = line.find("unreachable!").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "unreachable!() found - verify this is truly unreachable".to_string(),
                    suggestion: Some("Consider using proper error handling instead".to_string()),
                });
            }
            
            if trimmed.contains(".expect(") && !trimmed.starts_with("//") {
                let column = line.find(".expect(").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: ".expect() with custom message - still panics on Err/None".to_string(),
                    suggestion: Some("Use ? to propagate or handle with match/if let".to_string()),
                });
            }
        }
        diagnostics
    }
}

pub struct UnnecessaryConversionRule;

impl Rule for UnnecessaryConversionRule {
    fn name(&self) -> &'static str {
        "unnecessary-conversion"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        for (i, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.contains(".to_owned()") && !trimmed.starts_with("//") {
                if line.contains("String::from") || line.contains("&str") {
                    let column = line.find(".to_owned()").unwrap_or(0) + 1;
                    diagnostics.push(Diagnostic {
                        rule: self.name().to_string(),
                        severity: Severity::Suggestion,
                        file: ctx.file_path.clone(),
                        line: i + 1,
                        column,
                        message: "to_owned() on string literal - use String::from or to_string()".to_string(),
                        suggestion: Some("Use String::from(\"...\") or \"...\".to_string()".to_string()),
                    });
                }
            }
            
            if trimmed.contains(".into()") && !trimmed.starts_with("//") {
                let column = line.find(".into()").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Suggestion,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: ".into() conversion - explicit type may be clearer".to_string(),
                    suggestion: Some("Consider explicit conversion for clarity".to_string()),
                });
            }
            
            if trimmed.contains("as ") && !trimmed.starts_with("//") {
                if line.contains(" as u") || line.contains(" as i") || line.contains(" as f") {
                    let column = line.find(" as ").unwrap_or(0) + 1;
                    diagnostics.push(Diagnostic {
                        rule: self.name().to_string(),
                        severity: Severity::Suggestion,
                        file: ctx.file_path.clone(),
                        line: i + 1,
                        column,
                        message: "Numeric cast with `as` - may truncate or lose precision".to_string(),
                        suggestion: Some("Use TryFrom/TryInto for checked conversions".to_string()),
                    });
                }
            }
        }
        diagnostics
    }
}

pub struct CollectionInLoopRule;

impl Rule for CollectionInLoopRule {
    fn name(&self) -> &'static str {
        "collection-in-loop"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut in_loop = false;
        
        for (i, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("for ") || trimmed.starts_with("while ") || trimmed.starts_with("loop ") {
                in_loop = true;
            }
            
            if in_loop && (trimmed.starts_with("}") || trimmed.starts_with("} else")) {
                let brace_count = line.chars().filter(|c| *c == '}').count();
                let open_count = line.chars().filter(|c| *c == '{').count();
                if brace_count > open_count {
                    in_loop = false;
                }
            }
            
            if in_loop {
                if (trimmed.contains("Vec::new()") || trimmed.contains("vec![]") || 
                    trimmed.contains("HashMap::new()") || trimmed.contains("HashSet::new()") ||
                    trimmed.contains("String::new()")) && !trimmed.starts_with("//") {
                    let column = line.find("Vec::new()")
                        .or_else(|| line.find("vec![]"))
                        .or_else(|| line.find("HashMap::new()"))
                        .or_else(|| line.find("HashSet::new()"))
                        .or_else(|| line.find("String::new()"))
                        .unwrap_or(0) + 1;
                    diagnostics.push(Diagnostic {
                        rule: self.name().to_string(),
                        severity: Severity::Warning,
                        file: ctx.file_path.clone(),
                        line: i + 1,
                        column,
                        message: "Collection created inside loop - allocate outside and reuse".to_string(),
                        suggestion: Some("Move collection creation outside loop and clear() each iteration".to_string()),
                    });
                }
            }
        }
        diagnostics
    }
}

pub struct InefficientStringConcatRule;

impl Rule for InefficientStringConcatRule {
    fn name(&self) -> &'static str {
        "inefficient-string-concat"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        for (i, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.contains(" + ") && trimmed.contains("\"") && !trimmed.starts_with("//") {
                if line.matches('+').count() > 1 {
                    let column = line.find('+').unwrap_or(0) + 1;
                    diagnostics.push(Diagnostic {
                        rule: self.name().to_string(),
                        severity: Severity::Warning,
                        file: ctx.file_path.clone(),
                        line: i + 1,
                        column,
                        message: "Multiple string concatenations with + - inefficient".to_string(),
                        suggestion: Some("Use format!() or String::push_str() for multiple concatenations".to_string()),
                    });
                }
            }
            
            if trimmed.contains("format!(") && line.contains(" + ") {
                let column = line.find("format!(").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Suggestion,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "format! with + concatenation - put all in format!".to_string(),
                    suggestion: Some("Use format!(\"{}{}{}\", a, b, c) instead of format!(\"{}\", a) + b".to_string()),
                });
            }
        }
        diagnostics
    }
}

pub struct RedundantCloneRule;

impl Rule for RedundantCloneRule {
    fn name(&self) -> &'static str {
        "redundant-clone"
    }

    fn check(&self, ctx: &RuleContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        for (i, line) in ctx.lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.contains(".clone()") && !trimmed.starts_with("//") {
                let column = line.find(".clone()").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "Double clone - one is redundant".to_string(),
                    suggestion: Some("Remove one .clone() call".to_string()),
                });
            }
            
            if trimmed.contains(".clone()") && (trimmed.contains(".to_string()") || trimmed.contains("String::from")) {
                let column = line.find(".clone()").unwrap_or(0) + 1;
                diagnostics.push(Diagnostic {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    file: ctx.file_path.clone(),
                    line: i + 1,
                    column,
                    message: "Clone followed by to_string/from - redundant".to_string(),
                    suggestion: Some("Use to_string() or String::from directly".to_string()),
                });
            }
        }
        diagnostics
    }
}

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(UnnecessaryCloneRule),
        Box::new(SuspiciousUnwrapRule),
        Box::new(UnusedVariableRule),
        Box::new(UnnecessaryAllocationRule),
        Box::new(InefficientLoopRule),
        Box::new(ErrorHandlingRule),
        Box::new(UnnecessaryConversionRule),
        Box::new(CollectionInLoopRule),
        Box::new(InefficientStringConcatRule),
        Box::new(RedundantCloneRule),
    ]
}