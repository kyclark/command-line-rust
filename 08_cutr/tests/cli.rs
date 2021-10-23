use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "cutr";
const CSV: &str = "tests/inputs/movies1.csv";
const TSV: &str = "tests/inputs/movies1.tsv";
const BOOKS: &str = "tests/inputs/books.tsv";

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
        .args(&["-f", "1", CSV, &bad, TSV])
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn dies(args: &[&str], expected: &str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_not_enough_args() -> TestResult {
    dies(&[CSV], "Must have --fields, --bytes, or --chars")
}

// --------------------------------------------------
#[test]
fn dies_bad_digit_field() -> TestResult {
    let bad = random_string();
    dies(
        &[CSV, "-f", &bad],
        &format!("illegal list value: \"{}\"", &bad),
    )
}

// --------------------------------------------------
#[test]
fn dies_bad_digit_bytes() -> TestResult {
    let bad = random_string();
    dies(
        &[CSV, "-b", &bad],
        &format!("illegal list value: \"{}\"", &bad),
    )
}

// --------------------------------------------------
#[test]
fn dies_bad_digit_chars() -> TestResult {
    let bad = random_string();
    dies(
        &[CSV, "-c", &bad],
        &format!("illegal list value: \"{}\"", &bad),
    )
}

// --------------------------------------------------
#[test]
fn dies_empty_delimiter() -> TestResult {
    dies(
        &[CSV, "-f", "1", "-d", ""],
        "--delim \"\" must be a single byte",
    )
}

// --------------------------------------------------
#[test]
fn dies_bad_delimiter() -> TestResult {
    dies(
        &[CSV, "-f", "1", "-d", ",,"],
        "--delim \",,\" must be a single byte",
    )
}

// --------------------------------------------------
#[test]
fn dies_chars_bytes_fields() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&[CSV, "-c", "1", "-f", "1", "-b", "1"])
        .assert()
        .failure();
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_fields() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "1", "-b", "1"])
        .assert()
        .failure();
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_chars_fields() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&[CSV, "-c", "1", "-f", "1"])
        .assert()
        .failure();
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_chars_bytes() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&[CSV, "-c", "1", "-b", "1"])
        .assert()
        .failure();
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    println!("expected {}", &expected_file);
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
fn run_lossy(args: &[&str], expected_file: &str) -> TestResult {
    let contents = fs::read(expected_file)?;
    let expected = String::from_utf8_lossy(&contents);
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[test]
fn tsv_f1() -> TestResult {
    run(&[TSV, "-f", "1"], "tests/expected/movies1.tsv.f1.out")
}

// --------------------------------------------------
#[test]
fn tsv_f2() -> TestResult {
    run(&[TSV, "-f", "2"], "tests/expected/movies1.tsv.f2.out")
}

// --------------------------------------------------
#[test]
fn tsv_f3() -> TestResult {
    run(&[TSV, "-f", "3"], "tests/expected/movies1.tsv.f3.out")
}

// --------------------------------------------------
#[test]
fn tsv_f1_2() -> TestResult {
    run(&[TSV, "-f", "1-2"], "tests/expected/movies1.tsv.f1-2.out")
}

// --------------------------------------------------
#[test]
fn tsv_f2_3() -> TestResult {
    run(&[TSV, "-f", "2-3"], "tests/expected/movies1.tsv.f2-3.out")
}

// --------------------------------------------------
#[test]
fn tsv_f1_3() -> TestResult {
    run(&[TSV, "-f", "1-3"], "tests/expected/movies1.tsv.f1-3.out")
}

// --------------------------------------------------
#[test]
fn csv_f1() -> TestResult {
    run(
        &[CSV, "-f", "1", "-d", ","],
        "tests/expected/movies1.csv.f1.dcomma.out",
    )
}

// --------------------------------------------------
#[test]
fn csv_f2() -> TestResult {
    run(
        &[CSV, "-f", "2", "-d", ","],
        "tests/expected/movies1.csv.f2.dcomma.out",
    )
}

// --------------------------------------------------
#[test]
fn csv_f3() -> TestResult {
    run(
        &[CSV, "-f", "3", "-d", ","],
        "tests/expected/movies1.csv.f3.dcomma.out",
    )
}

// --------------------------------------------------
#[test]
fn csv_f1_2() -> TestResult {
    run(
        &[CSV, "-f", "1-2", "-d", ","],
        "tests/expected/movies1.csv.f1-2.dcomma.out",
    )
}

// --------------------------------------------------
#[test]
fn csv_f2_3() -> TestResult {
    run(
        &[CSV, "-f", "2-3", "-d", ","],
        "tests/expected/movies1.csv.f2-3.dcomma.out",
    )
}

// --------------------------------------------------
#[test]
fn csv_f1_3() -> TestResult {
    run(
        &[CSV, "-f", "1-3", "-d", ","],
        "tests/expected/movies1.csv.f1-3.dcomma.out",
    )
}

// --------------------------------------------------
#[test]
fn tsv_b1() -> TestResult {
    run(&[TSV, "-b", "1"], "tests/expected/movies1.tsv.b1.out")
}

// --------------------------------------------------
#[test]
fn tsv_b2() -> TestResult {
    run(&[TSV, "-b", "2"], "tests/expected/movies1.tsv.b2.out")
}

// --------------------------------------------------
#[test]
fn tsv_b8() -> TestResult {
    run_lossy(&[TSV, "-b", "8"], "tests/expected/movies1.tsv.b8.out")
}

// --------------------------------------------------
#[test]
fn tsv_b1_2() -> TestResult {
    run(&[TSV, "-b", "1-2"], "tests/expected/movies1.tsv.b1-2.out")
}

// --------------------------------------------------
#[test]
fn tsv_b2_3() -> TestResult {
    run(&[TSV, "-b", "2-3"], "tests/expected/movies1.tsv.b2-3.out")
}

// --------------------------------------------------
#[test]
fn tsv_b1_8() -> TestResult {
    run_lossy(&[TSV, "-b", "1-8"], "tests/expected/movies1.tsv.b1-8.out")
}

// --------------------------------------------------
#[test]
fn tsv_c1() -> TestResult {
    run(&[TSV, "-c", "1"], "tests/expected/movies1.tsv.c1.out")
}

// --------------------------------------------------
#[test]
fn tsv_c2() -> TestResult {
    run(&[TSV, "-c", "2"], "tests/expected/movies1.tsv.c2.out")
}

// --------------------------------------------------
#[test]
fn tsv_c8() -> TestResult {
    run(&[TSV, "-c", "8"], "tests/expected/movies1.tsv.c8.out")
}

// --------------------------------------------------
#[test]
fn tsv_c1_2() -> TestResult {
    run(&[TSV, "-c", "1-2"], "tests/expected/movies1.tsv.c1-2.out")
}

// --------------------------------------------------
#[test]
fn tsv_c2_3() -> TestResult {
    run(&[TSV, "-c", "2-3"], "tests/expected/movies1.tsv.c2-3.out")
}

// --------------------------------------------------
#[test]
fn tsv_c1_8() -> TestResult {
    run(&[TSV, "-c", "1-8"], "tests/expected/movies1.tsv.c1-8.out")
}

// --------------------------------------------------
#[test]
fn repeated_value() -> TestResult {
    run(&[BOOKS, "-c", "1,1"], "tests/expected/books.c1,1.out")
}
