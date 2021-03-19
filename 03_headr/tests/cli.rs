use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("headr")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin("headr")?;
    cmd.arg("foo")
        .assert()
        .stderr(predicate::str::contains("foo: No such file or directory"));

    Ok(())
}

// --------------------------------------------------
fn run(args: &Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).ok().unwrap();
    let mut cmd = Command::cargo_bin("headr")?;
    cmd.args(args).unwrap().assert().stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt"],
        "tests/inputs/empty.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_n2() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-n", "2"],
        "tests/inputs/empty.txt.n2.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_n4() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-n", "4"],
        "tests/inputs/empty.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(&vec!["tests/inputs/one.txt"], "tests/inputs/one.txt.out")
}

// --------------------------------------------------
#[test]
fn one_n2() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-n", "2"],
        "tests/inputs/one.txt.n2.out",
    )
}

// --------------------------------------------------
#[test]
fn one_n4() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-n", "4"],
        "tests/inputs/one.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&vec!["tests/inputs/two.txt"], "tests/inputs/two.txt.out")
}

// --------------------------------------------------
#[test]
fn two_n2() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-n", "2"],
        "tests/inputs/two.txt.n2.out",
    )
}

// --------------------------------------------------
#[test]
fn two_n4() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-n", "4"],
        "tests/inputs/two.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt"],
        "tests/inputs/three.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn three_n2() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-n", "2"],
        "tests/inputs/three.txt.n2.out",
    )
}

// --------------------------------------------------
#[test]
fn three_n4() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-n", "4"],
        "tests/inputs/three.txt.n4.out",
    )
}
