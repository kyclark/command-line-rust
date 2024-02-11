use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;
use tempfile::NamedTempFile;

struct Test {
    input: &'static str,
    out: &'static str,
    out_count: &'static str,
}

const PRG: &str = "uniqr";

const EMPTY: Test = Test {
    input: "tests/inputs/empty.txt",
    out: "tests/expected/empty.txt.out",
    out_count: "tests/expected/empty.txt.c.out",
};

const ONE: Test = Test {
    input: "tests/inputs/one.txt",
    out: "tests/expected/one.txt.out",
    out_count: "tests/expected/one.txt.c.out",
};

const TWO: Test = Test {
    input: "tests/inputs/two.txt",
    out: "tests/expected/two.txt.out",
    out_count: "tests/expected/two.txt.c.out",
};

const THREE: Test = Test {
    input: "tests/inputs/three.txt",
    out: "tests/expected/three.txt.out",
    out_count: "tests/expected/three.txt.c.out",
};

const SKIP: Test = Test {
    input: "tests/inputs/skip.txt",
    out: "tests/expected/skip.txt.out",
    out_count: "tests/expected/skip.txt.c.out",
};

const T1: Test = Test {
    input: "tests/inputs/t1.txt",
    out: "tests/expected/t1.txt.out",
    out_count: "tests/expected/t1.txt.c.out",
};

const T2: Test = Test {
    input: "tests/inputs/t2.txt",
    out: "tests/expected/t2.txt.out",
    out_count: "tests/expected/t2.txt.c.out",
};

const T3: Test = Test {
    input: "tests/inputs/t3.txt",
    out: "tests/expected/t3.txt.out",
    out_count: "tests/expected/t3.txt.c.out",
};

const T4: Test = Test {
    input: "tests/inputs/t4.txt",
    out: "tests/expected/t4.txt.out",
    out_count: "tests/expected/t4.txt.c.out",
};

const T5: Test = Test {
    input: "tests/inputs/t5.txt",
    out: "tests/expected/t5.txt.out",
    out_count: "tests/expected/t5.txt.c.out",
};

const T6: Test = Test {
    input: "tests/inputs/t6.txt",
    out: "tests/expected/t6.txt.out",
    out_count: "tests/expected/t6.txt.c.out",
};

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn dies_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(bad)
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
// HELPER FUNCTIONS
fn run(test: &Test) -> Result<()> {
    let expected = fs::read_to_string(test.out)?;
    let output = Command::cargo_bin(PRG)?
        .arg(test.input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
fn run_count(test: &Test) -> Result<()> {
    let expected = fs::read_to_string(test.out_count)?;
    let output = Command::cargo_bin(PRG)?
        .args([test.input, "-c"])
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
fn run_stdin(test: &Test) -> Result<()> {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.out)?;
    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
fn run_stdin_count(test: &Test) -> Result<()> {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.out_count)?;
    let output = Command::cargo_bin(PRG)?
        .arg("--count")
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
fn run_outfile(test: &Test) -> Result<()> {
    let expected = fs::read_to_string(test.out)?;
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();

    Command::cargo_bin(PRG)?
        .args([test.input, outpath])
        .assert()
        .success()
        .stdout("");
    let contents = fs::read_to_string(outpath)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

// --------------------------------------------------
fn run_outfile_count(test: &Test) -> Result<()> {
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();

    Command::cargo_bin(PRG)?
        .args([test.input, outpath, "--count"])
        .assert()
        .success()
        .stdout("");

    let expected = fs::read_to_string(test.out_count)?;
    let contents = fs::read_to_string(outpath)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

// --------------------------------------------------
fn run_stdin_outfile_count(test: &Test) -> Result<()> {
    let input = fs::read_to_string(test.input)?;
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();

    Command::cargo_bin(PRG)?
        .args(["-", outpath, "-c"])
        .write_stdin(input)
        .assert()
        .stdout("");

    let expected = fs::read_to_string(test.out_count)?;
    let contents = fs::read_to_string(outpath)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> Result<()> {
    run(&EMPTY)
}

#[test]
fn empty_count() -> Result<()> {
    run_count(&EMPTY)
}

#[test]
fn empty_stdin() -> Result<()> {
    run_stdin(&EMPTY)
}

#[test]
fn empty_stdin_count() -> Result<()> {
    run_stdin_count(&EMPTY)
}

#[test]
fn empty_outfile() -> Result<()> {
    run_outfile(&EMPTY)
}

#[test]
fn empty_outfile_count() -> Result<()> {
    run_outfile_count(&EMPTY)
}

#[test]
fn empty_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&EMPTY)
}

// --------------------------------------------------
#[test]
fn one() -> Result<()> {
    run(&ONE)
}

#[test]
fn one_count() -> Result<()> {
    run_count(&ONE)
}

#[test]
fn one_stdin() -> Result<()> {
    run_stdin(&ONE)
}

#[test]
fn one_stdin_count() -> Result<()> {
    run_stdin_count(&ONE)
}

#[test]
fn one_outfile() -> Result<()> {
    run_outfile(&ONE)
}

#[test]
fn one_outfile_count() -> Result<()> {
    run_outfile_count(&ONE)
}

#[test]
fn one_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&ONE)
}

// --------------------------------------------------
#[test]
fn two() -> Result<()> {
    run(&TWO)
}

#[test]
fn two_count() -> Result<()> {
    run_count(&TWO)
}

#[test]
fn two_stdin() -> Result<()> {
    run_stdin(&TWO)
}

#[test]
fn two_stdin_count() -> Result<()> {
    run_stdin_count(&TWO)
}

#[test]
fn two_outfile() -> Result<()> {
    run_outfile(&TWO)
}

#[test]
fn two_outfile_count() -> Result<()> {
    run_outfile_count(&TWO)
}

#[test]
fn two_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&TWO)
}

// --------------------------------------------------
#[test]
fn three() -> Result<()> {
    run(&THREE)
}

#[test]
fn three_count() -> Result<()> {
    run_count(&THREE)
}

#[test]
fn three_stdin() -> Result<()> {
    run_stdin(&THREE)
}

#[test]
fn three_stdin_count() -> Result<()> {
    run_stdin_count(&THREE)
}

#[test]
fn three_outfile() -> Result<()> {
    run_outfile(&THREE)
}

#[test]
fn three_outfile_count() -> Result<()> {
    run_outfile_count(&THREE)
}

#[test]
fn three_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&THREE)
}

// --------------------------------------------------
#[test]
fn skip() -> Result<()> {
    run(&SKIP)
}

#[test]
fn skip_count() -> Result<()> {
    run_count(&SKIP)
}

#[test]
fn skip_stdin() -> Result<()> {
    run_stdin(&SKIP)
}

#[test]
fn skip_stdin_count() -> Result<()> {
    run_stdin_count(&SKIP)
}

#[test]
fn skip_outfile() -> Result<()> {
    run_outfile(&SKIP)
}

#[test]
fn skip_outfile_count() -> Result<()> {
    run_outfile_count(&SKIP)
}

#[test]
fn skip_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&SKIP)
}

// --------------------------------------------------
#[test]
fn t1() -> Result<()> {
    run(&T1)
}

#[test]
fn t1_count() -> Result<()> {
    run_count(&T1)
}

#[test]
fn t1_stdin() -> Result<()> {
    run_stdin(&T1)
}

#[test]
fn t1_stdin_count() -> Result<()> {
    run_stdin_count(&T1)
}

#[test]
fn t1_outfile() -> Result<()> {
    run_outfile(&T1)
}

#[test]
fn t1_outfile_count() -> Result<()> {
    run_outfile_count(&T1)
}

#[test]
fn t1_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T1)
}

// --------------------------------------------------
#[test]
fn t2() -> Result<()> {
    run(&T2)
}

#[test]
fn t2_count() -> Result<()> {
    run_count(&T2)
}

#[test]
fn t2_stdin() -> Result<()> {
    run_stdin(&T2)
}

#[test]
fn t2_stdin_count() -> Result<()> {
    run_stdin_count(&T2)
}

#[test]
fn t2_outfile() -> Result<()> {
    run_outfile(&T2)
}

#[test]
fn t2_outfile_count() -> Result<()> {
    run_outfile_count(&T2)
}

#[test]
fn t2_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T2)
}

// --------------------------------------------------
#[test]
fn t3() -> Result<()> {
    run(&T3)
}

#[test]
fn t3_count() -> Result<()> {
    run_count(&T3)
}

#[test]
fn t3_stdin() -> Result<()> {
    run_stdin(&T3)
}

#[test]
fn t3_stdin_count() -> Result<()> {
    run_stdin_count(&T3)
}

#[test]
fn t3_outfile() -> Result<()> {
    run_outfile(&T3)
}

#[test]
fn t3_outfile_count() -> Result<()> {
    run_outfile_count(&T3)
}

#[test]
fn t3_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T3)
}

// --------------------------------------------------
#[test]
fn t4() -> Result<()> {
    run(&T4)
}

#[test]
fn t4_count() -> Result<()> {
    run_count(&T4)
}

#[test]
fn t4_stdin() -> Result<()> {
    run_stdin(&T4)
}

#[test]
fn t4_stdin_count() -> Result<()> {
    run_stdin_count(&T4)
}

#[test]
fn t4_outfile() -> Result<()> {
    run_outfile(&T4)
}

#[test]
fn t4_outfile_count() -> Result<()> {
    run_outfile_count(&T4)
}

#[test]
fn t4_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T4)
}

// --------------------------------------------------
#[test]
fn t5() -> Result<()> {
    run(&T5)
}

#[test]
fn t5_count() -> Result<()> {
    run_count(&T5)
}

#[test]
fn t5_stdin() -> Result<()> {
    run_stdin(&T5)
}

#[test]
fn t5_stdin_count() -> Result<()> {
    run_stdin_count(&T5)
}

#[test]
fn t5_outfile() -> Result<()> {
    run_outfile(&T5)
}

#[test]
fn t5_outfile_count() -> Result<()> {
    run_outfile_count(&T5)
}

#[test]
fn t5_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T5)
}

// --------------------------------------------------
#[test]
fn t6() -> Result<()> {
    run(&T6)
}

#[test]
fn t6_count() -> Result<()> {
    run_count(&T6)
}

#[test]
fn t6_stdin() -> Result<()> {
    run_stdin(&T6)
}

#[test]
fn t6_stdin_count() -> Result<()> {
    run_stdin_count(&T6)
}

#[test]
fn t6_outfile() -> Result<()> {
    run_outfile(&T6)
}

#[test]
fn t6_outfile_count() -> Result<()> {
    run_outfile_count(&T6)
}

#[test]
fn t6_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&T6)
}
