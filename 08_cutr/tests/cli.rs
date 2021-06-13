use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

static PRG: &str = "cutr";
static CSV: &str = "tests/inputs/1.csv";
static TSV: &str = "tests/inputs/1.tsv";

//struct Test<'a> {
//    args: Box<[&'a str]>,
//    expected: &'a str,
//}

//let TSV1: Test = Test {
//    args: Box::new(["-f", "1", "tests/inputs/1.tsv"]),
//    expected: "tests/inputs/1.tsv.f1.out",
//};

//fn run(test: &Test) -> TestResult {
//    let expected = fs::read_to_string(test.expected)?;

//    Command::cargo_bin(PRG)?
//        .args(test.args.into_iter())
//        .assert()
//        .stdout(expected);

//    Ok(())
//}

#[test]
fn tsv_f1() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.f1.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "1"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.f2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.f3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f1_2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.f1-2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "1-2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f2_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.f2-3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "2-3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_f1_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.f1-3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-f", "1-3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f1() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.csv.f1.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "1", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.csv.f2.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "2", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.csv.f3.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "3", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f1_2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.csv.f1-2.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "1-2", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f2_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.csv.f2-3.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "2-3", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn csv_f1_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.csv.f1-3.dcomma.out")?;

    Command::cargo_bin(PRG)?
        .args(&[CSV, "-f", "1-3", "-d", ","])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b1() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.b1.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "1"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
#[test]
fn tsv_b2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.b2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.b3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b1_2() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.b1-2.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "1-2"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b2_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.b2-3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "2-3"])
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn tsv_b1_3() -> TestResult {
    let expected = fs::read_to_string("tests/inputs/1.tsv.b1-3.out")?;

    Command::cargo_bin(PRG)?
        .args(&[TSV, "-b", "1-3"])
        .assert()
        .stdout(expected);

    Ok(())
}
