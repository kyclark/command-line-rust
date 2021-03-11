use assert_cmd::prelude::*;
use std::process::Command;
//use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn test1() -> TestResult {
    let mut cmd = Command::cargo_bin("uniqr")?;
    cmd.arg("tests/inputs/one.txt")
        .unwrap()
        .assert()
        .stdout("one\n");

    Ok(())
}

// --------------------------------------------------
#[test]
fn test2() -> TestResult {
    let mut cmd = Command::cargo_bin("uniqr")?;
    cmd.args(&["-c", "tests/inputs/one.txt"])
        .unwrap()
        .assert()
        .stdout("   1 one\n");

    Ok(())
}

// --------------------------------------------------
#[test]
fn test3() -> TestResult {
    let mut cmd = Command::cargo_bin("uniqr")?;
    cmd.args(&["tests/inputs/two.txt"])
        .unwrap()
        .assert()
        .stdout("one\n");

    Ok(())
}

// --------------------------------------------------
#[test]
fn test4() -> TestResult {
    let mut cmd = Command::cargo_bin("uniqr")?;
    cmd.args(&["-c", "tests/inputs/two.txt"])
        .unwrap()
        .assert()
        .stdout("   2 one\n");

    Ok(())
}

// --------------------------------------------------
#[test]
fn test5() -> TestResult {
    let mut cmd = Command::cargo_bin("uniqr")?;
    let expected =
        vec!["one", "two", "one", "three", "one", "four", ""].join("\n");

    cmd.args(&["tests/inputs/three.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn test6() -> TestResult {
    let expected = vec![
        "   2 one",
        "   2 two",
        "   1 one",
        "   3 three",
        "   1 one",
        "   4 four",
        "",
    ]
    .join("\n");
    let mut cmd = Command::cargo_bin("uniqr")?;
    cmd.args(&["--counts", "tests/inputs/three.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}
