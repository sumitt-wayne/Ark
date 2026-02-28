use std::process::Command;

pub struct GitResult {
    pub success: bool,
    pub output: String,
}

fn run(args: &[&str]) -> GitResult {
    let result = Command::new("git")
        .args(args)
        .output();

    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let combined = format!("{}{}", stdout, stderr).trim().to_string();

            GitResult {
                success: output.status.success(),
                output: combined,
            }
        }
        Err(e) => GitResult {
            success: false,
            output: format!("Failed to run git: {}", e),
        },
    }
}

pub fn is_git_repo() -> bool {
    run(&["rev-parse", "--git-dir"]).success
}

pub fn init() -> GitResult {
    run(&["init"])
}

pub fn add_all() -> GitResult {
    run(&["add", "."])
}

pub fn commit(message: &str) -> GitResult {
    run(&["commit", "-m", message])
}

pub fn push() -> GitResult {
    run(&["push"])
}

pub fn pull() -> GitResult {
    run(&["pull"])
}

pub fn get_remote() -> Option<String> {
    let result = run(&["remote", "get-url", "origin"]);
    if result.success {
        Some(result.output)
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn set_remote(url: &str) -> GitResult {
    run(&["remote", "add", "origin", url])
}
