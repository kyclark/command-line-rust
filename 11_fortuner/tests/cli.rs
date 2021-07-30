use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "fortuner";
const FORTUNES: &str = "tests/inputs/fortunes";

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
fn run(seed: &str, expected: &'static str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&[FORTUNES, "-s", seed])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn seed_3() -> TestResult {
    run("3",
        "It always seems impossible until its done.\n-- Theodor Seuss Geisel\n"
    )
}

// --------------------------------------------------
#[test]
fn seed_100() -> TestResult {
    run("100", "Excellent day to have a rotten day.\n")
}
