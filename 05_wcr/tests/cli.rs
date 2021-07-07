use assert_cmd::prelude::*;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;
static EMPTY: &str = "tests/inputs/empty.txt";
static FOX: &str = "tests/inputs/fox.txt";
static ATLAMAL: &str = "tests/inputs/atlamal.txt";

// --------------------------------------------------
#[test]
fn dies_bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.arg("blargh").assert().stderr(
        predicate::str::is_match("blargh: .* [(]os error 2[)]").unwrap(),
    );
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_chars_and_bytes() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-m", "-c"])
        .assert()
        .stderr(predicate::str::contains(
            "The argument '--bytes' cannot be used with '--chars'",
        ));
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/empty.txt.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.arg(EMPTY).unwrap().assert().stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/fox.txt.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.arg(FOX).unwrap().assert().stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/fox.txt.c.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["--bytes", FOX])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_chars() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/fox.txt.m.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["--chars", FOX])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_words() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/fox.txt.w.out")?;
    cmd.args(&["--words", FOX])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/fox.txt.l.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["--lines", FOX])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_words_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/fox.txt.wc.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-c", FOX])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_words_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/fox.txt.wl.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-l", FOX])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_bytes_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/fox.txt.cl.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-l", "-c", FOX])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/atlamal.txt.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.arg(ATLAMAL).unwrap().assert().stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/atlamal.txt.c.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-c", ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_words() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let expected = fs::read_to_string("tests/inputs/atlamal.txt.w.out")?;
    cmd.args(&["-w", ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/atlamal.txt.l.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-l", ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_words_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/atlamal.txt.wc.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-c", ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_words_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/atlamal.txt.wl.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", "-l", ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_bytes_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/atlamal.txt.cl.out")?;
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-l", "-c", ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_stdin() -> TestResult {
    let mut cmd = Command::cargo_bin("wcr")?;
    let input = fs::read_to_string(ATLAMAL)?;
    let expected = fs::read_to_string("tests/inputs/atlamal.txt.stdin.out")?;
    cmd.write_stdin(input).assert().stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/all.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&[EMPTY, FOX, ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_lines() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/all.l.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-l", EMPTY, FOX, ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_words() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/all.w.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-w", EMPTY, FOX, ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/all.c.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-c", EMPTY, FOX, ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_words_bytes() -> TestResult {
    let expected =
        fs::read_to_string("tests/inputs/all.wc.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-cw", EMPTY, FOX, ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_words_lines() -> TestResult {
    let expected =
        fs::read_to_string("tests/inputs/all.wl.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-wl", EMPTY, FOX, ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_bytes_lines() -> TestResult {
    let expected =
        fs::read_to_string("tests/inputs/all.cl.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("wcr")?;
    cmd.args(&["-cl", EMPTY, FOX, ATLAMAL])
        .unwrap()
        .assert()
        .stdout(expected);
    Ok(())
}
