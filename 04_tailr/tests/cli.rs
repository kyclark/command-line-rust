use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("tailr")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_file() -> TestResult {
    Command::cargo_bin("tailr")?.arg("foo").assert().stderr(
        predicate::str::is_match("foo: .* [(]os error 2[)]").unwrap(),
    );

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_bytes() -> TestResult {
    Command::cargo_bin("tailr")?
        .args(&["-c", "foo", "tests/inputs/empty.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal byte count -- foo"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_lines() -> TestResult {
    Command::cargo_bin("tailr")?
        .args(&["-n", "bar", "tests/inputs/empty.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal line count -- bar"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_and_lines() -> TestResult {
    let msg = "The argument '--lines <LINES>' cannot be \
               used with '--bytes <BYTES>'";

    Command::cargo_bin("tailr")?
        .args(&["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

// --------------------------------------------------
fn run(args: &Vec<&str>, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).ok().unwrap();
    Command::cargo_bin("tailr")?
        .args(args)
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt"],
        "tests/expected/empty.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_n3() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-n", "3"],
        "tests/expected/empty.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_n4() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-n", "4"],
        "tests/expected/empty.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_c8() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-c", "8"],
        "tests/expected/empty.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_c12() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-c", "12"],
        "tests/expected/empty.txt.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(&vec!["tests/inputs/one.txt"], "tests/expected/one.txt.out")
}

// --------------------------------------------------
#[test]
fn one_n3() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-n", "3"],
        "tests/expected/one.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn one_n4() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-n", "4"],
        "tests/expected/one.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn one_c8() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "8"],
        "tests/expected/one.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn one_c12() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "12"],
        "tests/expected/one.txt.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&vec!["tests/inputs/two.txt"], "tests/expected/two.txt.out")
}

// --------------------------------------------------
#[test]
fn two_n3() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-n", "3"],
        "tests/expected/two.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn two_n4() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-n", "4"],
        "tests/expected/two.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn two_c8() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-c", "8"],
        "tests/expected/two.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn two_c12() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-c", "12"],
        "tests/expected/two.txt.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt"],
        "tests/expected/three.txt.out",
    )
}

// --------------------------------------------------
#[test]
fn three_n3() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-n", "3"],
        "tests/expected/three.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn three_n4() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-n", "4"],
        "tests/expected/three.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn three_c8() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-c", "8"],
        "tests/expected/three.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn three_c12() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-c", "12"],
        "tests/expected/three.txt.c12.out",
    )
}

// --------------------------------------------------
#[test]
fn ten() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-n", "4"],
        "tests/expected/10.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn ten_n3() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-n", "3"],
        "tests/expected/10.txt.n3.out",
    )
}

// --------------------------------------------------
#[test]
fn ten_n4() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-n", "4"],
        "tests/expected/10.txt.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn ten_c8() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-c", "8"],
        "tests/expected/10.txt.c8.out",
    )
}

// --------------------------------------------------
#[test]
fn ten_c12() -> TestResult {
    run(
        &vec!["tests/inputs/10.txt", "-c", "12"],
        "tests/expected/10.txt.c12.out",
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
        "tests/expected/all.out",
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
        "tests/expected/all.n1.out",
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
        "tests/expected/all.n3.out",
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
        "tests/expected/all.c8.out",
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
        "tests/expected/all.c12.out",
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
        "tests/expected/all.n3.q.out",
    )
}
