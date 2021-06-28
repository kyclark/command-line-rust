extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
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
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("insensitive")
                .value_name("INSENSITIVE")
                .help("Case-insensitive")
                .short("i")
                .long("--insensitive")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("recursive")
                .value_name("RECURSIVE")
                .help("Recursive search")
                .short("r")
                .long("--recursive")
                .takes_value(false),
        )
        .get_matches();

    let pattern = matches.value_of("pattern").unwrap().to_string();
    let re = RegexBuilder::new(&pattern)
        .case_insensitive(matches.is_present("insensitive"))
        .build();

    if re.is_err() {
        return Err(From::from(format!("Invalid pattern \"{}\"", pattern)));
    }

    Ok(Config {
        pattern: re?,
        files: matches.values_of_lossy("files").unwrap(),
        recursive: matches.is_present("recursive"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut files: Vec<String> = vec![];
    if config.recursive {
        for file in &config.files {
            if let Ok(mut f) = find_files(file) {
                files.append(&mut f);
            }
        }
    } else {
        files = config.files;
    }

    for filename in &files {
        let file = File::open(filename)?;
        let file = BufReader::new(file);

        for line in file.lines() {
            if let Ok(line) = line {
                if config.pattern.is_match(&line) {
                    println!("{}:{}", &filename, &line);
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn find_files(path: &String) -> MyResult<Vec<String>> {
    let mut files: Vec<String> = vec![];
    let metadata = fs::metadata(&path)?;
    if metadata.is_dir() {
        let walker = WalkDir::new(path).into_iter();
        for entry in walker.filter_map(|e| e.ok()) {
            if &entry.path().display().to_string() != path {
                if let Ok(mut more) =
                    find_files(&entry.path().display().to_string())
                {
                    files.append(&mut more);
                }
            }
            //let meta = fs::metadata(&entry)?;
            //if meta.is_file() {
            //    files.push(entry.path().display().to_string());
            //}
            //else if meta.is_dir() {
            //}
        }
    } else if metadata.is_file() {
        files.push(path.to_string());
    }

    Ok(files)
}
