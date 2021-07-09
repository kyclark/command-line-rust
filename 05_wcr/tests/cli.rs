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
    Command::cargo_bin("wcr")?.arg("blargh").assert().stderr(
        predicate::str::is_match("blargh: .* [(]os error 2[)]").unwrap(),
    );
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_chars_and_bytes() -> TestResult {
    Command::cargo_bin("wcr")?
        .args(&["-m", "-c"])
        .assert()
        .stderr(predicate::str::contains(
            "The argument '--bytes' cannot be used with '--chars'",
        ));
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    let expected = fs::read_to_string("tests/expected/empty.txt.out")?;
    Command::cargo_bin("wcr")?
        .arg(EMPTY)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.txt.out")?;
    Command::cargo_bin("wcr")?
        .arg(FOX)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.txt.c.out")?;
    Command::cargo_bin("wcr")?
        .args(&["--bytes", FOX])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_chars() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.txt.m.out")?;
    Command::cargo_bin("wcr")?
        .args(&["--chars", FOX])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_words() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.txt.w.out")?;
    Command::cargo_bin("wcr")?
        .args(&["--words", FOX])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.txt.l.out")?;
    Command::cargo_bin("wcr")?
        .args(&["--lines", FOX])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_words_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.txt.wc.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-w", "-c", FOX])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_words_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.txt.wl.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-w", "-l", FOX])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_bytes_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.txt.cl.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-l", "-c", FOX])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal() -> TestResult {
    let expected = fs::read_to_string("tests/expected/atlamal.txt.out")?;
    Command::cargo_bin("wcr")?
        .arg(ATLAMAL)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/expected/atlamal.txt.c.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-c", ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_words() -> TestResult {
    let expected = fs::read_to_string("tests/expected/atlamal.txt.w.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-w", ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/atlamal.txt.l.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-l", ATLAMAL])
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_words_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/expected/atlamal.txt.wc.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-w", "-c", ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_words_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/atlamal.txt.wl.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-w", "-l", ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_bytes_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/atlamal.txt.cl.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-l", "-c", ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn atlamal_stdin() -> TestResult {
    let input = fs::read_to_string(ATLAMAL)?;
    let expected =
        fs::read_to_string("tests/expected/atlamal.txt.stdin.out")?;
    Command::cargo_bin("wcr")?
        .write_stdin(input)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all() -> TestResult {
    let expected = fs::read_to_string("tests/expected/all.out").ok().unwrap();
    Command::cargo_bin("wcr")?
        .args(&[EMPTY, FOX, ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_lines() -> TestResult {
    let expected =
        fs::read_to_string("tests/expected/all.l.out").ok().unwrap();
    Command::cargo_bin("wcr")?
        .args(&["-l", EMPTY, FOX, ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_words() -> TestResult {
    let expected =
        fs::read_to_string("tests/expected/all.w.out").ok().unwrap();
    Command::cargo_bin("wcr")?
        .args(&["-w", EMPTY, FOX, ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_bytes() -> TestResult {
    let expected =
        fs::read_to_string("tests/expected/all.c.out").ok().unwrap();
    Command::cargo_bin("wcr")?
        .args(&["-c", EMPTY, FOX, ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_words_bytes() -> TestResult {
    let expected = fs::read_to_string("tests/expected/all.wc.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-cw", EMPTY, FOX, ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_words_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/all.wl.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-wl", EMPTY, FOX, ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all_bytes_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/all.cl.out")?;
    Command::cargo_bin("wcr")?
        .args(&["-cl", EMPTY, FOX, ATLAMAL])
        .assert()
        .stdout(expected);
    Ok(())
}
