use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.arg("foo")
        .assert()
        .failure()
        .stderr("\"foo\" is not a valid file.\n");

    Ok(())
}

// --------------------------------------------------
fn run(args: &Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).ok().unwrap();
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(args).unwrap().assert().stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn foo() -> TestResult {
    run(&vec!["tests/inputs/foo.txt"], "tests/inputs/foo.txt")
}

// --------------------------------------------------
#[test]
fn foo_n() -> TestResult {
    run(
        &vec!["-n", "tests/inputs/foo.txt"],
        "tests/inputs/foo.txt.n.out",
    )
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    run(&vec!["tests/inputs/fox.txt"], "tests/inputs/fox.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_n() -> TestResult {
    run(
        &vec!["-n", "tests/inputs/fox.txt"],
        "tests/inputs/fox.txt.n.out",
    )
}

// --------------------------------------------------
#[test]
fn all() -> TestResult {
    run(
        &vec!["tests/inputs/foo.txt", "tests/inputs/fox.txt"],
        "tests/inputs/all.out",
    )
}

// --------------------------------------------------
#[test]
fn all_n() -> TestResult {
    run(
        &vec!["tests/inputs/foo.txt", "tests/inputs/fox.txt", "-n"],
        "tests/inputs/all.n.out",
    )
}
