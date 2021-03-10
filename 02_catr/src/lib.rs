extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wc")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .value_name("NUMBER")
                .help("Number lines")
                .short("n")
                .long("num")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    // Cf https://stackoverflow.com/questions/36088116/
    // how-to-do-polymorphic-io-from-either-a-file-or-stdin-in-rust/49964042
    for filename in &config.files {
        let file: Box<dyn BufRead> = match filename == "-" {
            true => Box::new(BufReader::new(io::stdin())),
            _ => Box::new(BufReader::new(File::open(filename).unwrap())),
        };

        for (i, line) in file.lines().enumerate() {
            let line = line?;
            if config.number_lines {
                println!("{:8}: {}", i + 1, &line);
            } else {
                println!("{}", &line);
            }
        }
    }

    Ok(())
}
