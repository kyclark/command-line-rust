use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
fn make_long_re(filename: &str, size: String) -> String {
    vec![
        r"[d-][r-][w-][x-][r-][w-][x-][r-][w-][x-]".to_string(), // perms
        r"[ ]".to_string(),                                      // space
        r"[\d ]{2}".to_string(),                                 // num links
        r"[ ]".to_string(),                                      // space
        r"[\w ]{8}".to_string(),                                 // username
        r"[ ]".to_string(),                                      // space
        r"[\w ]{8}".to_string(),                                 // groupname
        r"[ ]".to_string(),                                      // space
        size,
        r"[ ]".to_string(), // space
        r"(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)".to_string(),
        r"[ ]".to_string(),           // space
        r"[\d ]{2}".to_string(),      // day
        r"[ ]".to_string(),           // space
        r"\d{2}".to_string(),         // year
        r"[ ]".to_string(),           // space
        r"\d{2}[:]\d{2}".to_string(), // time
        r"[ ]".to_string(),           // space
        filename.to_string(),
    ]
    .join("")
}

// --------------------------------------------------
#[test]
fn bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    cmd.arg("foo")
        .assert()
        .stderr(predicate::str::contains("foo: No such file or directory"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn no_args() -> TestResult {
    // Uses current directory by default
    let mut cmd = Command::cargo_bin("lsr")?;
    cmd.assert().success();
    Ok(())
}

// --------------------------------------------------
fn fmt_num(num: &str) -> String {
    format!("[ ]{{{}}}{}", 8 - num.len(), num)
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    cmd.arg("tests/inputs/empty.txt")
        .unwrap()
        .assert()
        .stdout("tests/inputs/empty.txt\n");
    Ok(())
}

// --------------------------------------------------
#[test]
fn empty_long() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    let expected = make_long_re("tests/inputs/empty.txt", fmt_num("0"));
    cmd.args(vec!["--long", "tests/inputs/empty.txt"])
        .unwrap()
        .assert()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_long() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    let expected = make_long_re("tests/inputs/dir", r"[\d ]{8}".to_string());
    cmd.args(vec!["--long", "tests/inputs/dir"])
        .unwrap()
        .assert()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    cmd.arg("tests/inputs/fox.txt")
        .unwrap()
        .assert()
        .stdout("tests/inputs/fox.txt\n");
    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_long() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    let expected = make_long_re("tests/inputs/fox.txt", fmt_num("45"));
    cmd.args(vec!["--long", "tests/inputs/fox.txt"])
        .unwrap()
        .assert()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn bustle() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    cmd.arg("tests/inputs/bustle.txt")
        .unwrap()
        .assert()
        .stdout("tests/inputs/bustle.txt\n");
    Ok(())
}

// --------------------------------------------------
#[test]
fn bustle_long() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    let expected = make_long_re("tests/inputs/bustle.txt", fmt_num("193"));
    cmd.args(vec!["--long", "tests/inputs/bustle.txt"])
        .unwrap()
        .assert()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn spiders() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    cmd.arg("tests/inputs/dir/spiders.txt")
        .unwrap()
        .assert()
        .stdout("tests/inputs/dir/spiders.txt\n");
    Ok(())
}

// --------------------------------------------------
#[test]
fn spiders_long() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    let expected =
        make_long_re("tests/inputs/dir/spiders.txt", fmt_num("45"));
    cmd.args(vec!["--long", "tests/inputs/dir/spiders.txt"])
        .unwrap()
        .assert()
        .stdout(predicate::str::is_match(expected).unwrap());
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_list() -> TestResult {
    let mut cmd = Command::cargo_bin("lsr")?;
    for expected in vec![
        "tests/inputs/empty.txt",
        "tests/inputs/bustle.txt",
        "tests/inputs/fox.txt",
        "tests/inputs/dir",
    ] {
        cmd.arg("tests/inputs")
            .unwrap()
            .assert()
            .stdout(predicate::str::contains(expected));
    }
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_list_all() -> TestResult {
    for expected in vec![
        "tests/inputs/.hidden",
        "tests/inputs/empty.txt",
        "tests/inputs/bustle.txt",
        "tests/inputs/fox.txt",
        "tests/inputs/dir",
    ] {
        let mut cmd = Command::cargo_bin("lsr")?;
        cmd.args(vec!["--all", "tests/inputs"])
            .unwrap()
            .assert()
            .stdout(predicate::str::contains(expected));
    }
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_list_long() -> TestResult {
    for expected in vec![
        make_long_re("tests/inputs/empty.txt", fmt_num("0")),
        make_long_re("tests/inputs/bustle.txt", fmt_num("193")),
        make_long_re("tests/inputs/fox.txt", fmt_num("45")),
        make_long_re("tests/inputs/dir", r"[\d ]{8}".to_string()),
    ] {
        let mut cmd = Command::cargo_bin("lsr")?;
        cmd.args(vec!["-l", "tests/inputs"])
            .unwrap()
            .assert()
            .stdout(predicate::str::is_match(expected).unwrap());
    }
    Ok(())
}

// --------------------------------------------------
#[test]
fn dir_list_long_all() -> TestResult {
    for expected in vec![
        make_long_re("tests/inputs/.hidden", fmt_num("0")),
        make_long_re("tests/inputs/empty.txt", fmt_num("0")),
        make_long_re("tests/inputs/bustle.txt", fmt_num("193")),
        make_long_re("tests/inputs/fox.txt", fmt_num("45")),
        make_long_re("tests/inputs/dir", r"[\d ]{8}".to_string()),
    ] {
        let mut cmd = Command::cargo_bin("lsr")?;
        cmd.args(vec!["-la", "tests/inputs"])
            .unwrap()
            .assert()
            .stdout(predicate::str::is_match(expected).unwrap());
    }
    Ok(())
}
