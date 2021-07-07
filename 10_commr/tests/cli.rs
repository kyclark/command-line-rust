use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

const PRG: &str = "commr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FILE1: &str = "tests/inputs/file1.txt";
const FILE2: &str = "tests/inputs/file2.txt";

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_both_stdin() -> TestResult {
    let expected = "Both input files cannot be STDIN (\"-\")";
    Command::cargo_bin(PRG)?
        .args(&["-", "-"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty_empty() -> TestResult {
    let expected = fs::read_to_string("tests/expected/empty_empty.out")?;

    Command::cargo_bin(PRG)?
        .args(&[EMPTY, EMPTY])
        .assert()
        .stdout(expected);

    Ok(())
}
// --------------------------------------------------
#[test]
fn file1_file1() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file1.out")?;

    Command::cargo_bin(PRG)?
        .args(&[FILE1, FILE1])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_empty() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_empty.out")?;

    Command::cargo_bin(PRG)?
        .args(&[FILE1, EMPTY])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty_file2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/empty_file2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[EMPTY, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_1() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.1.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-1", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.2.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-2", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_3() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.3.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-3", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_1_2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.12.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-12", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_2_3() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.23.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-23", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_13() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.13.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-13", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_123() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.123.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-123", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
// insensitive
// --------------------------------------------------
#[test]
fn file1_file2_1_i() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.1.i.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-1", "-i", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_2_i() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.2.i.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-2", "-i", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_3_i() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.3.i.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-3", "-i", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_1_2_i() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.12.i.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-12", "-i", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_2_3_i() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.23.i.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-23", "-i", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_13_i() -> TestResult {
    let expected = fs::read_to_string("tests/expected/file1_file2.13.i.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-13", "-i", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn file1_file2_123_i() -> TestResult {
    let expected =
        fs::read_to_string("tests/expected/file1_file2.123.i.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-123", "-i", FILE1, FILE2])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn stdin_file1() -> TestResult {
    let input = fs::read_to_string("tests/inputs/file1.txt")?;
    let expected =
        fs::read_to_string("tests/expected/file1_file2.123.i.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-123", "-i", "-", FILE2])
        .write_stdin(input)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn stdin_file2() -> TestResult {
    let input = fs::read_to_string("tests/inputs/file2.txt")?;
    let expected =
        fs::read_to_string("tests/expected/file1_file2.123.i.out")?;

    Command::cargo_bin(PRG)?
        .args(&["-123", "-i", FILE1, "-"])
        .write_stdin(input)
        .assert()
        .stdout(expected);
    Ok(())
}
