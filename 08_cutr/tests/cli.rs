use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

static PRG: &str = "cutr";
static CSV: &str = "tests/inputs/movies1.csv";
static TSV: &str = "tests/inputs/movies1.tsv";

#[test]
fn dies_bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&["-f", "1", "blargh"]).assert().stderr(
        predicate::str::is_match("blargh: .* [(]os error 2[)]").unwrap(),
    );
    Ok(())
}

#[test]
fn dies_not_enough_args() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.arg(CSV).assert().stderr(predicate::str::contains(
        "Must have --fields, --bytes, or --chars",
    ));
    Ok(())
}

#[test]
fn dies_bad_digit_field() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&[CSV, "-f", "blargh"])
        .assert()
        .stderr(predicate::str::contains("illegal list value: \"blargh\""));
    Ok(())
}

#[test]
fn dies_bad_digit_bytes() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&[CSV, "-b", "blargh"])
        .assert()
        .stderr(predicate::str::contains("illegal list value: \"blargh\""));
    Ok(())
}

#[test]
fn dies_bad_digit_chars() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&[CSV, "-c", "blargh"])
        .assert()
        .stderr(predicate::str::contains("illegal list value: \"blargh\""));
    Ok(())
}

#[test]
fn dies_chars_bytes_fields() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&[CSV, "-c", "1", "-f", "1", "-b", "1"])
        .assert()
        .failure();
    Ok(())
}

#[test]
fn dies_bytes_fields() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&[CSV, "-f", "1", "-b", "1"]).assert().failure();
    Ok(())
}

#[test]
fn dies_chars_fields() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&[CSV, "-c", "1", "-f", "1"]).assert().failure();
    Ok(())
}

#[test]
fn dies_chars_bytes() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&[CSV, "-c", "1", "-b", "1"]).assert().failure();
    Ok(())
}

#[test]
fn tsv_f1() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.f1.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "1"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.f2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.f3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f1_2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.f1-2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "1-2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f2_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.f2-3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "2-3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f1_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.f1-3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "1-3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f1() -> TestResult {
    let expected =
        fs::read_to_string("tests/inputs/movies1.csv.f1.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "1", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f2() -> TestResult {
    let expected =
        fs::read_to_string("tests/inputs/movies1.csv.f2.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "2", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f3() -> TestResult {
    let expected =
        fs::read_to_string("tests/inputs/movies1.csv.f3.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "3", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f1_2() -> TestResult {
    let expected =
        fs::read_to_string("tests/inputs/movies1.csv.f1-2.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "1-2", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f2_3() -> TestResult {
    let expected =
        fs::read_to_string("tests/inputs/movies1.csv.f2-3.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "2-3", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f1_3() -> TestResult {
    let expected =
        fs::read_to_string("tests/inputs/movies1.csv.f1-3.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "1-3", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b1() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.b1.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "1"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.b2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b8() -> TestResult {
    let contents = fs::read("tests/inputs/movies1.tsv.b8.out")?;
    let expected = String::from_utf8_lossy(&contents);

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "8"])
        .assert()
        .stdout(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn tsv_b1_2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.b1-2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "1-2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b2_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.b2-3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "2-3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b1_8() -> TestResult {
    let contents = fs::read("tests/inputs/movies1.tsv.b1-8.out")?;
    let expected = String::from_utf8_lossy(&contents);

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "1-8"])
        .assert()
        .stdout(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn tsv_c1() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.c1.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-c", "1"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_c2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.c2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-c", "2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_c8() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.c8.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-c", "8"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_c1_2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.c1-2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-c", "1-2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_c2_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.c2-3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-c", "2-3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_c1_8() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/movies1.tsv.c1-8.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-c", "1-8"])
        .assert()
        .stdout(expected);

    Ok(())
}
