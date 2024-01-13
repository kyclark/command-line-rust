use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::{fs, path::Path};
use sys_info::os_type;

const PRG: &str = "grepr";
const BUSTLE: &str = "tests/inputs/bustle.txt";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const NOBODY: &str = "tests/inputs/nobody.txt";
const INPUTS_DIR: &str = "tests/inputs";

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
fn dies_no_args() -> Result<()> {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_pattern() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(["*foo", FOX])
        .assert()
        .failure()
        .stderr(predicate::str::contains(r#"Invalid pattern "*foo""#));
    Ok(())
}

// --------------------------------------------------
#[test]
fn warns_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .args(["foo", &bad])
        .assert()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let windows_file = format!("{expected_file}.windows");
    let expected_file = if os_type().unwrap() == "Windows"
        && Path::new(&windows_file).is_file()
    {
        &windows_file
    } else {
        expected_file
    };

    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?.args(args).output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty_file() -> Result<()> {
    run(&["foo", EMPTY], "tests/expected/empty.foo")
}

// --------------------------------------------------
#[test]
fn empty_regex() -> Result<()> {
    run(&["", FOX], "tests/expected/empty_regex.fox.txt")
}

// --------------------------------------------------
#[test]
fn bustle_capitalized() -> Result<()> {
    run(
        &["The", BUSTLE],
        "tests/expected/bustle.txt.the.capitalized",
    )
}

// --------------------------------------------------
#[test]
fn bustle_lowercase() -> Result<()> {
    run(&["the", BUSTLE], "tests/expected/bustle.txt.the.lowercase")
}

// --------------------------------------------------
#[test]
fn bustle_insensitive() -> Result<()> {
    run(
        &["--insensitive", "the", BUSTLE],
        "tests/expected/bustle.txt.the.lowercase.insensitive",
    )
}

// --------------------------------------------------
#[test]
fn nobody() -> Result<()> {
    run(&["nobody", NOBODY], "tests/expected/nobody.txt")
}

// --------------------------------------------------
#[test]
fn nobody_insensitive() -> Result<()> {
    run(
        &["-i", "nobody", NOBODY],
        "tests/expected/nobody.txt.insensitive",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files() -> Result<()> {
    run(
        &["The", BUSTLE, EMPTY, FOX, NOBODY],
        "tests/expected/all.the.capitalized",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_insensitive() -> Result<()> {
    run(
        &["-i", "the", BUSTLE, EMPTY, FOX, NOBODY],
        "tests/expected/all.the.lowercase.insensitive",
    )
}

// --------------------------------------------------
#[test]
fn recursive() -> Result<()> {
    run(
        &["--recursive", "dog", INPUTS_DIR],
        "tests/expected/dog.recursive",
    )
}

// --------------------------------------------------
#[test]
fn recursive_insensitive() -> Result<()> {
    run(
        &["-ri", "then", INPUTS_DIR],
        "tests/expected/the.recursive.insensitive",
    )
}

// --------------------------------------------------
#[test]
fn sensitive_count_capital() -> Result<()> {
    run(
        &["--count", "The", BUSTLE],
        "tests/expected/bustle.txt.the.capitalized.count",
    )
}

// --------------------------------------------------
#[test]
fn sensitive_count_lower() -> Result<()> {
    run(
        &["--count", "the", BUSTLE],
        "tests/expected/bustle.txt.the.lowercase.count",
    )
}

// --------------------------------------------------
#[test]
fn insensitive_count() -> Result<()> {
    run(
        &["-ci", "the", BUSTLE],
        "tests/expected/bustle.txt.the.lowercase.insensitive.count",
    )
}

// --------------------------------------------------
#[test]
fn nobody_count() -> Result<()> {
    run(&["-c", "nobody", NOBODY], "tests/expected/nobody.txt.count")
}

// --------------------------------------------------
#[test]
fn nobody_count_insensitive() -> Result<()> {
    run(
        &["-ci", "nobody", NOBODY],
        "tests/expected/nobody.txt.insensitive.count",
    )
}

// --------------------------------------------------
#[test]
fn sensitive_count_multiple() -> Result<()> {
    run(
        &["-c", "The", BUSTLE, EMPTY, FOX, NOBODY],
        "tests/expected/all.the.capitalized.count",
    )
}

// --------------------------------------------------
#[test]
fn insensitive_count_multiple() -> Result<()> {
    run(
        &["-ic", "the", BUSTLE, EMPTY, FOX, NOBODY],
        "tests/expected/all.the.lowercase.insensitive.count",
    )
}

// --------------------------------------------------
#[test]
fn warns_dir_not_recursive() -> Result<()> {
    let stdout = "tests/inputs/fox.txt:\
        The quick brown fox jumps over the lazy dog.";
    Command::cargo_bin(PRG)?
        .args(["fox", INPUTS_DIR, FOX])
        .assert()
        .stderr(predicate::str::contains("tests/inputs is a directory"))
        .stdout(predicate::str::contains(stdout));
    Ok(())
}

// --------------------------------------------------
#[test]
fn stdin() -> Result<()> {
    let input = fs::read_to_string(BUSTLE)?;
    let expected =
        fs::read_to_string("tests/expected/bustle.txt.the.capitalized")?;

    let output = Command::cargo_bin(PRG)?
        .arg("The")
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn stdin_insensitive_count() -> Result<()> {
    let files = &[BUSTLE, EMPTY, FOX, NOBODY];

    let mut input = String::new();
    for file in files {
        input += &fs::read_to_string(file)?;
    }

    let expected_file =
        "tests/expected/the.recursive.insensitive.count.stdin";
    let expected = fs::read_to_string(expected_file)?;

    let output = Command::cargo_bin(PRG)?
        .args(["-ci", "the", "-"])
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}
