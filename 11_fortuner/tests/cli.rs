use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "fortuner";
const FORTUNE_DIR: &str = "./tests/inputs";
const HUMORISTS: &str = "./tests/inputs/humorists";
const FORTUNES: &str = "./tests/inputs/fortunes";

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
fn dies_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .args(&[FORTUNES, &bad])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_seed() -> TestResult {
    let bad = random_string();
    let expected = format!("\"{}\" not a valid integer", &bad);
    Command::cargo_bin(PRG)?
        .args(&[FORTUNES, "--seed", &bad])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected: &'static str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn fortunes_seed_5() -> TestResult {
    run(
        &[FORTUNES, "-s", "5"],
        concat!(
            "Rivers know this: There is no hurry. We shall get there.\n",
            "-- A. A. Milne\n"
        ),
    )
}

//// --------------------------------------------------
#[test]
fn humorists_seed_31() -> TestResult {
    run(
        &[HUMORISTS, "-s", "31"],
        concat!(
            "I bought some used paint. It was in the shape of a house.\n",
            "		-- Steven Wright\n"
        ),
    )
}

// --------------------------------------------------
#[test]
fn dir_seed_51() -> TestResult {
    run(
        &[FORTUNE_DIR, "-s", "51"],
        "Friendship is love with understanding.\n",
    )
}

// --------------------------------------------------
fn run_outfiles(args: &[&str], out_file: &str, err_file: &str) -> TestResult {
    let out = fs::read_to_string(out_file)?;
    let err = fs::read_to_string(err_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stderr(err)
        .stdout(out);
    Ok(())
}

// --------------------------------------------------
#[test]
fn yogi_berra_cap() -> TestResult {
    run_outfiles(
        &["--pattern", "Yogi Berra", FORTUNE_DIR],
        "tests/expected/berra_cap.out",
        "tests/expected/berra_cap.err",
    )
}

// --------------------------------------------------
#[test]
fn will_rogers_cap() -> TestResult {
    run_outfiles(
        &["-m", "Will Rogers", FORTUNE_DIR],
        "tests/expected/rogers_cap.out",
        "tests/expected/rogers_cap.err",
    )
}

// --------------------------------------------------
#[test]
fn yogi_berra_lower() -> TestResult {
    run_outfiles(
        &["--pattern", "yogi berra", FORTUNE_DIR],
        "tests/expected/berra_lower.out",
        "tests/expected/berra_lower.err",
    )
}

// --------------------------------------------------
#[test]
fn will_rogers_lower() -> TestResult {
    run_outfiles(
        &["-m", "will rogers", FORTUNE_DIR],
        "tests/expected/rogers_lower.out",
        "tests/expected/rogers_lower.err",
    )
}

// --------------------------------------------------
#[test]
fn yogi_berra_lower_i() -> TestResult {
    run_outfiles(
        &["--insensitive", "--pattern", "yogi berra", FORTUNE_DIR],
        "tests/expected/berra_lower_i.out",
        "tests/expected/berra_lower_i.err",
    )
}

// --------------------------------------------------
#[test]
fn will_rogers_lower_i() -> TestResult {
    run_outfiles(
        &["-i", "-m", "will rogers", FORTUNE_DIR],
        "tests/expected/rogers_lower_i.out",
        "tests/expected/rogers_lower_i.err",
    )
}
