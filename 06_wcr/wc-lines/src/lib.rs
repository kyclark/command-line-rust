use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    words: bool,
    chars: bool,
    lines: bool,
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
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .help("Show word count")
                .takes_value(false)
                .short("w")
                .long("words"),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Show chars/bytes count")
                .takes_value(false)
                .short("c")
                .long("chars"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Show line count")
                .takes_value(false)
                .short("l")
                .long("lines"),
        )
        .get_matches();

    let mut words = matches.is_present("words");
    let mut chars = matches.is_present("chars");
    let mut lines = matches.is_present("lines");
    let opts = vec![words, chars, lines];

    if opts.iter().all(|v| v == &false) {
        words = true;
        chars = true;
        lines = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        words: words,
        chars: chars,
        lines: lines,
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
                    "{}{}{} {}",
                    format_field(&info.num_lines, &config.lines),
                    format_field(&info.num_words, &config.words),
                    format_field(&info.num_bytes, &config.chars),
                    &filename
                );
                total_lines += info.num_lines;
                total_words += info.num_words;
                total_bytes += info.num_bytes;
            }
            Err(err) => println!("{}: {}", &filename, err),
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{} total",
            format_field(&total_lines, &config.lines),
            format_field(&total_words, &config.words),
            format_field(&total_bytes, &config.chars)
        );
    }

    Ok(())
}

// --------------------------------------------------
fn format_field(value: &usize, show: &bool) -> String {
    if *show {
        format!("{:8}", value)
    } else {
        "".to_string()
    }
}

// --------------------------------------------------
pub fn count(filename: &str) -> MyResult<FileInfo> {
    let file = File::open(filename)?;
    let mut file = BufReader::new(file);
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;

    let mut line = String::new();
    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        num_words += line
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>()
            .len();
        num_bytes += line_bytes;
        line.clear();
    }

    Ok(FileInfo {
        num_lines: num_lines,
        num_words: num_words,
        num_bytes: num_bytes,
    })
}
