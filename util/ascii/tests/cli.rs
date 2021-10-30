use assert_cmd::Command;
use std::fs;

const PRG: &str = "ascii";
const EXPECTED_FILE: &str = "./tests/expected/ascii.txt";

// --------------------------------------------------
#[test]
fn runs() {
    let expected = fs::read_to_string(EXPECTED_FILE).unwrap();
    Command::cargo_bin(PRG)
        .unwrap()
        .assert()
        .success()
        .stdout(expected);
}
