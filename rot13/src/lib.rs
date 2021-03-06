extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: Option<String>,
    rotate: u32,
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
        .and_then(|v| v.parse::<u32>().ok());

    Ok(Config {
        file: file,
        rotate: rotate.unwrap_or(13),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    let file: Box<dyn BufRead> = match &config.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => {
            Box::new(BufReader::new(File::open(filename).unwrap()))
        }
    };

    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line?;
        println!("{}", rot(&line, &config.rotate));
    }

    Ok(())
}

// --------------------------------------------------
fn rot(input_text: &str, rotate: &u32) -> String {
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
    let re = Regex::new(r"[^A-Z]").unwrap();
    re.replace_all(&text, "").to_string()
}

// --------------------------------------------------
#[test]
fn test_rot() {
    assert_eq!(rot("", &0), "".to_string());
    assert_eq!(rot("abc123", &0), "ABCONETWOTHREE".to_string());
}
