use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::{error::Error, fs};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "lsr";
const HIDDEN: &str = "tests/inputs/.hidden";
const EMPTY: &str = "tests/inputs/empty.txt";
const BUSTLE: &str = "tests/inputs/bustle.txt";
const FOX: &str = "tests/inputs/fox.txt";

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
fn run_short(arg: &str) -> TestResult {
    Command::cargo_bin(PRG)?
        .arg(arg)
        .assert()
        .success()
        .stdout(format!("{}\n", arg));
    Ok(())
}

// --------------------------------------------------
fn run_long(filename: &str, permissions: &str, size: &str) -> TestResult {
    let cmd = Command::cargo_bin(PRG)?
        .args(&["--long", filename])
        .assert()
        .success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let parts: Vec<_> = stdout.split_whitespace().collect();
    assert_eq!(parts.get(0).unwrap(), &permissions);
    assert_eq!(parts.get(4).unwrap(), &size);
    assert_eq!(parts.last().unwrap(), &filename);
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run_short(EMPTY)
}

#[test]
fn empty_long() -> TestResult {
    run_long(EMPTY, "-rw-r--r--", "0")
}

// --------------------------------------------------
#[test]
fn bustle() -> TestResult {
    run_short(BUSTLE)
}

#[test]
fn bustle_long() -> TestResult {
    run_long(BUSTLE, "-rw-r--r--", "193")
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    run_short(FOX)
}

#[test]
fn fox_long() -> TestResult {
    run_long(FOX, "-rw-------", "45")
}

// --------------------------------------------------
#[test]
fn hidden() -> TestResult {
    run_short(HIDDEN)
}

#[test]
fn hidden_long() -> TestResult {
    run_long(HIDDEN, "-rw-r--r--", "0")
}

// --------------------------------------------------
fn dir_short(args: &[&str], expected: &[&str]) -> TestResult {
    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let lines: Vec<&str> =
        stdout.split("\n").filter(|s| !s.is_empty()).collect();
    assert_eq!(lines.len(), expected.len());
    for filename in expected {
        assert!(lines.contains(&filename));
    }
    Ok(())
}

#[test]
fn dir1() -> TestResult {
    dir_short(
        &["tests/inputs"],
        &[
            "tests/inputs/empty.txt",
            "tests/inputs/bustle.txt",
            "tests/inputs/fox.txt",
            "tests/inputs/dir",
        ],
    )
}

#[test]
fn dir1_all() -> TestResult {
    dir_short(
        &["tests/inputs", "--all"],
        &[
            "tests/inputs/empty.txt",
            "tests/inputs/bustle.txt",
            "tests/inputs/fox.txt",
            "tests/inputs/.hidden",
            "tests/inputs/dir",
        ],
    )
}

#[test]
fn dir2() -> TestResult {
    dir_short(&["tests/inputs/dir"], &["tests/inputs/dir/spiders.txt"])
}

#[test]
fn dir2_all() -> TestResult {
    dir_short(
        &["-a", "tests/inputs/dir"],
        &["tests/inputs/dir/spiders.txt", "tests/inputs/dir/.gitkeep"],
    )
}

// --------------------------------------------------
fn dir_long(args: &[&str], expected: &[(&str, &str, &str)]) -> TestResult {
    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let lines: Vec<&str> =
        stdout.split("\n").filter(|s| !s.is_empty()).collect();
    assert_eq!(lines.len(), expected.len());

    let mut check = vec![];
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let path = parts.last().unwrap().clone();
        let permissions = parts.get(0).unwrap().clone();
        let size = match permissions.chars().next() {
            Some('d') => "",
            _ => parts.get(4).unwrap().clone(),
        };
        check.push((path, permissions, size));
    }

    for entry in expected {
        assert!(check.contains(entry));
    }

    Ok(())
}

// --------------------------------------------------
#[test]
fn dir1_long() -> TestResult {
    dir_long(
        &["-l", "tests/inputs"],
        &[
            ("tests/inputs/empty.txt", "-rw-r--r--", "0"),
            ("tests/inputs/bustle.txt", "-rw-r--r--", "193"),
            ("tests/inputs/fox.txt", "-rw-------", "45"),
            ("tests/inputs/dir", "drwxr-xr-x", ""),
        ],
    )
}

#[test]
fn dir1_long_all() -> TestResult {
    dir_long(
        &["-la", "tests/inputs"],
        &[
            ("tests/inputs/empty.txt", "-rw-r--r--", "0"),
            ("tests/inputs/bustle.txt", "-rw-r--r--", "193"),
            ("tests/inputs/fox.txt", "-rw-------", "45"),
            ("tests/inputs/dir", "drwxr-xr-x", ""),
            ("tests/inputs/.hidden", "-rw-r--r--", "0"),
        ],
    )
}

#[test]
fn dir2_long() -> TestResult {
    dir_long(
        &["--long", "tests/inputs/dir"],
        &[("tests/inputs/dir/spiders.txt", "-rw-r--r--", "45")],
    )
}

#[test]
fn dir2_long_all() -> TestResult {
    dir_long(
        &["tests/inputs/dir", "--long", "--all"],
        &[
            ("tests/inputs/dir/spiders.txt", "-rw-r--r--", "45"),
            ("tests/inputs/dir/.gitkeep", "-rw-r--r--", "0"),
        ],
    )
}
