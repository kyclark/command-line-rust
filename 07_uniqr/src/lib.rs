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
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniq")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .help("Show counts")
                .short("c")
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches
            .value_of("in_file")
            .and_then(|v| Some(v.to_string()))
            .unwrap(),
        out_file: matches
            .value_of("out_file")
            .and_then(|v| Some(v.to_string())),
        count: matches.is_present("count"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut file: Box<dyn BufRead> = match config.in_file.as_str() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(&config.in_file)?)),
    };

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(&out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: &u64, line: &String| -> MyResult<()> {
        if count > &0 {
            if config.count {
                write!(out_file, "{:4} {}", &count, &line)?;
            } else {
                write!(out_file, "{}", &line)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut last = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if &line.trim_end() != &last.trim_end() {
            print(&count, &last)?;
            last = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }
    print(&count, &last)?;

    Ok(())
}
