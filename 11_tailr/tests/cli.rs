use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::{self, File};
use std::io::Read;

const PRG: &str = "tailr";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONE: &str = "tests/inputs/one.txt";
const TWO: &str = "tests/inputs/two.txt";
const THREE: &str = "tests/inputs/three.txt";
const TWELVE: &str = "tests/inputs/twelve.txt";

// --------------------------------------------------
fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn dies_no_args() -> Result<()> {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_bytes() -> Result<()> {
    let bad = random_string();
    let expected = format!("illegal byte count -- {bad}");
    Command::cargo_bin(PRG)?
        .args(["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_lines() -> Result<()> {
    let bad = random_string();
    let expected = format!("illegal line count -- {bad}");
    Command::cargo_bin(PRG)?
        .args(["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bytes_and_lines() -> Result<()> {
    let msg = "the argument '--lines <LINES>' cannot be used \
               with '--bytes <BYTES>'";

    Command::cargo_bin(PRG)?
        .args(["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .args([ONE, &bad, TWO])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    let output = Command::cargo_bin(PRG)?.args(args).output().expect("fail");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn empty() -> Result<()> {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n0() -> Result<()> {
    run(&[EMPTY, "-n", "0"], "tests/expected/empty.txt.n0.out")
}

#[test]
fn empty_n1() -> Result<()> {
    run(&[EMPTY, "-n", "1"], "tests/expected/empty.txt.n1.out")
}

#[test]
fn empty_n_minus_1() -> Result<()> {
    run(&[EMPTY, "-n=-1"], "tests/expected/empty.txt.n1.out")
}

#[test]
fn empty_n3() -> Result<()> {
    run(&[EMPTY, "-n", "3"], "tests/expected/empty.txt.n3.out")
}

#[test]
fn empty_n_minus_3() -> Result<()> {
    run(&[EMPTY, "-n=-3"], "tests/expected/empty.txt.n3.out")
}

#[test]
fn empty_n4() -> Result<()> {
    run(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

#[test]
fn empty_n200() -> Result<()> {
    run(&[EMPTY, "-n", "200"], "tests/expected/empty.txt.n200.out")
}

#[test]
fn empty_n_minus_200() -> Result<()> {
    run(&[EMPTY, "-n=-200"], "tests/expected/empty.txt.n200.out")
}

#[test]
fn empty_n_minus_4() -> Result<()> {
    run(&[EMPTY, "-n=-4"], "tests/expected/empty.txt.n4.out")
}

#[test]
fn empty_n_plus_0() -> Result<()> {
    run(&[EMPTY, "-n", "+0"], "tests/expected/empty.txt.n+0.out")
}

#[test]
fn empty_n_plus_1() -> Result<()> {
    run(&[EMPTY, "-n", "+1"], "tests/expected/empty.txt.n+1.out")
}

#[test]
fn empty_n_plus_2() -> Result<()> {
    run(&[EMPTY, "-n", "+2"], "tests/expected/empty.txt.n+2.out")
}

#[test]
fn empty_c3() -> Result<()> {
    run(&[EMPTY, "-c", "3"], "tests/expected/empty.txt.c3.out")
}

#[test]
fn empty_c_minus_3() -> Result<()> {
    run(&[EMPTY, "-c=-3"], "tests/expected/empty.txt.c3.out")
}

#[test]
fn empty_c8() -> Result<()> {
    run(&[EMPTY, "-c", "8"], "tests/expected/empty.txt.c8.out")
}

#[test]
fn empty_c_minus_8() -> Result<()> {
    run(&[EMPTY, "-c=8"], "tests/expected/empty.txt.c8.out")
}

#[test]
fn empty_c12() -> Result<()> {
    run(&[EMPTY, "-c", "12"], "tests/expected/empty.txt.c12.out")
}

#[test]
fn empty_c_minus_12() -> Result<()> {
    run(&[EMPTY, "-c=-12"], "tests/expected/empty.txt.c12.out")
}

#[test]
fn empty_c200() -> Result<()> {
    run(&[EMPTY, "-c", "200"], "tests/expected/empty.txt.c200.out")
}

#[test]
fn empty_c_minus_200() -> Result<()> {
    run(&[EMPTY, "-c=-200"], "tests/expected/empty.txt.c200.out")
}

#[test]
fn empty_c_plus_0() -> Result<()> {
    run(&[EMPTY, "-c", "+0"], "tests/expected/empty.txt.c+0.out")
}

#[test]
fn empty_c_plus_1() -> Result<()> {
    run(&[EMPTY, "-c", "+1"], "tests/expected/empty.txt.c+1.out")
}

#[test]
fn empty_c_plus_2() -> Result<()> {
    run(&[EMPTY, "-c", "+2"], "tests/expected/empty.txt.c+2.out")
}

// --------------------------------------------------
#[test]
fn one() -> Result<()> {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n0() -> Result<()> {
    run(&[ONE, "-n", "0"], "tests/expected/one.txt.n0.out")
}

#[test]
fn one_n1() -> Result<()> {
    run(&[ONE, "-n", "1"], "tests/expected/one.txt.n1.out")
}

#[test]
fn one_n_minus_1() -> Result<()> {
    run(&[ONE, "-n=-1"], "tests/expected/one.txt.n1.out")
}

#[test]
fn one_n3() -> Result<()> {
    run(&[ONE, "-n", "3"], "tests/expected/one.txt.n3.out")
}

#[test]
fn one_n_minus_3() -> Result<()> {
    run(&[ONE, "-n=-3"], "tests/expected/one.txt.n3.out")
}

#[test]
fn one_n4() -> Result<()> {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_n_minus_4() -> Result<()> {
    run(&[ONE, "-n=-4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_n200() -> Result<()> {
    run(&[ONE, "-n", "200"], "tests/expected/one.txt.n200.out")
}

#[test]
fn one_n_minus_200() -> Result<()> {
    run(&[ONE, "-n=-200"], "tests/expected/one.txt.n200.out")
}

#[test]
fn one_n_plus_0() -> Result<()> {
    run(&[ONE, "-n", "+0"], "tests/expected/one.txt.n+0.out")
}

#[test]
fn one_n_plus_1() -> Result<()> {
    run(&[ONE, "-n", "+1"], "tests/expected/one.txt.n+1.out")
}

#[test]
fn one_n_plus_2() -> Result<()> {
    run(&[ONE, "-n", "+2"], "tests/expected/one.txt.n+2.out")
}

#[test]
fn one_c3() -> Result<()> {
    run(&[ONE, "-c", "3"], "tests/expected/one.txt.c3.out")
}

#[test]
fn one_c_minus_3() -> Result<()> {
    run(&[ONE, "-c=-3"], "tests/expected/one.txt.c3.out")
}

#[test]
fn one_c8() -> Result<()> {
    run(&[ONE, "-c", "8"], "tests/expected/one.txt.c8.out")
}

#[test]
fn one_c_minus_8() -> Result<()> {
    run(&[ONE, "-c=8"], "tests/expected/one.txt.c8.out")
}

#[test]
fn one_c12() -> Result<()> {
    run(&[ONE, "-c", "12"], "tests/expected/one.txt.c12.out")
}

#[test]
fn one_c_minus_12() -> Result<()> {
    run(&[ONE, "-c=-12"], "tests/expected/one.txt.c12.out")
}

#[test]
fn one_c200() -> Result<()> {
    run(&[ONE, "-c", "200"], "tests/expected/one.txt.c200.out")
}

#[test]
fn one_c_minus_200() -> Result<()> {
    run(&[ONE, "-c=-200"], "tests/expected/one.txt.c200.out")
}

#[test]
fn one_c_plus_0() -> Result<()> {
    run(&[ONE, "-c", "+0"], "tests/expected/one.txt.c+0.out")
}

#[test]
fn one_c_plus_1() -> Result<()> {
    run(&[ONE, "-c", "+1"], "tests/expected/one.txt.c+1.out")
}

#[test]
fn one_c_plus_2() -> Result<()> {
    run(&[ONE, "-c", "+2"], "tests/expected/one.txt.c+2.out")
}

// --------------------------------------------------
#[test]
fn two() -> Result<()> {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n0() -> Result<()> {
    run(&[TWO, "-n", "0"], "tests/expected/two.txt.n0.out")
}

#[test]
fn two_n1() -> Result<()> {
    run(&[TWO, "-n", "1"], "tests/expected/two.txt.n1.out")
}

#[test]
fn two_n_minus_1() -> Result<()> {
    run(&[TWO, "-n=-1"], "tests/expected/two.txt.n1.out")
}

#[test]
fn two_n3() -> Result<()> {
    run(&[TWO, "-n", "3"], "tests/expected/two.txt.n3.out")
}

#[test]
fn two_n_minus_3() -> Result<()> {
    run(&[TWO, "-n=-3"], "tests/expected/two.txt.n3.out")
}

#[test]
fn two_n4() -> Result<()> {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_n_minus_4() -> Result<()> {
    run(&[TWO, "-n=-4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_n200() -> Result<()> {
    run(&[TWO, "-n", "200"], "tests/expected/two.txt.n200.out")
}

#[test]
fn two_n_minus_200() -> Result<()> {
    run(&[TWO, "-n=-200"], "tests/expected/two.txt.n200.out")
}

#[test]
fn two_n_plus_0() -> Result<()> {
    run(&[TWO, "-n", "+0"], "tests/expected/two.txt.n+0.out")
}

#[test]
fn two_n_plus_1() -> Result<()> {
    run(&[TWO, "-n", "+1"], "tests/expected/two.txt.n+1.out")
}

#[test]
fn two_n_plus_2() -> Result<()> {
    run(&[TWO, "-n", "+2"], "tests/expected/two.txt.n+2.out")
}

#[test]
fn two_c3() -> Result<()> {
    run(&[TWO, "-c", "3"], "tests/expected/two.txt.c3.out")
}

#[test]
fn two_c_minus_3() -> Result<()> {
    run(&[TWO, "-c=-3"], "tests/expected/two.txt.c3.out")
}

#[test]
fn two_c8() -> Result<()> {
    run(&[TWO, "-c", "8"], "tests/expected/two.txt.c8.out")
}

#[test]
fn two_c_minus_8() -> Result<()> {
    run(&[TWO, "-c=8"], "tests/expected/two.txt.c8.out")
}

#[test]
fn two_c12() -> Result<()> {
    run(&[TWO, "-c", "12"], "tests/expected/two.txt.c12.out")
}

#[test]
fn two_c_minus_12() -> Result<()> {
    run(&[TWO, "-c=-12"], "tests/expected/two.txt.c12.out")
}

#[test]
fn two_c200() -> Result<()> {
    run(&[TWO, "-c", "200"], "tests/expected/two.txt.c200.out")
}

#[test]
fn two_c_minus_200() -> Result<()> {
    run(&[TWO, "-c=-200"], "tests/expected/two.txt.c200.out")
}

#[test]
fn two_c_plus_0() -> Result<()> {
    run(&[TWO, "-c", "+0"], "tests/expected/two.txt.c+0.out")
}

#[test]
fn two_c_plus_1() -> Result<()> {
    run(&[TWO, "-c", "+1"], "tests/expected/two.txt.c+1.out")
}

#[test]
fn two_c_plus_2() -> Result<()> {
    run(&[TWO, "-c", "+2"], "tests/expected/two.txt.c+2.out")
}

// --------------------------------------------------
#[test]
fn three() -> Result<()> {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n0() -> Result<()> {
    run(&[THREE, "-n", "0"], "tests/expected/three.txt.n0.out")
}

#[test]
fn three_n1() -> Result<()> {
    run(&[THREE, "-n", "1"], "tests/expected/three.txt.n1.out")
}

#[test]
fn three_n_minus_1() -> Result<()> {
    run(&[THREE, "-n=-1"], "tests/expected/three.txt.n1.out")
}

#[test]
fn three_n3() -> Result<()> {
    run(&[THREE, "-n", "3"], "tests/expected/three.txt.n3.out")
}

#[test]
fn three_n_minus_3() -> Result<()> {
    run(&[THREE, "-n=-3"], "tests/expected/three.txt.n3.out")
}

#[test]
fn three_n4() -> Result<()> {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_n_minus_4() -> Result<()> {
    run(&[THREE, "-n=-4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_n200() -> Result<()> {
    run(&[THREE, "-n", "200"], "tests/expected/three.txt.n200.out")
}

#[test]
fn three_n_minus_200() -> Result<()> {
    run(&[THREE, "-n=-200"], "tests/expected/three.txt.n200.out")
}

#[test]
fn three_n_plus_0() -> Result<()> {
    run(&[THREE, "-n", "+0"], "tests/expected/three.txt.n+0.out")
}

#[test]
fn three_n_plus_1() -> Result<()> {
    run(&[THREE, "-n", "+1"], "tests/expected/three.txt.n+1.out")
}

#[test]
fn three_n_plus_2() -> Result<()> {
    run(&[THREE, "-n", "+2"], "tests/expected/three.txt.n+2.out")
}

#[test]
fn three_c3() -> Result<()> {
    run(&[THREE, "-c", "3"], "tests/expected/three.txt.c3.out")
}

#[test]
fn three_c_minus_3() -> Result<()> {
    run(&[THREE, "-c=-3"], "tests/expected/three.txt.c3.out")
}

#[test]
fn three_c8() -> Result<()> {
    run(&[THREE, "-c", "8"], "tests/expected/three.txt.c8.out")
}

#[test]
fn three_c_minus_8() -> Result<()> {
    run(&[THREE, "-c=8"], "tests/expected/three.txt.c8.out")
}

#[test]
fn three_c12() -> Result<()> {
    run(&[THREE, "-c", "12"], "tests/expected/three.txt.c12.out")
}

#[test]
fn three_c_minus_12() -> Result<()> {
    run(&[THREE, "-c=-12"], "tests/expected/three.txt.c12.out")
}

#[test]
fn three_c200() -> Result<()> {
    run(&[THREE, "-c", "200"], "tests/expected/three.txt.c200.out")
}

#[test]
fn three_c_minus_200() -> Result<()> {
    run(&[THREE, "-c=-200"], "tests/expected/three.txt.c200.out")
}

#[test]
fn three_c_plus_0() -> Result<()> {
    run(&[THREE, "-c", "+0"], "tests/expected/three.txt.c+0.out")
}

#[test]
fn three_c_plus_1() -> Result<()> {
    run(&[THREE, "-c", "+1"], "tests/expected/three.txt.c+1.out")
}

#[test]
fn three_c_plus_2() -> Result<()> {
    run(&[THREE, "-c", "+2"], "tests/expected/three.txt.c+2.out")
}

// --------------------------------------------------
#[test]
fn twelve() -> Result<()> {
    run(&[TWELVE], "tests/expected/twelve.txt.out")
}

#[test]
fn twelve_n0() -> Result<()> {
    run(&[TWELVE, "-n", "0"], "tests/expected/twelve.txt.n0.out")
}

#[test]
fn twelve_n1() -> Result<()> {
    run(&[TWELVE, "-n", "1"], "tests/expected/twelve.txt.n1.out")
}

#[test]
fn twelve_n_minus_1() -> Result<()> {
    run(&[TWELVE, "-n=-1"], "tests/expected/twelve.txt.n1.out")
}

#[test]
fn twelve_n3() -> Result<()> {
    run(&[TWELVE, "-n", "3"], "tests/expected/twelve.txt.n3.out")
}

#[test]
fn twelve_n_minus_3() -> Result<()> {
    run(&[TWELVE, "-n=-3"], "tests/expected/twelve.txt.n3.out")
}

#[test]
fn twelve_n4() -> Result<()> {
    run(&[TWELVE, "-n", "4"], "tests/expected/twelve.txt.n4.out")
}

#[test]
fn twelve_n_minus_4() -> Result<()> {
    run(&[TWELVE, "-n=-4"], "tests/expected/twelve.txt.n4.out")
}

#[test]
fn twelve_n200() -> Result<()> {
    run(&[TWELVE, "-n", "200"], "tests/expected/twelve.txt.n200.out")
}

#[test]
fn twelve_n_minus_200() -> Result<()> {
    run(&[TWELVE, "-n=-200"], "tests/expected/twelve.txt.n200.out")
}

#[test]
fn twelve_c3() -> Result<()> {
    run(&[TWELVE, "-c", "3"], "tests/expected/twelve.txt.c3.out")
}

#[test]
fn twelve_c_minus_3() -> Result<()> {
    run(&[TWELVE, "-c=-3"], "tests/expected/twelve.txt.c3.out")
}

#[test]
fn twelve_c8() -> Result<()> {
    run(&[TWELVE, "-c", "8"], "tests/expected/twelve.txt.c8.out")
}

#[test]
fn twelve_c_minus_8() -> Result<()> {
    run(&[TWELVE, "-c=8"], "tests/expected/twelve.txt.c8.out")
}

#[test]
fn twelve_c12() -> Result<()> {
    run(&[TWELVE, "-c", "12"], "tests/expected/twelve.txt.c12.out")
}

#[test]
fn twelve_c_minus_12() -> Result<()> {
    run(&[TWELVE, "-c=-12"], "tests/expected/twelve.txt.c12.out")
}

#[test]
fn twelve_c200() -> Result<()> {
    run(&[TWELVE, "-c", "200"], "tests/expected/twelve.txt.c200.out")
}

#[test]
fn twelve_c_minus_200() -> Result<()> {
    run(&[TWELVE, "-c=-200"], "tests/expected/twelve.txt.c200.out")
}

#[test]
fn twelve_n_plus_0() -> Result<()> {
    run(&[TWELVE, "-n", "+0"], "tests/expected/twelve.txt.n+0.out")
}

#[test]
fn twelve_n_plus_1() -> Result<()> {
    run(&[TWELVE, "-n", "+1"], "tests/expected/twelve.txt.n+1.out")
}

#[test]
fn twelve_n_plus_2() -> Result<()> {
    run(&[TWELVE, "-n", "+2"], "tests/expected/twelve.txt.n+2.out")
}

#[test]
fn twelve_c_plus_0() -> Result<()> {
    run(&[TWELVE, "-c", "+0"], "tests/expected/twelve.txt.c+0.out")
}

#[test]
fn twelve_c_plus_1() -> Result<()> {
    run(&[TWELVE, "-c", "+1"], "tests/expected/twelve.txt.c+1.out")
}

#[test]
fn twelve_c_plus_2() -> Result<()> {
    run(&[TWELVE, "-c", "+2"], "tests/expected/twelve.txt.c+2.out")
}

// --------------------------------------------------
#[test]
fn multiple_files() -> Result<()> {
    run(&[TWELVE, EMPTY, ONE, THREE, TWO], "tests/expected/all.out")
}

#[test]
fn multiple_files_n0() -> Result<()> {
    run(
        &["-n", "0", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n0.out",
    )
}

#[test]
fn multiple_files_n1() -> Result<()> {
    run(
        &["-n", "1", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n1.out",
    )
}

#[test]
fn multiple_files_n1_q() -> Result<()> {
    run(
        &["-n", "1", "-q", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n1.q.out",
    )
}

#[test]
fn multiple_files_n1_quiet() -> Result<()> {
    run(
        &["-n", "1", "--quiet", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n1.q.out",
    )
}

#[test]
fn multiple_files_n_minus_1() -> Result<()> {
    run(
        &["-n=-1", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n1.out",
    )
}

#[test]
fn multiple_files_n_plus_1() -> Result<()> {
    run(
        &["-n", "+1", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n+1.out",
    )
}

#[test]
fn multiple_files_n3() -> Result<()> {
    run(
        &["-n", "3", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n3.out",
    )
}

#[test]
fn multiple_files_n_minus_3() -> Result<()> {
    run(
        &["-n=-3", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n3.out",
    )
}

#[test]
fn multiple_files_n_plus_3() -> Result<()> {
    run(
        &["-n", "+3", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.n+3.out",
    )
}

#[test]
fn multiple_files_c0() -> Result<()> {
    run(
        &["-c", "0", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.c0.out",
    )
}

#[test]
fn multiple_files_c3() -> Result<()> {
    run(
        &["-c", "3", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.c3.out",
    )
}

#[test]
fn multiple_files_c_minus_3() -> Result<()> {
    run(
        &["-c=-3", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.c3.out",
    )
}

#[test]
fn multiple_files_c_plus_3() -> Result<()> {
    run(
        &["-c", "+3", TWELVE, EMPTY, ONE, THREE, TWO],
        "tests/expected/all.c+3.out",
    )
}
