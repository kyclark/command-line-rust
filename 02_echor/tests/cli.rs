use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// --------------------------------------------------
#[test]
fn dies_no_args() -> Result<()> {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn hello1() -> Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

// --------------------------------------------------
#[test]
fn hello2() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

// --------------------------------------------------
#[test]
fn hello1_no_newline() -> Result<()> {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

// --------------------------------------------------
#[test]
fn hello2_no_newline() -> Result<()> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
