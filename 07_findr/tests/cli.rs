use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::{borrow::Cow, fs, path::Path};

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
fn skips_bad_dir() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error [23][)]", &bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_name() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["--name", "*.csv"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid --name \"*.csv\""));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_type() -> TestResult {
    let expected = "error: 'x' isn't a valid value for '--type <TYPE>...'";
    Command::cargo_bin(PRG)?
        .args(&["--type", "x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[cfg(windows)]
fn format_file_name(expected_file: &str) -> Cow<str> {
    // Equivalent to: Cow::Owned(format!("{}.windows", expected_file))
    format!("{}.windows", expected_file).into()
}

// --------------------------------------------------
#[cfg(not(windows))]
fn format_file_name(expected_file: &str) -> Cow<str> {
    // Equivalent to: Cow::Borrowed(expected_file)
    expected_file.into()
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let file = format_file_name(expected_file);
    let contents = fs::read_to_string(file.as_ref())?;
    let mut expected: Vec<&str> =
        contents.split("\n").filter(|s| !s.is_empty()).collect();
    expected.sort();

    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let mut lines: Vec<&str> =
        stdout.split("\n").filter(|s| !s.is_empty()).collect();
    lines.sort();

    assert_eq!(lines, expected);

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
        &["tests/inputs", "-n", ".*[.]csv"],
        "tests/expected/name_csv.txt",
    )
}

// --------------------------------------------------
#[test]
fn name_csv_mp3() -> TestResult {
    run(
        &["tests/inputs", "-n", ".*[.]csv", "-n", ".*[.]mp3"],
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

// --------------------------------------------------
#[test]
fn path_g() -> TestResult {
    run(&["tests/inputs/g.csv"], "tests/expected/path_g.txt")
}

// --------------------------------------------------
#[test]
#[cfg(not(windows))]
fn unreadable_dir() -> TestResult {
    let dirname = "tests/inputs/cant-touch-this";
    if !Path::new(dirname).exists() {
        fs::create_dir(dirname)?;
    }

    //let metadata = fs::metadata(dirname)?;
    //let mut permissions = metadata.permissions();
    //permissions.set_mode(0o000);

    std::process::Command::new("chmod")
        .args(&["000", dirname])
        .status()
        .expect("failed");

    let cmd = Command::cargo_bin(PRG)?
        .arg("tests/inputs")
        .assert()
        .success();
    fs::remove_dir(dirname)?;

    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let lines: Vec<&str> =
        stdout.split("\n").filter(|s| !s.is_empty()).collect();

    assert_eq!(lines.len(), 17);

    let stderr = String::from_utf8(out.stderr.clone())?;
    assert!(stderr.contains("cant-touch-this: Permission denied"));
    Ok(())
}
