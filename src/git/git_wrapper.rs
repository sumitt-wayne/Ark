use std::process::Command;

pub struct GitResult {
    pub success: bool,
    pub output: String,
}

fn run(args: &[&str]) -> GitResult {
    let result = Command::new("git").args(args).output();

    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let combined = format!("{}{}", stdout, stderr).trim().to_string();
            GitResult { success: output.status.success(), output: combined }
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
    let result = run(&["init", "-b", "main"]);
    if result.success { result } else {
        let r = run(&["init"]);
        run(&["checkout", "-b", "main"]);
        r
    }
}

pub fn add_all() -> GitResult {
    run(&["add", "."])
}

pub fn commit(message: &str) -> GitResult {
    run(&["commit", "-m", message])
}

pub fn push() -> GitResult {
    // Try normal push first
    let result = run(&["push", "origin", "main"]);
    if result.success { return result; }

    // Set upstream and push
    let result2 = run(&["push", "--set-upstream", "origin", "main"]);
    if result2.success { return result2; }

    result2
}

pub fn pull() -> GitResult {
    // Try pull with rebase to avoid merge conflicts
    let result = run(&["pull", "origin", "main", "--allow-unrelated-histories", "--no-rebase"]);
    if result.success { return result; }

    // If empty repo, just continue
    if result.output.contains("couldn't find remote ref") 
    || result.output.contains("no tracking information")
    || result.output.contains("does not have") {
        return GitResult { success: true, output: "No remote history yet.".to_string() };
    }

    result
}

pub fn get_remote() -> Option<String> {
    let result = run(&["remote", "get-url", "origin"]);
    if result.success { Some(result.output) } else { None }
}

#[allow(dead_code)]
pub fn set_remote(url: &str) -> GitResult {
    run(&["remote", "add", "origin", url])
}

pub fn clone(url: &str, folder: &str) -> GitResult {
    run(&["clone", url, folder])
}
