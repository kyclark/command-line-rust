use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

const PRG: &str = "wcr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const ATLAMAL: &str = "tests/inputs/atlamal.txt";

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn dies_chars_and_bytes() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(["-m", "-c"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "the argument '--chars' cannot be used with '--bytes'",
        ));
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?.args(args).output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> Result<()> {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn fox() -> Result<()> {
    run(&[FOX], "tests/expected/fox.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_bytes() -> Result<()> {
    run(&["--bytes", FOX], "tests/expected/fox.txt.c.out")
}

// --------------------------------------------------
#[test]
fn fox_chars() -> Result<()> {
    run(&["--chars", FOX], "tests/expected/fox.txt.m.out")
}

// --------------------------------------------------
#[test]
fn fox_words() -> Result<()> {
    run(&["--words", FOX], "tests/expected/fox.txt.w.out")
}

// --------------------------------------------------
#[test]
fn fox_lines() -> Result<()> {
    run(&["--lines", FOX], "tests/expected/fox.txt.l.out")
}

// --------------------------------------------------
#[test]
fn fox_words_bytes() -> Result<()> {
    run(&["-w", "-c", FOX], "tests/expected/fox.txt.wc.out")
}

// --------------------------------------------------
#[test]
fn fox_words_lines() -> Result<()> {
    run(&["-w", "-l", FOX], "tests/expected/fox.txt.wl.out")
}

// --------------------------------------------------
#[test]
fn fox_bytes_lines() -> Result<()> {
    run(&["-l", "-c", FOX], "tests/expected/fox.txt.cl.out")
}

// --------------------------------------------------
#[test]
fn atlamal() -> Result<()> {
    run(&[ATLAMAL], "tests/expected/atlamal.txt.out")
}

// --------------------------------------------------
#[test]
fn atlamal_bytes() -> Result<()> {
    run(&["-c", ATLAMAL], "tests/expected/atlamal.txt.c.out")
}

// --------------------------------------------------
#[test]
fn atlamal_words() -> Result<()> {
    run(&["-w", ATLAMAL], "tests/expected/atlamal.txt.w.out")
}

// --------------------------------------------------
#[test]
fn atlamal_lines() -> Result<()> {
    run(&["-l", ATLAMAL], "tests/expected/atlamal.txt.l.out")
}

// --------------------------------------------------
#[test]
fn atlamal_words_bytes() -> Result<()> {
    run(&["-w", "-c", ATLAMAL], "tests/expected/atlamal.txt.wc.out")
}

// --------------------------------------------------
#[test]
fn atlamal_words_lines() -> Result<()> {
    run(&["-w", "-l", ATLAMAL], "tests/expected/atlamal.txt.wl.out")
}

// --------------------------------------------------
#[test]
fn atlamal_bytes_lines() -> Result<()> {
    run(&["-l", "-c", ATLAMAL], "tests/expected/atlamal.txt.cl.out")
}

// --------------------------------------------------
#[test]
fn atlamal_stdin() -> Result<()> {
    let input = fs::read_to_string(ATLAMAL)?;
    let expected =
        fs::read_to_string("tests/expected/atlamal.txt.stdin.out")?;

    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_all() -> Result<()> {
    run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")
}

// --------------------------------------------------
#[test]
fn test_all_lines() -> Result<()> {
    run(&["-l", EMPTY, FOX, ATLAMAL], "tests/expected/all.l.out")
}

// --------------------------------------------------
#[test]
fn test_all_words() -> Result<()> {
    run(&["-w", EMPTY, FOX, ATLAMAL], "tests/expected/all.w.out")
}

// --------------------------------------------------
#[test]
fn test_all_bytes() -> Result<()> {
    run(&["-c", EMPTY, FOX, ATLAMAL], "tests/expected/all.c.out")
}

// --------------------------------------------------
#[test]
fn test_all_words_bytes() -> Result<()> {
    run(&["-cw", EMPTY, FOX, ATLAMAL], "tests/expected/all.wc.out")
}

// --------------------------------------------------
#[test]
fn test_all_words_lines() -> Result<()> {
    run(&["-wl", EMPTY, FOX, ATLAMAL], "tests/expected/all.wl.out")
}

// --------------------------------------------------
#[test]
fn test_all_bytes_lines() -> Result<()> {
    run(&["-cl", EMPTY, FOX, ATLAMAL], "tests/expected/all.cl.out")
}
