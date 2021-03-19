use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn echo() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(&["hello", "there"])
        .unwrap()
        .assert()
        .stdout("hello there\n");

    Ok(())
}

// --------------------------------------------------
#[test]
fn newline() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(&["hello", "there", "-n"])
        .unwrap()
        .assert()
        .stdout("hello there");

    Ok(())
}
