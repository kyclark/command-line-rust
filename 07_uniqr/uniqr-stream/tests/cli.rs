use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
fn run<'a>(
    filename: &'a str,
    args: &'a mut Vec<&'a str>,
    expected_file: &str,
) -> TestResult {
    let expected = fs::read_to_string(expected_file).ok().unwrap();

    let input = fs::read_to_string(filename).ok().unwrap();
    let _cmd_stdin = Command::cargo_bin("uniqr")?
        .args(&*args)
        .write_stdin(input)
        .assert()
        .stdout(predicate::str::contains(&expected));

    args.push(filename);
    let _cmd = Command::cargo_bin("uniqr")?
        .args(args)
        .assert()
        .stdout(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(
        "tests/inputs/empty.txt",
        &mut vec![],
        "tests/inputs/empty.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_c() -> TestResult {
    run(
        "tests/inputs/empty.txt",
        &mut vec!["-c"],
        "tests/inputs/empty.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(
        "tests/inputs/one.txt",
        &mut vec![],
        "tests/inputs/one.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn one_c() -> TestResult {
    run(
        "tests/inputs/one.txt",
        &mut vec!["-c"],
        "tests/inputs/one.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(
        "tests/inputs/two.txt",
        &mut vec![],
        "tests/inputs/two.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn two_c() -> TestResult {
    run(
        "tests/inputs/two.txt",
        &mut vec!["-c"],
        "tests/inputs/two.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run(
        "tests/inputs/three.txt",
        &mut vec![],
        "tests/inputs/three.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn three_c() -> TestResult {
    run(
        "tests/inputs/three.txt",
        &mut vec!["-c"],
        "tests/inputs/three.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn skip() -> TestResult {
    run(
        "tests/inputs/skip.txt",
        &mut vec![],
        "tests/inputs/skip.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn skip_c() -> TestResult {
    run(
        "tests/inputs/skip.txt",
        &mut vec!["-c"],
        "tests/inputs/skip.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn t1() -> TestResult {
    run(
        "tests/inputs/t1.txt",
        &mut vec![],
        "tests/inputs/t1.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn t1_c() -> TestResult {
    run(
        "tests/inputs/t1.txt",
        &mut vec!["-c"],
        "tests/inputs/t1.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn t2() -> TestResult {
    run(
        "tests/inputs/t2.txt",
        &mut vec![],
        "tests/inputs/t2.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn t2_c() -> TestResult {
    run(
        "tests/inputs/t2.txt",
        &mut vec!["-c"],
        "tests/inputs/t2.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn t3() -> TestResult {
    run(
        "tests/inputs/t3.txt",
        &mut vec![],
        "tests/inputs/t3.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn t3_c() -> TestResult {
    run(
        "tests/inputs/t3.txt",
        &mut vec!["-c"],
        "tests/inputs/t3.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn t4() -> TestResult {
    run(
        "tests/inputs/t4.txt",
        &mut vec![],
        "tests/inputs/t4.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn t4_c() -> TestResult {
    run(
        "tests/inputs/t4.txt",
        &mut vec!["-c"],
        "tests/inputs/t4.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn t5() -> TestResult {
    run(
        "tests/inputs/t5.txt",
        &mut vec![],
        "tests/inputs/t5.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn t5_c() -> TestResult {
    run(
        "tests/inputs/t5.txt",
        &mut vec!["-c"],
        "tests/inputs/t5.txt.c.out",
    )
}

// --------------------------------------------------
#[test]
fn t6() -> TestResult {
    run(
        "tests/inputs/t6.txt",
        &mut vec![],
        "tests/inputs/t6.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn t6_c() -> TestResult {
    run(
        "tests/inputs/t6.txt",
        &mut vec!["-c"],
        "tests/inputs/t6.txt.c.out",
    )
}
