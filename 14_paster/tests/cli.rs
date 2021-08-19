use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::{error::Error, fs};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "paster";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONE: &str = "tests/inputs/one.txt";
const TWO: &str = "tests/inputs/two.txt";
const THREE: &str = "tests/inputs/three.txt";
const NAMES: &str = "tests/inputs/names.txt";
const DEADLIFTS: &str = "tests/inputs/deadlifts.txt";

// --------------------------------------------------
fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error [23][)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

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
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.out")
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(&[ONE], "tests/expected/one.out")
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&[TWO], "tests/expected/two.out")
}

// --------------------------------------------------
#[test]
fn empty_two() -> TestResult {
    run(&[EMPTY, TWO], "tests/expected/empty_two.out")
}

// --------------------------------------------------
#[test]
fn two_empty_delim_comma() -> TestResult {
    run(
        &["-d", ",", TWO, EMPTY],
        "tests/expected/two_empty_d_comma.out",
    )
}

// --------------------------------------------------
#[test]
fn one_two() -> TestResult {
    run(&[ONE, TWO], "tests/expected/one_two.out")
}

// --------------------------------------------------
#[test]
fn two_three() -> TestResult {
    run(&[TWO, THREE], "tests/expected/two_three.out")
}

// --------------------------------------------------
#[test]
fn empty_one_two_three() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE],
        "tests/expected/empty_one_two_three.out",
    )
}

// --------------------------------------------------
#[test]
fn names() -> TestResult {
    run(&[NAMES], "tests/expected/names.out")
}

// --------------------------------------------------
#[test]
fn names_deadlifts_delim_colon() -> TestResult {
    run(
        &["-d", ":", NAMES, DEADLIFTS],
        "tests/expected/names_deadlifts_d_colon.out",
    )
}

// --------------------------------------------------
#[test]
fn names_serial_multiple_delims() -> TestResult {
    run(
        &["-s", "-d", "\t\n", NAMES],
        "tests/expected/names_serial_delims.out",
    )
}

// --------------------------------------------------
#[test]
fn stdin() -> TestResult {
    let input = fs::read_to_string(THREE)?;
    let expected = fs::read_to_string("tests/expected/two_three.out")?;
    Command::cargo_bin(PRG)?
        .args([TWO, "-"])
        .write_stdin(input)
        .assert()
        .stdout(expected);
    Ok(())
}
