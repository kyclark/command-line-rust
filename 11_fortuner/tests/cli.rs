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
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .args(&[&bad])
        .assert()
        .success()
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
fn fortunes_seed_3() -> TestResult {
    run(&[FORTUNES, "-s", "3"],
        "It always seems impossible until its done.\n-- Theodor Seuss Geisel\n"
    )
}

// --------------------------------------------------
#[test]
fn humorists_seed_1() -> TestResult {
    run(
        &[HUMORISTS, "-s", "1"],
        concat!(
            "Life is wasted on the living.\n",
            "		-- The Restaurant at the Edge of the Universe.\n"
        ),
    )
}

// --------------------------------------------------
#[test]
fn dir_seed_5() -> TestResult {
    run(
        &[FORTUNE_DIR, "-s", "5"],
        "It is good to have an end to journey toward; \
        but it is the journey that matters, in the end.\n\
        -- Ursula K. Le Guin\n",
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
