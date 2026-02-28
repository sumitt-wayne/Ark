use std::fs;
use std::path::Path;

pub struct ScanResult {
    pub file: String,
    pub line_number: usize,
    pub line: String,
    pub issue: String,
}

const PATTERNS: &[(&str, &str)] = &[
    ("password", "Possible password found"),
    ("passwd", "Possible password found"),
    ("secret", "Possible secret found"),
    ("api_key", "Possible API key found"),
    ("apikey", "Possible API key found"),
    ("access_token", "Possible access token found"),
    ("auth_token", "Possible auth token found"),
    ("private_key", "Possible private key found"),
    ("aws_access_key_id", "AWS Access Key found"),
    ("aws_secret_access_key", "AWS Secret Key found"),
    ("database_url", "Database URL found"),
    ("db_password", "Database password found"),
    ("bearer", "Bearer token found"),
    ("BEGIN RSA PRIVATE KEY", "RSA Private Key found"),
    ("BEGIN OPENSSH PRIVATE KEY", "SSH Private Key found"),
];

// Only scan files that commonly contain secrets
const SCANNABLE_EXTENSIONS: &[&str] = &[
    "env", "json", "yaml", "yml", "toml", "ini",
    "cfg", "conf", "config", "properties", "xml",
    "txt", "md",
];

pub fn scan_files() -> Vec<ScanResult> {
    let mut results = Vec::new();
    scan_dir(Path::new("."), &mut results);
    results
}

fn scan_dir(dir: &Path, results: &mut Vec<ScanResult>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();

        if should_ignore(&path_str) {
            continue;
        }

        if path.is_dir() {
            scan_dir(&path, results);
        } else if path.is_file() && is_scannable(&path) {
            scan_file(&path, results);
        }
    }
}

fn is_scannable(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => SCANNABLE_EXTENSIONS.contains(&ext.to_string_lossy().as_ref()),
        // Also scan files with no extension like .env, Makefile
        None => {
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
            name.starts_with(".env") || name == "makefile"
        }
    }
}

fn scan_file(path: &Path, results: &mut Vec<ScanResult>) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    for (line_number, line) in content.lines().enumerate() {
        let line_lower = line.to_lowercase();

        for (pattern, issue) in PATTERNS {
            if line_lower.contains(&pattern.to_lowercase()) {
                results.push(ScanResult {
                    file: path.to_string_lossy().to_string(),
                    line_number: line_number + 1,
                    line: line.trim().to_string(),
                    issue: issue.to_string(),
                });
                break;
            }
        }
    }
}

fn should_ignore(path: &str) -> bool {
    let ignore_list = [".ark", "target", ".git"];
    ignore_list.iter().any(|i| path.contains(i))
}
