use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn run_idid(args: &[&str], workdir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_idid"))
        .args(args)
        .current_dir(workdir)
        .output()
        .expect("failed to run idid")
}

#[test]
fn install_creates_log_file_and_records_source() {
    let temp = tempdir().expect("failed to create tempdir");

    let output = run_idid(&["install", "wezterm", "flatpak"], temp.path());

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Logged install: wezterm from flatpak"));

    let log = fs::read_to_string(temp.path().join("data/log.csv")).expect("failed to read log");
    let mut lines = log.lines();

    assert_eq!(lines.next(), Some("timestamp,action,package,source"));

    let entry = lines.next().expect("missing log entry");
    assert!(entry.contains(",install,wezterm,flatpak"));
}

#[test]
fn remove_defaults_source_to_unknown() {
    let temp = tempdir().expect("failed to create tempdir");

    let output = run_idid(&["remove", "ghostty"], temp.path());

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Logged removal: ghostty"));

    let log = fs::read_to_string(temp.path().join("data/log.csv")).expect("failed to read log");
    let entry = log.lines().nth(1).expect("missing log entry");
    assert!(entry.contains(",remove,ghostty,Unknown"));
}

#[test]
fn history_prints_existing_log_contents() {
    let temp = tempdir().expect("failed to create tempdir");
    let data_dir = temp.path().join("data");
    fs::create_dir_all(&data_dir).expect("failed to create data dir");

    let expected = "timestamp,action,package,source\n2026-03-07 19:26:29,install,wezterm,flatpak\n";
    fs::write(data_dir.join("log.csv"), expected).expect("failed to seed log");

    let output = run_idid(&["history"], temp.path());

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), format!("{expected}\n"));
}
