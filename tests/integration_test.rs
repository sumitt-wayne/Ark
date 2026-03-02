use std::fs;
use std::process::Command;
use std::path::Path;

fn ark_cmd(dir: &str, args: &[&str]) -> std::process::Output {
    // Use the compiled binary directly
    let binary = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/debug/ark");

    Command::new(binary)
        .args(args)
        .current_dir(dir)
        .output()
        .expect("Failed to run ark binary. Run 'cargo build' first.")
}

fn setup(test_name: &str) -> String {
    let dir = format!("/tmp/ark_test_{}", test_name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn cleanup(dir: &str) {
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_start_creates_ark_directory() {
    let dir = setup("start");

    let output = ark_cmd(&dir, &["start"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("initialized successfully"));
    assert!(Path::new(&format!("{}/.ark", dir)).exists());
    assert!(Path::new(&format!("{}/.ark/config.json", dir)).exists());
    assert!(Path::new(&format!("{}/.ark/commits", dir)).exists());
    assert!(Path::new(&format!("{}/.ark/snapshots", dir)).exists());

    cleanup(&dir);
}

#[test]
fn test_double_start_gives_error() {
    let dir = setup("double_start");

    ark_cmd(&dir, &["start"]);
    let output = ark_cmd(&dir, &["start"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("already exists"));

    cleanup(&dir);
}

#[test]
fn test_check_without_init_gives_error() {
    let dir = setup("check_no_init");

    let output = ark_cmd(&dir, &["check"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("Not an Ark repository"));

    cleanup(&dir);
}

#[test]
fn test_save_and_history() {
    let dir = setup("save_history");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/test.txt", dir), "hello ark").unwrap();

    let save_output = ark_cmd(&dir, &["save", "test commit"]);
    let stdout = String::from_utf8_lossy(&save_output.stdout);
    assert!(stdout.contains("saved successfully"));

    let history_output = ark_cmd(&dir, &["history"]);
    let history_stdout = String::from_utf8_lossy(&history_output.stdout);
    assert!(history_stdout.contains("test commit"));

    cleanup(&dir);
}

#[test]
fn test_save_no_changes() {
    let dir = setup("save_no_changes");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/test.txt", dir), "hello").unwrap();
    ark_cmd(&dir, &["save", "first save"]);

    let output = ark_cmd(&dir, &["save"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Nothing to save"));

    cleanup(&dir);
}

#[test]
fn test_undo_removes_latest_commit() {
    let dir = setup("undo");

    ark_cmd(&dir, &["start"]);

    fs::write(format!("{}/test.txt", dir), "version 1").unwrap();
    ark_cmd(&dir, &["save", "first"]);

    fs::write(format!("{}/test.txt", dir), "version 2").unwrap();
    ark_cmd(&dir, &["save", "second"]);

    let undo_output = ark_cmd(&dir, &["undo"]);
    let stdout = String::from_utf8_lossy(&undo_output.stdout);
    assert!(stdout.contains("Undo successful"));

    let history_output = ark_cmd(&dir, &["history"]);
    let history_stdout = String::from_utf8_lossy(&history_output.stdout);
    assert!(history_stdout.contains("1 total saves"));

    cleanup(&dir);
}

#[test]
fn test_scan_detects_secrets() {
    let dir = setup("scan_secrets");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/secret.env", dir), "API_KEY=sk-1234567890abcdef").unwrap();

    let output = ark_cmd(&dir, &["scan"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("potential issue"));

    cleanup(&dir);
}

#[test]
fn test_scan_clean_project() {
    let dir = setup("scan_clean");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["scan"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No secrets"));

    cleanup(&dir);
}

#[test]
fn test_branch_create_and_list() {
    let dir = setup("branch_create");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["branch", "new", "feature"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Branch created"));

    let list_output = ark_cmd(&dir, &["branch", "list"]);
    let list_stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(list_stdout.contains("main"));
    assert!(list_stdout.contains("feature"));

    cleanup(&dir);
}

#[test]
fn test_branch_switch() {
    let dir = setup("branch_switch");

    ark_cmd(&dir, &["start"]);
    ark_cmd(&dir, &["branch", "new", "feature"]);

    let output = ark_cmd(&dir, &["branch", "go", "feature"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Switched to branch"));

    cleanup(&dir);
}

#[test]
fn test_branch_delete() {
    let dir = setup("branch_delete");

    ark_cmd(&dir, &["start"]);
    ark_cmd(&dir, &["branch", "new", "temp"]);

    let output = ark_cmd(&dir, &["branch", "delete", "temp"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Branch deleted"));

    cleanup(&dir);
}

#[test]
fn test_cannot_delete_main() {
    let dir = setup("branch_no_delete_main");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["branch", "delete", "main"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Cannot delete"));

    cleanup(&dir);
}

#[test]
fn test_remote_show_no_remote() {
    let dir = setup("remote_no_remote");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["remote", "show"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No remote configured"));

    cleanup(&dir);
}

#[test]
fn test_sync_without_remote() {
    let dir = setup("sync_no_remote");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["sync"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No remote configured"));

    cleanup(&dir);
}

#[test]
fn test_ai_commit_without_setup() {
    let dir = setup("ai_no_setup");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["ai", "commit"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("AI not configured"));

    cleanup(&dir);
}

#[test]
fn test_ai_review_without_setup() {
    let dir = setup("ai_review_no_setup");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["ai", "review"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("AI not configured"));

    cleanup(&dir);
}

#[test]
fn test_diff_no_changes() {
    let dir = setup("diff_no_changes");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/test.txt", dir), "hello").unwrap();
    ark_cmd(&dir, &["save", "first save"]);

    let output = ark_cmd(&dir, &["diff"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No changes detected"));

    cleanup(&dir);
}

#[test]
fn test_diff_with_changes() {
    let dir = setup("diff_with_changes");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/test.txt", dir), "hello").unwrap();
    ark_cmd(&dir, &["save", "first save"]);

    fs::write(format!("{}/new_file.txt", dir), "new content").unwrap();

    let output = ark_cmd(&dir, &["diff"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("new_file.txt"));

    cleanup(&dir);
}

#[test]
fn test_merge_branch() {
    let dir = setup("merge_branch");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/main.txt", dir), "main content").unwrap();
    ark_cmd(&dir, &["save", "main first save"]);

    ark_cmd(&dir, &["branch", "new", "feature"]);
    ark_cmd(&dir, &["branch", "go", "feature"]);
    fs::write(format!("{}/feature.txt", dir), "feature content").unwrap();
    ark_cmd(&dir, &["save", "feature save"]);

    ark_cmd(&dir, &["branch", "go", "main"]);
    let output = ark_cmd(&dir, &["merge", "feature"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Merge successful"));

    cleanup(&dir);
}

#[test]
fn test_merge_nonexistent_branch() {
    let dir = setup("merge_nonexistent");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["merge", "nonexistent"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found"));

    cleanup(&dir);
}

#[test]
fn test_tag_create_and_list() {
    let dir = setup("tag_create");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/test.txt", dir), "hello").unwrap();
    ark_cmd(&dir, &["save", "first save"]);
    ark_cmd(&dir, &["tag", "new", "v1.0", "First release"]);

    let output = ark_cmd(&dir, &["tag", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("v1.0"));

    cleanup(&dir);
}

#[test]
fn test_tag_delete() {
    let dir = setup("tag_delete");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/test.txt", dir), "hello").unwrap();
    ark_cmd(&dir, &["save", "first save"]);
    ark_cmd(&dir, &["tag", "new", "v1.0", "First release"]);
    ark_cmd(&dir, &["tag", "delete", "v1.0"]);

    let output = ark_cmd(&dir, &["tag", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No tags found"));

    cleanup(&dir);
}

#[test]
fn test_stash_save_and_list() {
    let dir = setup("stash_save");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/test.txt", dir), "hello").unwrap();
    ark_cmd(&dir, &["save", "first save"]);
    fs::write(format!("{}/test.txt", dir), "changed").unwrap();
    ark_cmd(&dir, &["stash", "save", "WIP changes"]);

    let output = ark_cmd(&dir, &["stash", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("WIP changes"));

    cleanup(&dir);
}

#[test]
fn test_stash_pop() {
    let dir = setup("stash_pop");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/test.txt", dir), "hello").unwrap();
    ark_cmd(&dir, &["save", "first save"]);
    fs::write(format!("{}/test.txt", dir), "changed").unwrap();
    ark_cmd(&dir, &["stash", "save", "WIP"]);
    ark_cmd(&dir, &["stash", "pop"]);

    let output = ark_cmd(&dir, &["stash", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No stashes found"));

    cleanup(&dir);
}

#[test]
fn test_restore_file() {
    let dir = setup("restore_file");

    ark_cmd(&dir, &["start"]);
    fs::write(format!("{}/test.txt", dir), "hello").unwrap();
    ark_cmd(&dir, &["save", "first save"]);

    let output = ark_cmd(&dir, &["restore", "test.txt"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Restored"));

    cleanup(&dir);
}

#[test]
fn test_branch_rename() {
    let dir = setup("branch_rename");

    ark_cmd(&dir, &["start"]);
    ark_cmd(&dir, &["branch", "new", "old-name"]);
    ark_cmd(&dir, &["branch", "rename", "old-name", "new-name"]);

    let output = ark_cmd(&dir, &["branch", "list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("new-name"));
    assert!(!stdout.contains("old-name"));

    cleanup(&dir);
}

#[test]
fn test_cannot_rename_main() {
    let dir = setup("cannot_rename_main");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["branch", "rename", "main", "other"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Cannot rename"));

    cleanup(&dir);
}

#[test]
fn test_ai_diff_without_setup() {
    let dir = setup("ai_diff_no_setup");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["ai", "diff"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("AI not configured"));

    cleanup(&dir);
}

#[test]
fn test_ai_suggest_without_setup() {
    let dir = setup("ai_suggest_no_setup");

    ark_cmd(&dir, &["start"]);

    let output = ark_cmd(&dir, &["ai", "suggest"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("AI not configured"));

    cleanup(&dir);
}
