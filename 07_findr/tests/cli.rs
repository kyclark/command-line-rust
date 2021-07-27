use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "findr";

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
fn dies_bad_dir() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_name() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["--name", "*.csv"])
        .assert()
        .stderr(predicate::str::contains("Invalid --name regex \"*.csv\""));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_type() -> TestResult {
    let expected = "error: 'x' isn't a valid value for '--type <TYPE>...'";
    Command::cargo_bin(PRG)?
        .args(&["--type", "x"])
        .assert()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn path1() -> TestResult {
    run(&["tests/inputs"], "tests/expected/path1.txt")
}

// --------------------------------------------------
#[test]
fn path_a() -> TestResult {
    run(&["tests/inputs/a"], "tests/expected/path_a.txt")
}

// --------------------------------------------------
#[test]
fn path_a_b() -> TestResult {
    run(&["tests/inputs/a/b"], "tests/expected/path_a_b.txt")
}

// --------------------------------------------------
#[test]
fn path_d() -> TestResult {
    run(&["tests/inputs/d"], "tests/expected/path_d.txt")
}

// --------------------------------------------------
#[test]
fn path_a_b_d() -> TestResult {
    run(
        &["tests/inputs/a/b", "tests/inputs/d"],
        "tests/expected/path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f() -> TestResult {
    run(&["tests/inputs", "-t", "f"], "tests/expected/type_f.txt")
}

// --------------------------------------------------
#[test]
fn type_f_path_a() -> TestResult {
    run(
        &["tests/inputs/a", "-t", "f"],
        "tests/expected/type_f_path_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b() -> TestResult {
    run(
        &["tests/inputs/a/b", "--type", "f"],
        "tests/expected/type_f_path_a_b.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_d() -> TestResult {
    run(
        &["tests/inputs/d", "--type", "f"],
        "tests/expected/type_f_path_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b_d() -> TestResult {
    run(
        &["tests/inputs/a/b", "tests/inputs/d", "--type", "f"],
        "tests/expected/type_f_path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d() -> TestResult {
    run(&["tests/inputs", "-t", "d"], "tests/expected/type_d.txt")
}

// --------------------------------------------------
#[test]
fn type_d_path_a() -> TestResult {
    run(
        &["tests/inputs/a", "-t", "d"],
        "tests/expected/type_d_path_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b() -> TestResult {
    run(
        &["tests/inputs/a/b", "--type", "d"],
        "tests/expected/type_d_path_a_b.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_d() -> TestResult {
    run(
        &["tests/inputs/d", "--type", "d"],
        "tests/expected/type_d_path_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b_d() -> TestResult {
    run(
        &["tests/inputs/a/b", "tests/inputs/d", "--type", "d"],
        "tests/expected/type_d_path_a_b_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_l() -> TestResult {
    run(&["tests/inputs", "-t", "l"], "tests/expected/type_l.txt")
}

// --------------------------------------------------
#[test]
fn type_f_l() -> TestResult {
    run(
        &["tests/inputs", "-t", "l", "f"],
        "tests/expected/type_f_l.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_csv() -> TestResult {
    run(
        &["tests/inputs", "-n", ".*.csv"],
        "tests/expected/name_csv.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_csv_mp3() -> TestResult {
    run(
        &["tests/inputs", "-n", ".*.csv", "-n", ".*.mp3"],
        "tests/expected/name_csv_mp3.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_txt_path_a_d() -> TestResult {
    run(
        &["tests/inputs/a", "tests/inputs/d", "--name", ".*.txt"],
        "tests/expected/name_txt_path_a_d.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_a() -> TestResult {
    run(&["tests/inputs", "-n", "a"], "tests/expected/name_a.txt")
}

// --------------------------------------------------
#[test]
fn type_f_name_a() -> TestResult {
    run(
        &["tests/inputs", "-t", "f", "-n", "a"],
        "tests/expected/type_f_name_a.txt",
    )
}

// --------------------------------------------------
#[test]
fn type_d_name_a() -> TestResult {
    run(
        &["tests/inputs", "--type", "d", "--name", "a"],
        "tests/expected/type_d_name_a.txt",
    )
}
