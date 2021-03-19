use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("change")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_not_an_int() -> TestResult {
    let mut cmd = Command::cargo_bin("change")?;
    let err = "\"foo\" is not an integer";
    cmd.arg("foo")
        .assert()
        .failure()
        .stderr(predicate::str::contains(err));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_int() -> TestResult {
    let mut cmd = Command::cargo_bin("change")?;
    let err = "Amount \"0\" must be between 1 and 100";
    cmd.arg("0")
        .assert()
        .failure()
        .stderr(predicate::str::contains(err));

    Ok(())
}

// --------------------------------------------------
fn run(arg: &str, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).ok().unwrap();
    let mut cmd = Command::cargo_bin("change")?;
    cmd.arg(arg).unwrap().assert().stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run("1", "tests/expected/1.out")
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run("2", "tests/expected/2.out")
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run("3", "tests/expected/3.out")
}

// --------------------------------------------------
#[test]
fn four() -> TestResult {
    run("4", "tests/expected/4.out")
}

// --------------------------------------------------
#[test]
fn five() -> TestResult {
    run("5", "tests/expected/5.out")
}

// --------------------------------------------------
#[test]
fn six() -> TestResult {
    run("6", "tests/expected/6.out")
}

// --------------------------------------------------
#[test]
fn seven() -> TestResult {
    run("7", "tests/expected/7.out")
}

// --------------------------------------------------
#[test]
fn eight() -> TestResult {
    run("8", "tests/expected/8.out")
}

// --------------------------------------------------
#[test]
fn nine() -> TestResult {
    run("9", "tests/expected/9.out")
}

// --------------------------------------------------
#[test]
fn ten() -> TestResult {
    run("10", "tests/expected/10.out")
}

// --------------------------------------------------
#[test]
fn eleven() -> TestResult {
    run("11", "tests/expected/11.out")
}

// --------------------------------------------------
#[test]
fn twelve() -> TestResult {
    run("12", "tests/expected/12.out")
}

// --------------------------------------------------
#[test]
fn thirteen() -> TestResult {
    run("13", "tests/expected/13.out")
}

// --------------------------------------------------
#[test]
fn fourteen() -> TestResult {
    run("14", "tests/expected/14.out")
}

// --------------------------------------------------
#[test]
fn fifteen() -> TestResult {
    run("15", "tests/expected/15.out")
}

// --------------------------------------------------
#[test]
fn sixteen() -> TestResult {
    run("16", "tests/expected/16.out")
}

// --------------------------------------------------
#[test]
fn seventeen() -> TestResult {
    run("17", "tests/expected/17.out")
}

// --------------------------------------------------
#[test]
fn eighteen() -> TestResult {
    run("18", "tests/expected/18.out")
}

// --------------------------------------------------
#[test]
fn nineteen() -> TestResult {
    run("19", "tests/expected/19.out")
}

// --------------------------------------------------
#[test]
fn twenty() -> TestResult {
    run("20", "tests/expected/20.out")
}

// --------------------------------------------------
#[test]
fn twentyone() -> TestResult {
    run("21", "tests/expected/21.out")
}

// --------------------------------------------------
#[test]
fn twentytwo() -> TestResult {
    run("22", "tests/expected/22.out")
}

// --------------------------------------------------
#[test]
fn twentythree() -> TestResult {
    run("23", "tests/expected/23.out")
}

// --------------------------------------------------
#[test]
fn twentyfour() -> TestResult {
    run("24", "tests/expected/24.out")
}

// --------------------------------------------------
#[test]
fn twentyfive() -> TestResult {
    run("25", "tests/expected/25.out")
}

// --------------------------------------------------
#[test]
fn twentysix() -> TestResult {
    run("26", "tests/expected/26.out")
}

// --------------------------------------------------
#[test]
fn twentyseven() -> TestResult {
    run("27", "tests/expected/27.out")
}
