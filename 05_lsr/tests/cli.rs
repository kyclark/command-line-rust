use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    cmd.arg("foo")
        .assert()
        .stderr(predicate::str::contains("foo: No such file or directory"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn no_args() -> TestResult {
    // Uses current directory by default
    let mut cmd = Command::cargo_bin("lsr")?;
    cmd.assert().success();
    Ok(())
}

// --------------------------------------------------
fn run(args: &Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).ok().unwrap();
    let mut cmd = Command::cargo_bin("lsr")?;
    cmd.args(args).unwrap().assert().stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt"],
        "tests/expected/empty.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_long() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-l"],
        "tests/expected/empty.txt.l.out",
    )
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(&vec!["tests/inputs/one.txt"], "tests/expected/one.txt.out")
}

// --------------------------------------------------
#[test]
fn one_long() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-l"],
        "tests/expected/one.txt.l.out",
    )
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&vec!["tests/inputs/two.txt"], "tests/expected/two.txt.out")
}

// --------------------------------------------------
#[test]
fn two_long() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-l"],
        "tests/expected/two.txt.l.out",
    )
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt"],
        "tests/expected/three.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn three_long() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-l"],
        "tests/expected/three.txt.l.out",
    )
}

// --------------------------------------------------
#[test]
fn dir() -> TestResult {
    run(&vec!["tests/inputs"], "tests/expected/dir.out")
}

// --------------------------------------------------
#[test]
fn dir_long() -> TestResult {
    run(&vec!["tests/inputs", "-l"], "tests/expected/dir.l.out")
}

// --------------------------------------------------
#[test]
fn all() -> TestResult {
    run(
        &vec![
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/two.txt",
            "tests/inputs/three.txt",
        ],
        "tests/expected/all.out",
    )
}

// --------------------------------------------------
#[test]
fn all_long() -> TestResult {
    run(
        &vec![
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/two.txt",
            "tests/inputs/three.txt",
            "-l",
        ],
        "tests/expected/all.l.out",
    )
}
