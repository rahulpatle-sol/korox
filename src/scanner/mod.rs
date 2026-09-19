use std::path::PathBuf;
use walkdir::WalkDir;
use anyhow::Result;

pub fn scan_project(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut rust_files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    rust_files.push(entry.path().to_path_buf());
                }
            }
        }
    }
    Ok(rust_files)
}

pub fn get_changed_files(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let output = std::process::Command::new("git")
        .args(["diff", "--name-only", "HEAD"])
        .current_dir(path)
        .output()?;
    
    if !output.status.success() {
        return Ok(Vec::new());
    }
    
    let files = String::from_utf8_lossy(&output.stdout);
    let mut rust_files = Vec::new();
    
    for line in files.lines() {
        let file_path = path.join(line);
        if file_path.extension().map_or(false, |ext| ext == "rs") && file_path.exists() {
            rust_files.push(file_path);
        }
    }
    
    Ok(rust_files)
}

pub fn get_staged_files(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let output = std::process::Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(path)
        .output()?;
    
    if !output.status.success() {
        return Ok(Vec::new());
    }
    
    let files = String::from_utf8_lossy(&output.stdout);
    let mut rust_files = Vec::new();
    
    for line in files.lines() {
        let file_path = path.join(line);
        if file_path.extension().map_or(false, |ext| ext == "rs") && file_path.exists() {
            rust_files.push(file_path);
        }
    }
    
    Ok(rust_files)
}

pub fn get_all_changed_files(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = get_changed_files(path)?;
    let staged = get_staged_files(path)?;
    files.extend(staged);
    files.sort();
    files.dedup();
    Ok(files)
}