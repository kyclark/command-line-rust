use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("headr")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_file() -> TestResult {
    Command::cargo_bin("headr")?.arg("foo").assert().stderr(
        predicate::str::is_match("foo: .* [(]os error 2[)]").unwrap(),
    );

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_bytes() -> TestResult {
    Command::cargo_bin("headr")?
        .args(&["-c", "foo", "tests/inputs/empty.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("illegal byte count -- foo"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_lines() -> TestResult {
    Command::cargo_bin("headr")?
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

    Command::cargo_bin("headr")?
        .args(&["-n", "1", "-c", "2"])
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

    Command::cargo_bin("headr")?
        .args(args)
        .assert()
        .stdout(predicate::str::contains(expected));

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
fn empty_n2() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-n", "2"],
        "tests/expected/empty.txt.n2.out",
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
fn empty_c2() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-c", "2"],
        "tests/expected/empty.txt.c2.out",
    )
}

// --------------------------------------------------
#[test]
fn empty_c4() -> TestResult {
    run(
        &vec!["tests/inputs/empty.txt", "-c", "4"],
        "tests/expected/empty.txt.c4.out",
    )
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run(&vec!["tests/inputs/one.txt"], "tests/expected/one.txt.out")
}

// --------------------------------------------------
#[test]
fn one_n2() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-n", "2"],
        "tests/expected/one.txt.n2.out",
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
fn one_c1() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "1"],
        "tests/expected/one.txt.c1.out",
    )
}

// --------------------------------------------------
#[test]
fn one_c2() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "2"],
        "tests/expected/one.txt.c2.out",
    )
}

// --------------------------------------------------
#[test]
fn one_c4() -> TestResult {
    run(
        &vec!["tests/inputs/one.txt", "-c", "4"],
        "tests/expected/one.txt.c4.out",
    )
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&vec!["tests/inputs/two.txt"], "tests/expected/two.txt.out")
}

// --------------------------------------------------
#[test]
fn two_n2() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-n", "2"],
        "tests/expected/two.txt.n2.out",
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
fn two_c2() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-c", "2"],
        "tests/expected/two.txt.c2.out",
    )
}

// --------------------------------------------------
#[test]
fn two_c4() -> TestResult {
    run(
        &vec!["tests/inputs/two.txt", "-c", "4"],
        "tests/expected/two.txt.c4.out",
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
fn three_n2() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-n", "2"],
        "tests/expected/three.txt.n2.out",
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
fn three_c2() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-c", "2"],
        "tests/expected/three.txt.c2.out",
    )
}

// --------------------------------------------------
#[test]
fn three_c4() -> TestResult {
    run(
        &vec!["tests/inputs/three.txt", "-c", "4"],
        "tests/expected/three.txt.c4.out",
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
        "tests/expected/all.out",
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
        "tests/expected/all.n2.out",
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
        "tests/expected/all.n4.out",
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
        "tests/expected/all.c1.out",
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
        "tests/expected/all.c2.out",
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
        "tests/expected/all.c4.out",
    )
}
