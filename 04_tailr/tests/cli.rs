use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("tailr")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin("tailr")?;
    cmd.arg("foo").assert().stderr(
        predicate::str::is_match("foo: .* [(]os error 2[)]").unwrap(),
    );

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_bytes() -> TestResult {
    let mut cmd = Command::cargo_bin("tailr")?;
    cmd.args(&["-c", "foo", "tests/inputs/empty.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal byte count -- foo"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_lines() -> TestResult {
    let mut cmd = Command::cargo_bin("tailr")?;
    cmd.args(&["-n", "bar", "tests/inputs/empty.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal line count -- bar"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_and_lines() -> TestResult {
    let mut cmd = Command::cargo_bin("tailr")?;
    let msg = "The argument '--lines <LINES>' cannot be \
               used with '--bytes <BYTES>'";

    cmd.args(&["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

// --------------------------------------------------
fn run(args: &Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).ok().unwrap();
    let mut cmd = Command::cargo_bin("tailr")?;
    cmd.args(args).unwrap().assert().stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt"],
        "tests/inputs/empty.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_n3() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-n", "3"],
        "tests/inputs/empty.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_n4() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-n", "4"],
        "tests/inputs/empty.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_c8() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-c", "8"],
        "tests/inputs/empty.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_c12() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-c", "12"],
        "tests/inputs/empty.txt.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(&vec!["tests/inputs/one.txt"], "tests/inputs/one.txt.out")
}

// --------------------------------------------------
#[test]
fn one_n3() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-n", "3"],
        "tests/inputs/one.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn one_n4() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-n", "4"],
        "tests/inputs/one.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn one_c8() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "8"],
        "tests/inputs/one.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn one_c12() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "12"],
        "tests/inputs/one.txt.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&vec!["tests/inputs/two.txt"], "tests/inputs/two.txt.out")
}

// --------------------------------------------------
#[test]
fn two_n3() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-n", "3"],
        "tests/inputs/two.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn two_n4() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-n", "4"],
        "tests/inputs/two.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn two_c8() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-c", "8"],
        "tests/inputs/two.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn two_c12() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-c", "12"],
        "tests/inputs/two.txt.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt"],
        "tests/inputs/three.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn three_n3() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-n", "3"],
        "tests/inputs/three.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn three_n4() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-n", "4"],
        "tests/inputs/three.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn three_c8() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-c", "8"],
        "tests/inputs/three.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn three_c12() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-c", "12"],
        "tests/inputs/three.txt.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn ten() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-n", "4"],
        "tests/inputs/10.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn ten_n3() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-n", "3"],
        "tests/inputs/10.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn ten_n4() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-n", "4"],
        "tests/inputs/10.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn ten_c8() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-c", "8"],
        "tests/inputs/10.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn ten_c12() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-c", "12"],
        "tests/inputs/10.txt.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files() -> TestResult {
    run(
        &vec![
            "tests/inputs/10.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
        ],
        "tests/inputs/all.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_n1() -> TestResult {
    run(
        &vec![
            "tests/inputs/10.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
            "-n",
            "1",
        ],
        "tests/inputs/all.n1.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_n3() -> TestResult {
    run(
        &vec![
            "-n",
            "3",
            "tests/inputs/10.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
        ],
        "tests/inputs/all.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_c8() -> TestResult {
    run(
        &vec![
            "tests/inputs/10.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
            "-c",
            "8",
        ],
        "tests/inputs/all.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_c12() -> TestResult {
    run(
        &vec![
            "-c",
            "12",
            "tests/inputs/10.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
        ],
        "tests/inputs/all.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_quiet() -> TestResult {
    run(
        &vec![
            "-q",
            "-n",
            "3",
            "tests/inputs/10.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
        ],
        "tests/inputs/all.n3.q.out",
    )
}
