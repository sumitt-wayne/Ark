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
