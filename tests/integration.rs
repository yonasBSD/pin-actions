use std::{fs, process::Command};

use assert_cmd::{assert::OutputAssertExt, cargo::*};
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_help_flag() {
    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pin GitHub Actions"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_missing_workflows_directory() {
    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--workflows-dir")
        .arg("/nonexistent/path")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Workflows directory not found"));
}

#[test]
fn test_empty_workflows_directory() {
    let temp = TempDir::new().unwrap();
    let workflows_dir = temp.path().join("workflows");
    fs::create_dir(&workflows_dir).unwrap();

    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--workflows-dir")
        .arg(&workflows_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("No workflow files found"));
}

#[test]
fn test_dry_run_mode() {
    let temp = TempDir::new().unwrap();
    let workflows_dir = temp.path().join("workflows");
    fs::create_dir(&workflows_dir).unwrap();

    let workflow_content = r#"
name: Test
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
"#;

    fs::write(workflows_dir.join("test.yml"), workflow_content).unwrap();

    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--workflows-dir")
        .arg(&workflows_dir)
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dry run mode"));

    // Verify file wasn't modified
    let content = fs::read_to_string(workflows_dir.join("test.yml")).unwrap();
    assert_eq!(content, workflow_content);
}

#[test]
fn test_json_output() {
    let temp = TempDir::new().unwrap();
    let workflows_dir = temp.path().join("workflows");
    fs::create_dir(&workflows_dir).unwrap();

    let workflow_content = r#"
name: Test
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4
"#;

    fs::write(workflows_dir.join("test.yml"), workflow_content).unwrap();

    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--workflows-dir")
        .arg(&workflows_dir)
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("files_processed"));
}

#[test]
fn test_backup_creation() {
    let temp = TempDir::new().unwrap();
    let workflows_dir = temp.path().join("workflows");
    fs::create_dir(&workflows_dir).unwrap();

    let workflow_content = r#"
name: Test
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
"#;

    let workflow_path = workflows_dir.join("test.yml");
    fs::write(&workflow_path, workflow_content).unwrap();

    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--workflows-dir")
        .arg(&workflows_dir)
        .arg("--backup")
        .assert()
        .success();

    // Verify backup was created
    let backup_path = workflows_dir.join("test.yml.bak");
    assert!(backup_path.exists());

    let backup_content = fs::read_to_string(backup_path).unwrap();
    assert_eq!(backup_content, workflow_content);
}

#[test]
fn test_skip_local_actions() {
    let temp = TempDir::new().unwrap();
    let workflows_dir = temp.path().join("workflows");
    fs::create_dir(&workflows_dir).unwrap();

    let workflow_content = r#"
name: Test
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: ./local-action@v1
      - uses: actions/checkout@v4
"#;

    fs::write(workflows_dir.join("test.yml"), workflow_content).unwrap();

    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--workflows-dir")
        .arg(&workflows_dir)
        .assert()
        .success();

    // Verify local action wasn't touched
    let content = fs::read_to_string(workflows_dir.join("test.yml")).unwrap();
    assert!(content.contains("./local-action@v1"));
}

#[test]
fn test_already_pinned_actions() {
    let temp = TempDir::new().unwrap();
    let workflows_dir = temp.path().join("workflows");
    fs::create_dir(&workflows_dir).unwrap();

    let workflow_content = r#"
name: Test
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4
"#;

    fs::write(workflows_dir.join("test.yml"), workflow_content).unwrap();

    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--workflows-dir")
        .arg(&workflows_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Already pinned:   1"));
}

#[test]
fn test_verbose_output() {
    let temp = TempDir::new().unwrap();
    let workflows_dir = temp.path().join("workflows");
    fs::create_dir(&workflows_dir).unwrap();

    fs::write(workflows_dir.join("test.yml"), "name: Test\non: [push]").unwrap();

    let mut cmd = Command::new(cargo_bin!("pin-actions"));
    cmd.arg("--workflows-dir")
        .arg(&workflows_dir)
        .arg("--verbose")
        .assert()
        .success();
}
