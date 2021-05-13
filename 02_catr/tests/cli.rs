use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

static PROGRAM: &str = "catr";
static EMPTY: &str = "tests/inputs/empty.txt";
static FOX: &str = "tests/inputs/fox.txt";
static SPIDERS: &str = "tests/inputs/spiders.txt";
static BUSTLE: &str = "tests/inputs/the-bustle.txt";

// --------------------------------------------------
#[test]
fn usage() -> TestResult {
    let mut cmd = Command::cargo_bin(PROGRAM)?;
    for flag in vec!["-h", "--help"] {
        cmd.arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

// --------------------------------------------------
#[test]
fn bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin(PROGRAM)?;
    cmd.arg("foo")
        .assert()
        .failure()
        .stderr("\"foo\" is not a valid file.\n");

    Ok(())
}

// --------------------------------------------------
fn run(args: Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin(PROGRAM)?;
    cmd.args(args).unwrap().assert().stdout(expected);

    Ok(())
}

// --------------------------------------------------
fn run_stdin(
    stdin_file: &str,
    args: Vec<&str>,
    expected_file: &str,
) -> TestResult {
    let input = fs::read_to_string(stdin_file)?;
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin(PROGRAM)?;
    cmd.args(args)
        .with_stdin()
        .buffer(input)
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, vec!["-"], "tests/inputs/the-bustle.txt.stdin.out")
}

// --------------------------------------------------
#[test]
fn bustle_stdin_n() -> TestResult {
    run_stdin(
        BUSTLE,
        vec!["-n", "-"],
        "tests/inputs/the-bustle.txt.n.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_stdin_b() -> TestResult {
    run_stdin(
        BUSTLE,
        vec!["-b", "-"],
        "tests/inputs/the-bustle.txt.b.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(vec![EMPTY], "tests/inputs/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n() -> TestResult {
    run(vec!["-n", EMPTY], "tests/inputs/empty.txt.n.out")
}

// --------------------------------------------------
#[test]
fn empty_b() -> TestResult {
    run(vec!["-b", EMPTY], "tests/inputs/empty.txt.b.out")
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    run(vec![FOX], "tests/inputs/fox.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_n() -> TestResult {
    run(vec!["-n", FOX], "tests/inputs/fox.txt.n.out")
}

// --------------------------------------------------
#[test]
fn fox_b() -> TestResult {
    run(vec!["-b", FOX], "tests/inputs/fox.txt.b.out")
}

// --------------------------------------------------
#[test]
fn spiders() -> TestResult {
    run(vec![SPIDERS], "tests/inputs/spiders.txt.out")
}

// --------------------------------------------------
#[test]
fn spiders_n() -> TestResult {
    run(vec!["--number", SPIDERS], "tests/inputs/spiders.txt.n.out")
}

// --------------------------------------------------
#[test]
fn spiders_b() -> TestResult {
    run(
        vec!["--number-nonblank", SPIDERS],
        "tests/inputs/spiders.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle() -> TestResult {
    run(vec![BUSTLE], "tests/inputs/the-bustle.txt.out")
}

// --------------------------------------------------
#[test]
fn bustle_n() -> TestResult {
    run(vec!["-n", BUSTLE], "tests/inputs/the-bustle.txt.n.out")
}

// --------------------------------------------------
#[test]
fn bustle_b() -> TestResult {
    run(vec!["-b", BUSTLE], "tests/inputs/the-bustle.txt.b.out")
}

// --------------------------------------------------
#[test]
fn all() -> TestResult {
    run(vec![FOX, SPIDERS, BUSTLE], "tests/inputs/all.out")
}

// --------------------------------------------------
#[test]
fn all_n() -> TestResult {
    run(vec![FOX, SPIDERS, BUSTLE, "-n"], "tests/inputs/all.n.out")
}

// --------------------------------------------------
#[test]
fn all_b() -> TestResult {
    run(vec![FOX, SPIDERS, BUSTLE, "-b"], "tests/inputs/all.b.out")
}
