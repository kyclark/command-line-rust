extern crate clap;
extern crate inflector;
extern crate regex;

// Cf https://github.com/huggingface/tokenizers/tree/master/tokenizers

#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};
use inflector::cases::titlecase::{is_title_case, to_title_case};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use unicode_segmentation::UnicodeSegmentation;

type MyResult<T> = Result<T, Box<dyn Error>>;

lazy_static! {
    static ref QU_WORD: Regex = Regex::new(r"^([Qq]u)(.+)").unwrap();
    static ref CONSONANT_WORD: Regex = Regex::new(
        r"^([BCDFGHJKLMNPQRSTVWXYZbcdfghjklmnpqrstvwxyz]+)([AEIOUaeiou].*)",
    )
    .unwrap();
    static ref IS_WORD: Regex = Regex::new(r"^\w+$").unwrap();
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("piggy")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust piggy")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        let file = File::open(filename)?;
        let file = BufReader::new(file);

        for line in file.lines() {
            let line = line?;
            println!(
                "{}",
                // https://stackoverflow.com/questions/58897146/
                // using-iter-map-why-does-a-closure-work-but-
                // passing-the-function-directly-does-n
                //split(&line).iter().map(|w| pig(w)).collect::<String>()
                split(&line).iter().map(pig).collect::<String>()
            );
        }
    }
    Ok(())
}

// --------------------------------------------------
fn split<'a>(text: &'a str) -> Vec<&'a str> {
    text.split_word_bounds().collect::<Vec<&str>>()
}

// --------------------------------------------------
#[test]
fn test_split() {
    assert_eq!(
        split("He said, \"I'd like to eat cake!\""),
        vec![
            "He", " ", "said", ",", " ", "\"", "I\'d", " ", "like", " ",
            "to", " ", "eat", " ", "cake", "!", "\""
        ]
    );
}

// --------------------------------------------------
fn pig<S: AsRef<str>>(word: S) -> String {
    //lazy_static! {
    //    static ref QU_WORD: Regex = Regex::new(r"^([Qq]u)(.+)").unwrap();
    //    static ref CONSONANT_WORD: Regex = Regex::new(
    //        r"^([BCDFGHJKLMNPQRSTVWXYZbcdfghjklmnpqrstvwxyz]+)([AEIOUaeiou].*)"
    //    )
    //    .unwrap();
    //    static ref IS_WORD: Regex = Regex::new(r"^\w+$").unwrap();
    //};

    if IS_WORD.is_match(word.as_ref()) {
        if let Some(caps) = QU_WORD.captures(&word.as_ref()) {
            let qu = &caps.get(1).unwrap().as_str();
            let rest = &caps.get(2).unwrap().as_str();

            if is_title_case(qu) {
                format!("{}-{}ay", to_title_case(&rest), &qu.to_lowercase())
            } else {
                format!("{}-{}ay", &rest, &qu)
            }
        } else if let Some(caps) = CONSONANT_WORD.captures(&word.as_ref()) {
            let consonants = &caps.get(1).unwrap().as_str();
            let rest = &caps.get(2).unwrap().as_str();

            if is_title_case(consonants) {
                format!(
                    "{}-{}ay",
                    to_title_case(&rest),
                    &consonants.to_lowercase()
                )
            } else {
                format!("{}-{}ay", &rest, &consonants)
            }
        } else {
            word.as_ref().to_string() + &"-yay".to_string()
        }
    } else {
        word.as_ref().to_string()
    }
}

// --------------------------------------------------
#[test]
fn test_pig() {
    assert_eq!(pig(" "), " ".to_string());
    assert_eq!(pig("\n"), "\n".to_string());
    assert_eq!(pig("a"), "a-yay".to_string());
    assert_eq!(pig("A"), "A-yay".to_string());
    assert_eq!(pig("i"), "i-yay".to_string());
    assert_eq!(pig("apple"), "apple-yay".to_string());
    assert_eq!(pig("Apple"), "Apple-yay".to_string());
    assert_eq!(pig("cat"), "at-cay".to_string());
    assert_eq!(pig("Cat"), "At-cay".to_string());
    assert_eq!(pig("chair"), "air-chay".to_string());
    assert_eq!(pig("Chair"), "Air-chay".to_string());
    assert_eq!(pig("the"), "e-thay".to_string());
    assert_eq!(pig("The"), "E-thay".to_string());
    assert_eq!(pig("quick"), "ick-quay".to_string());
    assert_eq!(pig("Quick"), "Ick-quay".to_string());
}
