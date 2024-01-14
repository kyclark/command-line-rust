use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;

const PRG: &str = "calr";

// --------------------------------------------------
#[test]
fn dies_year_0() -> Result<()> {
    Command::cargo_bin(PRG)?.arg("0").assert().failure().stderr(
        predicate::str::contains(
            "error: invalid value '0' for '[YEAR]': 0 is not in 1..=9999",
        ),
    );
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_year_10000() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg("10000")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: invalid value \'10000\' \
                for \'[YEAR]\': 10000 is not in 1..=9999",
        ));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_invalid_year() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg("foo")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: invalid value \'foo\' for \'[YEAR]\': \
                invalid digit found in string",
        ));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_month_0() -> Result<()> {
    let output = Command::cargo_bin(PRG)?
        .args(["-m", "0"])
        .output()
        .expect("fail");
    assert!(!output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, "");

    let stderr = String::from_utf8(output.stderr).expect("invalid UTF-8");
    assert_eq!(stderr.trim(), r#"month "0" not in the range 1 through 12"#);

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_month_13() -> Result<()> {
    let output = Command::cargo_bin(PRG)?
        .args(["-m", "13"])
        .output()
        .expect("fail");
    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("invalid UTF-8");
    assert_eq!(stderr.trim(), r#"month "13" not in the range 1 through 12"#);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_invalid_month() -> Result<()> {
    let output = Command::cargo_bin(PRG)?
        .args(["-m", "foo"])
        .output()
        .expect("fail");
    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("invalid UTF-8");
    assert_eq!(stderr.trim(), r#"Invalid month "foo""#);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_y_and_month() -> Result<()> {
    let expected = "the argument '-m <MONTH>' cannot be used with '--year'";
    Command::cargo_bin(PRG)?
        .args(["-m", "1", "-y"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_y_and_year() -> Result<()> {
    let expected = "the argument '--year' cannot be used with '[YEAR]'";
    Command::cargo_bin(PRG)?
        .args(["-y", "2000"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// --------------------------------------------------
#[test]
fn month_num() -> Result<()> {
    let expected = &[
        ("1", "January"),
        ("2", "February"),
        ("3", "March"),
        ("4", "April"),
        ("5", "May"),
        ("6", "June"),
        ("7", "July"),
        ("8", "August"),
        ("9", "September"),
        ("10", "October"),
        ("11", "November"),
        ("12", "December"),
    ];

    for (num, month) in expected {
        Command::cargo_bin(PRG)?
            .args(["-m", num])
            .assert()
            .success()
            .stdout(predicates::str::contains(month.to_string()));
    }
    Ok(())
}

// --------------------------------------------------
#[test]
fn partial_month() -> Result<()> {
    let expected = &[
        ("ja", "January"),
        ("f", "February"),
        ("mar", "March"),
        ("ap", "April"),
        ("may", "May"),
        ("jun", "June"),
        ("jul", "July"),
        ("au", "August"),
        ("s", "September"),
        ("n", "November"),
        ("d", "December"),
    ];

    for (arg, month) in expected {
        Command::cargo_bin(PRG)?
            .args(["-m", arg])
            .assert()
            .success()
            .stdout(predicates::str::contains(month.to_string()));
    }
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
fn default_one_month() -> Result<()> {
    let cmd = Command::cargo_bin(PRG)?.assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let lines: Vec<_> = stdout.split('\n').collect();
    assert_eq!(lines.len(), 9);
    assert_eq!(lines[0].len(), 22);
    Ok(())
}

// --------------------------------------------------
#[test]
fn test_2_2020_leap_year() -> Result<()> {
    run(&["-m", "2", "2020"], "tests/expected/2-2020.txt")
}

// --------------------------------------------------
#[test]
fn test_4_2020() -> Result<()> {
    run(&["-m", "4", "2020"], "tests/expected/4-2020.txt")
}

// --------------------------------------------------
#[test]
fn test_april_2020() -> Result<()> {
    run(&["2020", "-m", "april"], "tests/expected/4-2020.txt")
}

// --------------------------------------------------
#[test]
fn test_2020() -> Result<()> {
    run(&["2020"], "tests/expected/2020.txt")
}

// --------------------------------------------------
#[test]
fn year() -> Result<()> {
    let cmd = Command::cargo_bin(PRG)?.arg("-y").assert().success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let lines: Vec<&str> = stdout.split('\n').collect();
    assert_eq!(lines.len(), 37);
    Ok(())
}
