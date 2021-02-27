extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: Option<usize>,
    bytes: Option<u64>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("head")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .value_name("BYTES")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .and_then(|s| s.trim().parse::<usize>().ok());

    let bytes = matches
        .value_of("bytes")
        .and_then(|s| s.trim().parse::<u64>().ok());

    Ok(Config {
        lines: lines,
        bytes: bytes,
        files: matches.values_of_lossy("file").unwrap(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let num_files = &config.files.len();

    for filename in &config.files {
        if num_files > &1 {
            println!("==> {} <==", &filename);
        }

        match File::open(filename) {
            Ok(file) => {
                let file = BufReader::new(file);

                if let Some(num_bytes) = config.bytes {
                    let handle = &mut file.take(num_bytes);
                    let mut buffer = String::new();
                    handle.read_to_string(&mut buffer)?;
                    println!("{}", buffer);
                } else if let Some(num_lines) = config.lines {
                    for line in file.lines().take(num_lines) {
                        println!("{}", line?.trim());
                    }
                }
            }
            Err(err) => println!("{}: {}", filename, err),
        }
    }

    Ok(())
}
