use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct Test<'a> {
    args: &'a Vec<&'a str>,
    out: &'a str,
}

static PRG: &str = "findr";

// --------------------------------------------------
#[test]
fn dies_bad_dir() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.arg("blargh").assert().stderr(
        predicate::str::is_match("\"blargh\": .* [(]os error 2[)]").unwrap(),
    );
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_name() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&["--name", "*.csv"])
        .assert()
        .stderr(predicate::str::contains("Invalid --name regex \"*.csv\""));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_type() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&["--type", "x"]).assert().stderr(
        predicate::str::is_match(
            "error: 'x' isn't a valid value for '--type <TYPE>'",
        )
        .unwrap(),
    );
    Ok(())
}

// --------------------------------------------------
fn run(test: &Test) -> TestResult {
    let expected = fs::read_to_string(test.out)?;

    Command::cargo_bin(PRG)?
        .args(test.args)
        .assert()
        .stdout(expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn path1() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs"],
        out: "tests/expected/path1.txt",
    })
}

// --------------------------------------------------
#[test]
fn path_a() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a"],
        out: "tests/expected/path_a.txt",
    })
}

// --------------------------------------------------
#[test]
fn path_a_b() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a/b"],
        out: "tests/expected/path_a_b.txt",
    })
}

// --------------------------------------------------
#[test]
fn path_d() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/d"],
        out: "tests/expected/path_d.txt",
    })
}

// --------------------------------------------------
#[test]
fn path_a_b_d() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a/b", "tests/inputs/d"],
        out: "tests/expected/path_a_b_d.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_f() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs", "-t", "f"],
        out: "tests/expected/type_f.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_f_path_a() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a", "-t", "f"],
        out: "tests/expected/type_f_path_a.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a/b", "--type", "f"],
        out: "tests/expected/type_f_path_a_b.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_f_path_d() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/d", "--type", "f"],
        out: "tests/expected/type_f_path_d.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_f_path_a_b_d() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a/b", "tests/inputs/d", "--type", "f"],
        out: "tests/expected/type_f_path_a_b_d.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_d() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs", "-t", "d"],
        out: "tests/expected/type_d.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_d_path_a() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a", "-t", "d"],
        out: "tests/expected/type_d_path_a.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a/b", "--type", "d"],
        out: "tests/expected/type_d_path_a_b.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_d_path_d() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/d", "--type", "d"],
        out: "tests/expected/type_d_path_d.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_d_path_a_b_d() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a/b", "tests/inputs/d", "--type", "d"],
        out: "tests/expected/type_d_path_a_b_d.txt",
    })
}

// --------------------------------------------------
#[test]
fn name_csv() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs", "-n", ".*.csv"],
        out: "tests/expected/name_csv.txt",
    })
}

// --------------------------------------------------
#[test]
fn name_csv_mp3() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs", "-n", ".*.csv", "-n", ".*.mp3"],
        out: "tests/expected/name_csv_mp3.txt",
    })
}

// --------------------------------------------------
#[test]
fn name_txt_path_a_d() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs/a", "tests/inputs/d", "--name", ".*.txt"],
        out: "tests/expected/name_txt_path_a_d.txt",
    })
}

// --------------------------------------------------
#[test]
fn name_a() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs", "-n", "a"],
        out: "tests/expected/name_a.txt",
    })
}
// --------------------------------------------------
#[test]
fn type_f_name_a() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs", "-t", "f", "-n", "a"],
        out: "tests/expected/type_f_name_a.txt",
    })
}

// --------------------------------------------------
#[test]
fn type_d_name_a() -> TestResult {
    run(&Test {
        args: &vec!["tests/inputs", "--type", "d", "--name", "a"],
        out: "tests/expected/type_d_name_a.txt",
    })
}
