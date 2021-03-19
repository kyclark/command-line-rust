extern crate clap;

use clap::{App, Arg};
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Cursor, SeekFrom};
use std::str;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: Option<usize>,
    bytes: Option<i64>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tail")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust tail")
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
        .and_then(|s| s.trim().parse::<i64>().ok());

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
                let mut file = BufReader::new(file);

                if let Some(num_bytes) = config.bytes {
                    file.seek(SeekFrom::End(num_bytes * -1))?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    print!("{}", str::from_utf8(&buffer)?);
                } else if let Some(num_lines) = config.lines {
                    for line in take_lines(file, num_lines)? {
                        println!("{}", line);
                    }
                }
            }
            Err(err) => println!("{}: {}", filename, err),
        }
    }

    Ok(())
}

// --------------------------------------------------
fn take_lines<T: BufRead>(file: T, num: usize) -> MyResult<VecDeque<String>> {
    let mut last: VecDeque<String> = VecDeque::with_capacity(num);
    for line in file.lines() {
        let line = line?;
        last.push_back(line.to_string());
        if last.len() > num {
            last.pop_front();
        }
    }

    Ok(last)
}

// --------------------------------------------------
#[test]
fn test_take_lines() {
    let lines1 = Cursor::new(b"lorem\nipsum\r\ndolor");
    let res1 = take_lines(lines1, 1);
    assert!(res1.is_ok());
    if let Ok(vec) = res1 {
        assert_eq!(vec, vec!["dolor"]);
    }

    let lines2 = Cursor::new(b"lorem\nipsum\r\ndolor");
    let res2 = take_lines(lines2, 2);
    assert!(res2.is_ok());
    if let Ok(vec) = res2 {
        assert_eq!(vec, vec!["ipsum", "dolor"]);
    }
}
