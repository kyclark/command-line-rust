use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

const PRG: &str = "commr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FILE1: &str = "tests/inputs/file1.txt";
const FILE2: &str = "tests/inputs/file2.txt";
const BLANK: &str = "tests/inputs/blank.txt";

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
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
fn dies_bad_file1() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .args(&[&bad, FILE1])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_file2() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .args(&[FILE1, &bad])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_both_stdin() -> TestResult {
    let expected = "Both input files cannot be STDIN (\"-\")";
    Command::cargo_bin(PRG)?
        .args(&["-", "-"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
fn run_stdin(
    args: &[&str],
    input_file: &str,
    expected_file: &str,
) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty_empty() -> TestResult {
    run(&[EMPTY, EMPTY], "tests/expected/empty_empty.out")
}

// --------------------------------------------------
#[test]
fn file1_file1() -> TestResult {
    run(&[FILE1, FILE1], "tests/expected/file1_file1.out")
}

// --------------------------------------------------
#[test]
fn file1_file2() -> TestResult {
    run(&[FILE1, FILE2], "tests/expected/file1_file2.out")
}

// --------------------------------------------------
#[test]
fn file1_empty() -> TestResult {
    run(&[FILE1, EMPTY], "tests/expected/file1_empty.out")
}

// --------------------------------------------------
#[test]
fn empty_file2() -> TestResult {
    run(&[EMPTY, FILE2], "tests/expected/empty_file2.out")
}

// --------------------------------------------------
#[test]
fn file1_file2_1() -> TestResult {
    run(&["-1", FILE1, FILE2], "tests/expected/file1_file2.1.out")
}

// --------------------------------------------------
#[test]
fn file1_file2_2() -> TestResult {
    run(&["-2", FILE1, FILE2], "tests/expected/file1_file2.2.out")
}

// --------------------------------------------------
#[test]
fn file1_file2_3() -> TestResult {
    run(&["-3", FILE1, FILE2], "tests/expected/file1_file2.3.out")
}

// --------------------------------------------------
#[test]
fn file1_file2_1_2() -> TestResult {
    run(&["-12", FILE1, FILE2], "tests/expected/file1_file2.12.out")
}

// --------------------------------------------------
#[test]
fn file1_file2_2_3() -> TestResult {
    run(&["-23", FILE1, FILE2], "tests/expected/file1_file2.23.out")
}

// --------------------------------------------------
#[test]
fn file1_file2_13() -> TestResult {
    run(&["-13", FILE1, FILE2], "tests/expected/file1_file2.13.out")
}

// --------------------------------------------------
#[test]
fn file1_file2_123() -> TestResult {
    run(
        &["-123", FILE1, FILE2],
        "tests/expected/file1_file2.123.out",
    )
}

// --------------------------------------------------
// insensitive
// --------------------------------------------------
#[test]
fn file1_file2_1_i() -> TestResult {
    run(
        &["-1", "-i", FILE1, FILE2],
        "tests/expected/file1_file2.1.i.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_2_i() -> TestResult {
    run(
        &["-2", "-i", FILE1, FILE2],
        "tests/expected/file1_file2.2.i.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_3_i() -> TestResult {
    run(
        &["-3", "-i", FILE1, FILE2],
        "tests/expected/file1_file2.3.i.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_1_2_i() -> TestResult {
    run(
        &["-12", "-i", FILE1, FILE2],
        "tests/expected/file1_file2.12.i.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_2_3_i() -> TestResult {
    run(
        &["-23", "-i", FILE1, FILE2],
        "tests/expected/file1_file2.23.i.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_13_i() -> TestResult {
    run(
        &["-13", "-i", FILE1, FILE2],
        "tests/expected/file1_file2.13.i.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_123_i() -> TestResult {
    run(
        &["-123", "-i", FILE1, FILE2],
        "tests/expected/file1_file2.123.i.out",
    )
}

// --------------------------------------------------
#[test]
fn stdin_file1() -> TestResult {
    run_stdin(
        &["-123", "-i", "-", FILE2],
        FILE1,
        "tests/expected/file1_file2.123.i.out",
    )
}

// --------------------------------------------------
#[test]
fn stdin_file2() -> TestResult {
    run_stdin(
        &["-123", "-i", FILE1, "-"],
        FILE2,
        "tests/expected/file1_file2.123.i.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_delim() -> TestResult {
    run(
        &[FILE1, FILE2, "-d", ":"],
        "tests/expected/file1_file2.delim.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_1_delim() -> TestResult {
    run(
        &[FILE1, FILE2, "-1", "-d", ":"],
        "tests/expected/file1_file2.1.delim.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_2_delim() -> TestResult {
    run(
        &[FILE1, FILE2, "-2", "-d", ":"],
        "tests/expected/file1_file2.2.delim.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_3_delim() -> TestResult {
    run(
        &[FILE1, FILE2, "-3", "-d", ":"],
        "tests/expected/file1_file2.3.delim.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_12_delim() -> TestResult {
    run(
        &[FILE1, FILE2, "-12", "-d", ":"],
        "tests/expected/file1_file2.12.delim.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_23_delim() -> TestResult {
    run(
        &[FILE1, FILE2, "-23", "-d", ":"],
        "tests/expected/file1_file2.23.delim.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_13_delim() -> TestResult {
    run(
        &[FILE1, FILE2, "-13", "-d", ":"],
        "tests/expected/file1_file2.13.delim.out",
    )
}

// --------------------------------------------------
#[test]
fn file1_file2_123_delim() -> TestResult {
    run(
        &[FILE1, FILE2, "-123", "-d", ":"],
        "tests/expected/file1_file2.123.delim.out",
    )
}

// --------------------------------------------------
#[test]
fn blank_file1() -> TestResult {
    run(&[BLANK, FILE1], "tests/expected/blank_file1.out")
}

//// --------------------------------------------------
//#[test]
//fn file1_blanks() -> TestResult {
//    run(&[FILE1, BLANKS], "tests/expected/file1_blanks.out")
//}
