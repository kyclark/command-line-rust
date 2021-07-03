use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct Test<'a> {
    args: &'a Vec<&'a str>,
    out: &'a str,
}

const PRG: &str = "grepr";

// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_pattern() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&["*foo", "tests/inputs/fox.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid pattern \"*foo\""));
    Ok(())
}

// --------------------------------------------------
#[test]
fn warns_bad_file() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(&["foo", "tests/inputs/foxx.txt"]).assert().stderr(
        predicate::str::contains(
            "tests/inputs/foxx.txt: No such file or directory",
        ),
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
fn empty() -> TestResult {
    run(&Test {
        args: &vec!["foo", "tests/inputs/empty.txt"],
        out: "tests/expected/empty.foo",
    })
}

// --------------------------------------------------
#[test]
fn bustle_capitalized() -> TestResult {
    run(&Test {
        args: &vec!["The", "tests/inputs/bustle.txt"],
        out: "tests/expected/bustle.txt.the.capitalized",
    })
}

// --------------------------------------------------
#[test]
fn bustle_lowercase() -> TestResult {
    run(&Test {
        args: &vec!["the", "tests/inputs/bustle.txt"],
        out: "tests/expected/bustle.txt.the.lowercase",
    })
}

// --------------------------------------------------
#[test]
fn bustle_insensitive() -> TestResult {
    run(&Test {
        args: &vec!["--insensitive", "the", "tests/inputs/bustle.txt"],
        out: "tests/expected/bustle.txt.the.lowercase.insensitive",
    })
}

// --------------------------------------------------
#[test]
fn nobody() -> TestResult {
    run(&Test {
        args: &vec!["nobody", "tests/inputs/nobody.txt"],
        out: "tests/expected/nobody.txt",
    })
}

// --------------------------------------------------
#[test]
fn nobody_insensitive() -> TestResult {
    run(&Test {
        args: &vec!["-i", "nobody", "tests/inputs/nobody.txt"],
        out: "tests/expected/nobody.txt.insensitive",
    })
}

// --------------------------------------------------
#[test]
fn multiple_files() -> TestResult {
    run(&Test {
        args: &vec![
            "The",
            "tests/inputs/bustle.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/fox.txt",
            "tests/inputs/nobody.txt",
        ],
        out: "tests/expected/all.the.capitalized",
    })
}

// --------------------------------------------------
#[test]
fn multiple_files_insensitive() -> TestResult {
    run(&Test {
        args: &vec![
            "-i",
            "the",
            "tests/inputs/bustle.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/fox.txt",
            "tests/inputs/nobody.txt",
        ],
        out: "tests/expected/all.the.lowercase.insensitive",
    })
}

// --------------------------------------------------
#[test]
fn recursive() -> TestResult {
    run(&Test {
        args: &vec!["--recursive", "dog", "tests/inputs"],
        out: "tests/expected/dog.recursive",
    })
}

// --------------------------------------------------
#[test]
fn recursive_insensitive() -> TestResult {
    run(&Test {
        args: &vec!["-ri", "the", "tests/inputs"],
        out: "tests/expected/the.recursive.insensitive",
    })
}

// --------------------------------------------------
#[test]
fn sensitive_count_capital() -> TestResult {
    run(&Test {
        args: &vec!["--count", "The", "tests/inputs/bustle.txt"],
        out: "tests/expected/bustle.txt.the.capitalized.count",
    })
}

// --------------------------------------------------
#[test]
fn sensitive_count_lower() -> TestResult {
    run(&Test {
        args: &vec!["--count", "the", "tests/inputs/bustle.txt"],
        out: "tests/expected/bustle.txt.the.lowercase.count",
    })
}

// --------------------------------------------------
#[test]
fn insensitive_count() -> TestResult {
    run(&Test {
        args: &vec!["-ci", "the", "tests/inputs/bustle.txt"],
        out: "tests/expected/bustle.txt.the.lowercase.insensitive.count",
    })
}

// --------------------------------------------------
#[test]
fn nobody_count() -> TestResult {
    run(&Test {
        args: &vec!["-c", "nobody", "tests/inputs/nobody.txt"],
        out: "tests/expected/nobody.txt.count",
    })
}

// --------------------------------------------------
#[test]
fn nobody_count_insensitive() -> TestResult {
    run(&Test {
        args: &vec!["-ci", "nobody", "tests/inputs/nobody.txt"],
        out: "tests/expected/nobody.txt.insensitive.count",
    })
}

// --------------------------------------------------
#[test]
fn sensitive_count_multiple() -> TestResult {
    run(&Test {
        args: &vec![
            "-c",
            "The",
            "tests/inputs/bustle.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/fox.txt",
            "tests/inputs/nobody.txt",
        ],
        out: "tests/expected/all.the.capitalized.count",
    })
}

// --------------------------------------------------
#[test]
fn insensitive_count_multiple() -> TestResult {
    run(&Test {
        args: &vec![
            "-ic",
            "the",
            "tests/inputs/bustle.txt",
            "tests/inputs/empty.txt",
            "tests/inputs/fox.txt",
            "tests/inputs/nobody.txt",
        ],
        out: "tests/expected/all.the.lowercase.insensitive.count",
    })
}

// --------------------------------------------------
#[test]
fn warns_dir_not_recursive() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    let stdout = "tests/inputs/fox.txt:\
        The quick brown fox jumps over the lazy dog.";
    cmd.args(&["fox", "tests/inputs", "tests/inputs/fox.txt"])
        .assert()
        .stderr(predicate::str::contains("tests/inputs is a directory"))
        .stdout(predicate::str::contains(stdout));
    Ok(())
}

// --------------------------------------------------
#[test]
fn stdin() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    let input = fs::read_to_string("tests/inputs/bustle.txt")?;
    let expected =
        fs::read_to_string("tests/expected/bustle.txt.the.capitalized")?;

    cmd.arg("The").write_stdin(input).assert().stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn stdin_insensitive_count() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    let files = vec![
        "tests/inputs/bustle.txt",
        "tests/inputs/empty.txt",
        "tests/inputs/fox.txt",
        "tests/inputs/nobody.txt",
    ];

    let mut input = String::new();
    for file in files {
        input += &fs::read_to_string(file)?;
    }

    let expected_file =
        "tests/expected/the.recursive.insensitive.count.stdin";
    let expected = fs::read_to_string(expected_file)?;

    cmd.args(&["-ci", "the", "-"])
        .write_stdin(input)
        .assert()
        .stdout(expected);
    Ok(())
}
