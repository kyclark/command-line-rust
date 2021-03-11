use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
//#[test]
//fn bad_file() -> TestResult {
//    let mut cmd = Command::cargo_bin("catr")?;
//    cmd.arg("foo").unwrap().unwrap_err();

//    Ok(())
//}

// --------------------------------------------------
#[test]
fn foo() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/foo.txt.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.arg("tests/inputs/foo.txt")
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn foo_n() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/foo.txt.n.out")
        .ok()
        .unwrap();
    println!("{}", &expected);
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-n", "tests/inputs/foo.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/fox.txt.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.arg("tests/inputs/fox.txt")
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn fox_n() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/fox.txt.n.out")
        .ok()
        .unwrap();
    println!("{}", &expected);
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-n", "tests/inputs/fox.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn all() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/all.out").ok().unwrap();
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["tests/inputs/foo.txt", "tests/inputs/fox.txt"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn all_n() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/all.n.out").ok().unwrap();
    println!("{}", &expected);
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["tests/inputs/foo.txt", "tests/inputs/fox.txt", "-n"])
        .unwrap()
        .assert()
        .stdout(expected);

    Ok(())
}
