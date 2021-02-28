extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: String,
    files: Vec<String>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("grep")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust grep")
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    Ok(Config {
        pattern: matches.value_of("pattern").unwrap().to_string(),
        files: matches.values_of_lossy("file").unwrap(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let re = Regex::new(&config.pattern).unwrap();

    for filename in &config.files {
        let file = File::open(filename)?;
        let file = BufReader::new(file);

        // for line in file.lines().filter(|l| re.is_match(l)) {
        for line in file.lines() {
            let line = line?;
            if re.is_match(&line) {
                println!("{}: {}", &filename, &line);
            }
        }
    }

    Ok(())
}
