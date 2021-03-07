extern crate clap;
extern crate regex;

use clap::{App, Arg};
use textwrap;
//use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: Option<String>,
    rotate: usize,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wc")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)"),
        )
        .arg(
            Arg::with_name("rotate")
                .value_name("ROTATE")
                .help("Rotate value")
                .short("r")
                .long("rotate")
                .default_value("13"),
        )
        .get_matches();

    let file = matches.value_of("file").and_then(|v| Some(v.to_string()));

    if let Some(filename) = &file {
        if let Some(error) = File::open(filename).err() {
            return Err(From::from(format!("{}: {}", filename, error)));
        }
    }

    let rotate = matches
        .value_of("rotate")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(13);

    if rotate > 26 || rotate < 1 {
        return Err(From::from(format!(
            "--rotate \"{}\" must be between 1 and 26",
            rotate
        )));
    }

    Ok(Config {
        file: file,
        rotate: rotate,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let file: Box<dyn BufRead> = match &config.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => {
            Box::new(BufReader::new(File::open(filename).unwrap()))
        }
    };

    let lines = io::BufReader::new(file).lines();
    let mut text: String = "".to_string();
    for line in lines {
        let line = line?;
        text += &line;
    }

    let rotated = rot(&text, &config.rotate);
    let chunks = rotated
        .chars()
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", textwrap::wrap(&chunks, 50).join("\n"));

    Ok(())
}

// --------------------------------------------------
fn rot(input_text: &str, rotate: &usize) -> String {
    let mut text = input_text.to_uppercase();
    let nums = [
        ("1", "ONE"),
        ("2", "TWO"),
        ("3", "THREE"),
        ("4", "FOUR"),
        ("5", "FIVE"),
        ("6", "SIX"),
        ("7", "SEVEN"),
        ("8", "EIGHT"),
        ("9", "NINE"),
        ("0", "ZERO"),
    ];
    for (numeral, number) in &nums {
        text = text.replace(numeral, number);
    }

    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let shifted = vec![
        letters[*rotate..].to_string(),
        letters[..*rotate].to_string(),
    ]
    .join("");

    let mut translate: HashMap<String, String> = HashMap::new();
    for (c1, c2) in letters.chars().zip(shifted.chars()) {
        translate.insert(c1.to_string(), c2.to_string());
    }

    text.chars()
        .filter_map(|c| translate.get(&c.to_string()))
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}

// --------------------------------------------------
#[test]
fn test_rot() {
    assert_eq!(rot("", &0), "".to_string());
    assert_eq!(rot("abc123", &0), "ABCONETWOTHREE".to_string());
}
