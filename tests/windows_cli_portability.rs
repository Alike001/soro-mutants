//! Windows-native CLI smoke checks: no bash invocation or shell quoting required.
#![cfg(windows)]

use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn copy_tree(from: &Path, to: &Path) {
    for entry in walkdir::WalkDir::new(from) {
        let entry = entry.expect("fixture entry");
        let relative = entry.path().strip_prefix(from).expect("fixture prefix");
        let destination = to.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&destination).expect("create fixture directory");
        } else if entry.file_type().is_file() {
            fs::copy(entry.path(), &destination).expect("copy fixture file");
        }
    }
}

#[test]
fn list_json_is_stable_for_windows_paths_containing_spaces() {
    let temporary = std::env::temp_dir().join(format!(
        "soro mutants windows path {} {}",
        std::process::id(),
        std::thread::current().name().unwrap_or("smoke")
    ));
    // Ensure a repeatable clean fixture even after a prior interrupted test.
    if temporary.exists() {
        fs::remove_dir_all(&temporary).expect("remove stale test fixture");
    }
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/weak-auth");
    copy_tree(&source, &temporary);
    let binary = env!("CARGO_BIN_EXE_cargo-soro-mutants");
    let run = || {
        Command::new(binary)
            .arg("list")
            .arg(&temporary)
            .arg("--json")
            .output()
            .expect("launch CLI directly without a shell")
    };
    let first = run();
    let second = run();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(
        second.status.success(),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert_eq!(
        first.stdout, second.stdout,
        "JSON output must be deterministic"
    );
    let _: Value = serde_json::from_slice(&first.stdout).expect("valid JSON report");

    let count = Command::new(binary)
        .arg("list")
        .arg(&temporary)
        .arg("--count-only")
        .output()
        .expect("count-only CLI");
    assert!(
        count.status.success(),
        "{}",
        String::from_utf8_lossy(&count.stderr)
    );
    let count_text = String::from_utf8(count.stdout).expect("UTF-8 count");
    assert!(
        count_text.trim().parse::<usize>().is_ok(),
        "invalid count: {count_text}"
    );
    fs::remove_dir_all(&temporary).expect("remove test fixture");
}
