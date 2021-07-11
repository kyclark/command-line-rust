use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "calr";

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_4_2020() -> TestResult {
    run(&["-m", "4", "2020"], "tests/expected/4-2020.txt")
}

// --------------------------------------------------
#[test]
fn test_april_2020() -> TestResult {
    run(&["2020", "-m", "april"], "tests/expected/4-2020.txt")
}

// --------------------------------------------------
#[test]
fn test_2020() -> TestResult {
    run(&["2020"], "tests/expected/2020.txt")
}
