use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn run(args: Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).unwrap().assert().stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(vec!["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(vec!["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(vec!["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(vec!["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
