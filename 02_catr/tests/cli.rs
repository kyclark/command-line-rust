use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

// --------------------------------------------------
#[test]
fn usage() -> TestResult {
    for flag in vec!["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
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
fn bad_file() -> TestResult {
    let bad = gen_bad_file();
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .failure()
        .stderr(format!("\"{}\" is not a valid file.\n", &bad));

    Ok(())
}

// --------------------------------------------------
fn run(args: Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .stdout(expected);

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
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, vec!["-"], "tests/expected/the-bustle.txt.stdin.out")
}

// --------------------------------------------------
#[test]
fn bustle_stdin_n() -> TestResult {
    run_stdin(
        BUSTLE,
        vec!["-n", "-"],
        "tests/expected/the-bustle.txt.n.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_stdin_b() -> TestResult {
    run_stdin(
        BUSTLE,
        vec!["-b", "-"],
        "tests/expected/the-bustle.txt.b.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(vec![EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n() -> TestResult {
    run(vec!["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

// --------------------------------------------------
#[test]
fn empty_b() -> TestResult {
    run(vec!["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    run(vec![FOX], "tests/expected/fox.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_n() -> TestResult {
    run(vec!["-n", FOX], "tests/expected/fox.txt.n.out")
}

// --------------------------------------------------
#[test]
fn fox_b() -> TestResult {
    run(vec!["-b", FOX], "tests/expected/fox.txt.b.out")
}

// --------------------------------------------------
#[test]
fn spiders() -> TestResult {
    run(vec![SPIDERS], "tests/expected/spiders.txt.out")
}

// --------------------------------------------------
#[test]
fn spiders_n() -> TestResult {
    run(
        vec!["--number", SPIDERS],
        "tests/expected/spiders.txt.n.out",
    )
}

// --------------------------------------------------
#[test]
fn spiders_b() -> TestResult {
    run(
        vec!["--number-nonblank", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle() -> TestResult {
    run(vec![BUSTLE], "tests/expected/the-bustle.txt.out")
}

// --------------------------------------------------
#[test]
fn bustle_n() -> TestResult {
    run(vec!["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

// --------------------------------------------------
#[test]
fn bustle_b() -> TestResult {
    run(vec!["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

// --------------------------------------------------
#[test]
fn all() -> TestResult {
    run(vec![FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

// --------------------------------------------------
#[test]
fn all_n() -> TestResult {
    run(vec![FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

// --------------------------------------------------
#[test]
fn all_b() -> TestResult {
    run(vec![FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
