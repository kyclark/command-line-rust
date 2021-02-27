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
}

#[derive(Debug)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
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
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;

    for filename in &config.files {
        match count(&filename) {
            Ok(info) => {
                println!(
                    "{:8}{:8}{:8} {}",
                    info.num_lines, info.num_words, info.num_bytes, &filename
                );
                total_lines += info.num_lines;
                total_words += info.num_words;
                total_bytes += info.num_bytes;
            }
            Err(err) => println!("{}: {}", &filename, err),
        }
    }

    if config.files.len() > 1 {
        println!("{:8}{:8}{:8} total", total_lines, total_words, total_bytes);
    }

    //let info: Vec<MyResult<FileInfo>> =
    //    config.files.into_iter().map(|f| count(&f)).collect();

    //println!("{:?}", info);

    Ok(())
}

// --------------------------------------------------
pub fn count(filename: &str) -> MyResult<FileInfo> {
    //let mut file = File::open(filename)?;
    //let mut contents = String::new();
    //file.read_to_string(&mut contents)?;
    //println!("{}", &contents);

    let file = File::open(filename)?;
    let file = BufReader::new(file);
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;

    for line in file.lines() {
        let line = line?;
        num_lines += 1;
        num_words += line
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>()
            .len();
        num_bytes += line.chars().count() + 1;
    }

    Ok(FileInfo {
        num_lines: num_lines,
        num_words: num_words,
        num_bytes: num_bytes,
    })
}
