use clap::{App, Arg};
use regex::RegexBuilder;
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: String,
    files: Vec<String>,
    recursive: bool,
    insensitive: bool,
    count: bool,
    invert_match: bool,
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
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .help("Count occurrences")
                .short("c")
                .long("count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("invert")
                .value_name("INVERT")
                .help("Invert match")
                .short("v")
                .long("invert_match")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        pattern: matches.value_of("pattern").unwrap().to_string(),
        files: matches.values_of_lossy("files").unwrap(),
        recursive: matches.is_present("recursive"),
        insensitive: matches.is_present("insensitive"),
        count: matches.is_present("count"),
        invert_match: matches.is_present("invert"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    // println!("{:#?}", config);

    let pattern = RegexBuilder::new(&config.pattern)
        .case_insensitive(config.insensitive)
        .build()
        .map_err(|_| format!("Invalid pattern \"{}\"", &config.pattern))?;

    let files = find_files(&config)?;
    let num_files = &files.len();
    let print = |file: &str, val: &str| {
        if num_files > &1 {
            println!("{}:{}", file, val);
        } else {
            println!("{}", val);
        }
    };

    for filename in &files {
        if fs::metadata(&filename).ok().map_or(false, |v| v.is_dir()) {
            eprintln!("{} is a directory", filename);
            continue;
        };

        let file: Box<dyn BufRead> = match filename.as_str() {
            "-" => Box::new(BufReader::new(io::stdin())),
            _ => Box::new(BufReader::new(
                File::open(filename)
                    .map_err(|e| format!("{}: {}", filename, e))?,
            )),
        };

        let matches: Vec<String> = file
            .lines()
            .map(|line| line.ok().unwrap())
            .filter(|line| {
                (pattern.is_match(&line) && !config.invert_match)
                    || (!pattern.is_match(&line) && config.invert_match)
            })
            .collect();

        if config.count {
            print(&filename, &matches.len().to_string());
        } else {
            for line in matches {
                print(&filename, &line);
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
