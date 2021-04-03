use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
fn run(args: &Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).ok().unwrap();
    let mut cmd = Command::cargo_bin("calr")?;
    cmd.args(args).unwrap().assert().stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn test_4_2020() -> TestResult {
    run(&vec!["-m", "4", "2020"], "tests/expected/4-2020.txt")
}

// --------------------------------------------------
#[test]
fn test_april_2020() -> TestResult {
    run(&vec!["2020", "-m", "april"], "tests/expected/4-2020.txt")
}

// --------------------------------------------------
#[test]
fn test_2020() -> TestResult {
    run(&vec!["2020"], "tests/expected/2020.txt")
}
