use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
//use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/empty.txt.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.arg("tests/inputs/empty.txt")
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/one.txt.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.arg("tests/inputs/one.txt")
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn one_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/one.txt.c.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-c", "tests/inputs/one.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn one_words() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/one.txt.w.out")
        .ok()
        .unwrap();
    cmd.args(&["-w", "tests/inputs/one.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn one_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/one.txt.l.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-l", "tests/inputs/one.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn one_words_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/one.txt.wc.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-c", "tests/inputs/one.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn one_words_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/one.txt.wl.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-l", "tests/inputs/one.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn one_bytes_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/one.txt.cl.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-l", "-c", "tests/inputs/one.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/one.txt.out").ok().unwrap();
    cmd.arg("tests/inputs/one.txt")
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn two_bytes() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/two.txt.c.out")
        .ok()
        .unwrap();
    cmd.args(&["-c", "tests/inputs/two.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn two_words() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/two.txt.w.out")
        .ok()
        .unwrap();
    cmd.args(&["-w", "tests/inputs/two.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn two_lines() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/two.txt.l.out")
        .ok()
        .unwrap();
    cmd.args(&["-l", "tests/inputs/two.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn two_words_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/two.txt.wc.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-c", "tests/inputs/two.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn two_words_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/two.txt.wl.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-l", "tests/inputs/two.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn two_bytes_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/two.txt.cl.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-l", "-c", "tests/inputs/two.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/one.txt.out").ok().unwrap();
    cmd.arg("tests/inputs/one.txt")
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn three_bytes() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/three.txt.c.out")
        .ok()
        .unwrap();
    cmd.args(&["-c", "tests/inputs/three.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn three_words() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/three.txt.w.out")
        .ok()
        .unwrap();
    cmd.args(&["-w", "tests/inputs/three.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn three_lines() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/three.txt.l.out")
        .ok()
        .unwrap();
    cmd.args(&["-l", "tests/inputs/three.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn three_words_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/three.txt.wc.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-c", "tests/inputs/three.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn three_words_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/three.txt.wl.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-l", "tests/inputs/three.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn three_bytes_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/three.txt.cl.out")
        .ok()
        .unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-l", "-c", "tests/inputs/three.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/all.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&[
        "tests/inputs/empty.txt",
        "tests/inputs/one.txt",
        "tests/inputs/two.txt",
        "tests/inputs/three.txt",
    ])
    .unwrap()
    .assert()
    .stdout(expected);

    Ok(())
}
