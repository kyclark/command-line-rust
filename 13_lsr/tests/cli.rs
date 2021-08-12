use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "lsr";
const HIDDEN: &str = "tests/inputs/.hidden";
const EMPTY: &str = "tests/inputs/empty.txt";
const BUSTLE: &str = "tests/inputs/bustle.txt";
const FOX: &str = "tests/inputs/fox.txt";
const DIR: &str = "tests/inputs/dir";
//const SPIDERS: &str = "tests/inputs/dir/spiders.txt";

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
fn make_long_re(filename: &str, size: &str) -> String {
    vec![
        r"([ld-][r-][w-][x-][r-][w-][x-][r-][w-][x-])", // perms
        r"[ ]",                                         // space
        r"[\d ]{2}",                                    // num links
        r"[ ]",                                         // space
        r"\w+",                                         // username
        r"[ ]",                                         // space
        r"\w+",                                         // groupname
        r"[ ]",                                         // space
        size,
        r"[ ]", // space
        r"\w+", // stuff
        filename,
    ]
    .join("")
}

// --------------------------------------------------
#[test]
fn bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected =
        format!("{}: No such file or directory (os error 2)", &bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn no_args() -> TestResult {
    // Uses current directory by default
    Command::cargo_bin(PRG)?
        .assert()
        .success()
        .stdout(predicate::str::contains("Cargo.toml"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg(EMPTY)
        .assert()
        .success()
        .stdout("tests/inputs/empty.txt\n");
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty_long() -> TestResult {
    let expected = make_long_re(EMPTY, "[ ]{7}0");
    Command::cargo_bin(PRG)?
        .args(&["--long", EMPTY])
        .assert()
        .success()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_long() -> TestResult {
    let expected = make_long_re("tests/inputs/dir", r"[\d ]{8}");
    Command::cargo_bin(PRG)?
        .args(&["--long", "tests/inputs/dir"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("tests/inputs/fox.txt")
        .assert()
        .success()
        .stdout("tests/inputs/fox.txt\n");
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_long() -> TestResult {
    let expected = make_long_re("tests/inputs/fox.txt", "[ ]{6}45");
    Command::cargo_bin(PRG)?
        .args(&["--long", "tests/inputs/fox.txt"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn bustle() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("tests/inputs/bustle.txt")
        .assert()
        .success()
        .stdout("tests/inputs/bustle.txt\n");
    Ok(())
}

// --------------------------------------------------
#[test]
fn bustle_long() -> TestResult {
    let expected = make_long_re("tests/inputs/bustle.txt", "[ ]{5}193");
    Command::cargo_bin(PRG)?
        .args(&["--long", "tests/inputs/bustle.txt"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn spiders() -> TestResult {
    Command::cargo_bin(PRG)?
        .arg("tests/inputs/dir/spiders.txt")
        .assert()
        .success()
        .stdout("tests/inputs/dir/spiders.txt\n");
    Ok(())
}

// --------------------------------------------------
#[test]
fn spiders_long() -> TestResult {
    let expected = make_long_re("tests/inputs/dir/spiders.txt", "[ ]{6}45");
    Command::cargo_bin(PRG)?
        .args(&["--long", "tests/inputs/dir/spiders.txt"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_list() -> TestResult {
    for expected in &[EMPTY, BUSTLE, FOX, DIR] {
        Command::cargo_bin(PRG)?
            .arg("tests/inputs")
            .assert()
            .success()
            .stdout(predicate::str::contains(expected.to_string()));
    }
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_list_all() -> TestResult {
    for expected in &[HIDDEN, EMPTY, BUSTLE, FOX, DIR] {
        Command::cargo_bin(PRG)?
            .args(&["--all", "tests/inputs"])
            .assert()
            .success()
            .stdout(predicate::str::contains(expected.to_string()));
    }
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_list_long() -> TestResult {
    for expected in &[
        make_long_re(EMPTY, "[ ]{7}0"),
        make_long_re(BUSTLE, "[ ]{5}193"),
        make_long_re(FOX, "[ ]{6}45"),
        make_long_re(DIR, r"[\d ]{8}"),
    ] {
        Command::cargo_bin(PRG)?
            .args(&["-l", "tests/inputs"])
            .assert()
            .success()
            .stdout(predicate::str::is_match(expected).unwrap());
    }
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_list_long_all() -> TestResult {
    for expected in &[
        make_long_re(HIDDEN, "[ ]{7}0"),
        make_long_re(EMPTY, "[ ]{7}0"),
        make_long_re(BUSTLE, "[ ]{5}193"),
        make_long_re(FOX, "[ ]{6}45"),
        make_long_re(DIR, r"[\d ]{8}"),
    ] {
        Command::cargo_bin(PRG)?
            .args(&["-la", "tests/inputs"])
            .assert()
            .stdout(predicate::str::is_match(expected).unwrap());
    }
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_list_with_link() -> TestResult {
    let cmd = Command::cargo_bin(PRG)?
        .args(&["--long", DIR])
        .assert()
        .success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let mut lines: Vec<&str> =
        stdout.split("\n").filter(|s| !s.is_empty()).collect();
    lines.sort();

    assert_eq!(lines.len(), 2);

    if let Some(first) = lines.first() {
        assert!(first.starts_with("-"));
    }

    if let Some(last) = lines.last() {
        assert!(last.starts_with("l"));
    }

    Ok(())
}
