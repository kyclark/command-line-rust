use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("grepr")
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
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("insensitive")
                .value_name("INSENSITIVE")
                .help("Case-insensitive")
                .short("i")
                .long("insensitive")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("recursive")
                .value_name("RECURSIVE")
                .help("Recursive search")
                .short("r")
                .long("recursive")
                .takes_value(false),
        )
        .get_matches();

    let pattern = matches.value_of("pattern").unwrap();
    let re = RegexBuilder::new(&pattern)
        .case_insensitive(matches.is_present("insensitive"))
        .build();

    if re.is_err() {
        return Err(From::from(format!("Invalid pattern \"{}\"", pattern)));
    }

    Ok(Config {
        pattern: re.unwrap(),
        files: matches.values_of_lossy("files").unwrap(),
        recursive: matches.is_present("recursive"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config)?;
    let num_files = &files.len();

    for filename in &files {
        let file: Box<dyn BufRead> = match filename.as_str() {
            "-" => Box::new(BufReader::new(io::stdin())),
            _ => match fs::metadata(&filename)?.is_dir() {
                true => Err(format!("{} is a directory", filename)),
                _ => Box::new(BufReader::new(
                    File::open(filename)
                        .map_err(|e| format!("{}: {}", filename, e))?,
                )),
            },
        };

        for line in file.lines() {
            let line = line?;
            if config.pattern.is_match(&line) {
                if num_files > &1 {
                    println!("{}:{}", &filename, &line);
                } else {
                    println!("{}", &line);
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn find_files(config: &Config) -> MyResult<Vec<String>> {
    let mut files: Vec<String> = vec![];
    if config.recursive {
        for path in &config.files {
            let metadata = fs::metadata(&path)?;
            if metadata.is_dir() {
                for entry in WalkDir::new(path) {
                    let entry = entry?;
                    if entry.file_type().is_file() {
                        files.push(entry.path().display().to_string());
                    }
                }
            } else if metadata.is_file() {
                files.push(path.to_string());
            }
        }
    } else {
        files = config.files.iter().map(|v| v.clone()).collect();
    }

    Ok(files)
}
