use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("headr")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin("headr")?;
    cmd.arg("foo").assert().stderr(
        predicate::str::is_match("foo: .* [(]os error 2[)]").unwrap(),
    );

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_bytes() -> TestResult {
    let mut cmd = Command::cargo_bin("headr")?;
    cmd.args(&["-c", "foo", "tests/inputs/empty.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal byte count -- foo"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_lines() -> TestResult {
    let mut cmd = Command::cargo_bin("headr")?;
    cmd.args(&["-n", "bar", "tests/inputs/empty.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal line count -- bar"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_and_lines() -> TestResult {
    let mut cmd = Command::cargo_bin("headr")?;
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
    let mut f = File::open(expected_file)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);
    let mut cmd = Command::cargo_bin("headr")?;
    cmd.args(args)
        .unwrap()
        .assert()
        .stdout(predicate::str::contains(expected));

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
fn empty_n2() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-n", "2"],
        "tests/inputs/empty.txt.n2.out",
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
fn empty_c2() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-c", "2"],
        "tests/inputs/empty.txt.c2.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_c4() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-c", "4"],
        "tests/inputs/empty.txt.c4.out",
    )
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(&vec!["tests/inputs/one.txt"], "tests/inputs/one.txt.out")
}

// --------------------------------------------------
#[test]
fn one_n2() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-n", "2"],
        "tests/inputs/one.txt.n2.out",
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
fn one_c1() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "1"],
        "tests/inputs/one.txt.c1.out",
    )
}

// --------------------------------------------------
#[test]
fn one_c2() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "2"],
        "tests/inputs/one.txt.c2.out",
    )
}

// --------------------------------------------------
#[test]
fn one_c4() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "4"],
        "tests/inputs/one.txt.c4.out",
    )
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&vec!["tests/inputs/two.txt"], "tests/inputs/two.txt.out")
}

// --------------------------------------------------
#[test]
fn two_n2() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-n", "2"],
        "tests/inputs/two.txt.n2.out",
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
fn two_c2() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-c", "2"],
        "tests/inputs/two.txt.c2.out",
    )
}

// --------------------------------------------------
#[test]
fn two_c4() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-c", "4"],
        "tests/inputs/two.txt.c4.out",
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
fn three_n2() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-n", "2"],
        "tests/inputs/three.txt.n2.out",
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
fn three_c2() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-c", "2"],
        "tests/inputs/three.txt.c2.out",
    )
}

// --------------------------------------------------
#[test]
fn three_c4() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-c", "4"],
        "tests/inputs/three.txt.c4.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files() -> TestResult {
    run(
        &vec![
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
fn multiple_files_n2() -> TestResult {
    run(
        &vec![
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
            "-n",
            "2",
        ],
        "tests/inputs/all.n2.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_n4() -> TestResult {
    run(
        &vec![
            "-n",
            "4",
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
        ],
        "tests/inputs/all.n4.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_c1() -> TestResult {
    run(
        &vec![
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
            "-c",
            "1",
        ],
        "tests/inputs/all.c1.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_c2() -> TestResult {
    run(
        &vec![
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
            "-c",
            "2",
        ],
        "tests/inputs/all.c2.out",
    )
}

// --------------------------------------------------
#[test]
fn multiple_files_c4() -> TestResult {
    run(
        &vec![
            "-c",
            "4",
            "tests/inputs/empty.txt",
            "tests/inputs/one.txt",
            "tests/inputs/three.txt",
            "tests/inputs/two.txt",
        ],
        "tests/inputs/all.c4.out",
    )
}
