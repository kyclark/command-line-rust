use assert_cmd::prelude::*;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

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
