use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;
static EMPTY: &str = "tests/inputs/empty.txt";
static ONE: &str = "tests/inputs/one.txt";
static TWO: &str = "tests/inputs/two.txt";
static THREE: &str = "tests/inputs/three.txt";
static SKIP: &str = "tests/inputs/skip.txt";
static T1: &str = "tests/inputs/t1.txt";
static T2: &str = "tests/inputs/t2.txt";
static T3: &str = "tests/inputs/t3.txt";
static T4: &str = "tests/inputs/t4.txt";
static T5: &str = "tests/inputs/t5.txt";
static T6: &str = "tests/inputs/t6.txt";

// --------------------------------------------------
fn run<'a>(
    filename: &'a str,
    args: &'a mut Vec<&'a str>,
    expected_file: &str,
) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let input = fs::read_to_string(filename)?;

    Command::cargo_bin("uniqr")?
        .args(&*args)
        .write_stdin(input)
        .assert()
        .stdout(predicate::str::contains(&expected));

    args.push(filename);
    Command::cargo_bin("uniqr")?
        .args(args)
        .assert()
        .stdout(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(EMPTY, &mut vec![], "tests/inputs/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_c() -> TestResult {
    run(EMPTY, &mut vec!["-c"], "tests/inputs/empty.txt.c.out")
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(ONE, &mut vec![], "tests/inputs/one.txt.out")
}

// --------------------------------------------------
#[test]
fn one_c() -> TestResult {
    run(ONE, &mut vec!["-c"], "tests/inputs/one.txt.c.out")
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(TWO, &mut vec![], "tests/inputs/two.txt.out")
}

// --------------------------------------------------
#[test]
fn two_c() -> TestResult {
    run(TWO, &mut vec!["-c"], "tests/inputs/two.txt.c.out")
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run(THREE, &mut vec![], "tests/inputs/three.txt.out")
}

// --------------------------------------------------
#[test]
fn three_c() -> TestResult {
    run(THREE, &mut vec!["-c"], "tests/inputs/three.txt.c.out")
}

// --------------------------------------------------
#[test]
fn skip() -> TestResult {
    run(SKIP, &mut vec![], "tests/inputs/skip.txt.out")
}

// --------------------------------------------------
#[test]
fn skip_c() -> TestResult {
    run(SKIP, &mut vec!["-c"], "tests/inputs/skip.txt.c.out")
}

// --------------------------------------------------
#[test]
fn t1() -> TestResult {
    run(T1, &mut vec![], "tests/inputs/t1.txt.out")
}

// --------------------------------------------------
#[test]
fn t1_c() -> TestResult {
    run(T1, &mut vec!["-c"], "tests/inputs/t1.txt.c.out")
}

// --------------------------------------------------
#[test]
fn t2() -> TestResult {
    run(T2, &mut vec![], "tests/inputs/t2.txt.out")
}

// --------------------------------------------------
#[test]
fn t2_c() -> TestResult {
    run(T2, &mut vec!["-c"], "tests/inputs/t2.txt.c.out")
}

// --------------------------------------------------
#[test]
fn t3() -> TestResult {
    run(T3, &mut vec![], "tests/inputs/t3.txt.out")
}

// --------------------------------------------------
#[test]
fn t3_c() -> TestResult {
    run(T3, &mut vec!["-c"], "tests/inputs/t3.txt.c.out")
}

// --------------------------------------------------
#[test]
fn t4() -> TestResult {
    run(T4, &mut vec![], "tests/inputs/t4.txt.out")
}

// --------------------------------------------------
#[test]
fn t4_c() -> TestResult {
    run(T4, &mut vec!["-c"], "tests/inputs/t4.txt.c.out")
}

// --------------------------------------------------
#[test]
fn t5() -> TestResult {
    run(T5, &mut vec![], "tests/inputs/t5.txt.out")
}

// --------------------------------------------------
#[test]
fn t5_c() -> TestResult {
    run(T5, &mut vec!["-c"], "tests/inputs/t5.txt.c.out")
}

// --------------------------------------------------
#[test]
fn t6() -> TestResult {
    run(T6, &mut vec![], "tests/inputs/t6.txt.out")
}

// --------------------------------------------------
#[test]
fn t6_c() -> TestResult {
    run(T6, &mut vec!["-c"], "tests/inputs/t6.txt.c.out")
}
