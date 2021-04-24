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
    lines: usize,
    bytes: Option<usize>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
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
                .takes_value(true)
                .conflicts_with("lines")
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

    let lines = parse_int(&matches, "lines")?;
    let bytes = parse_int(&matches, "bytes")?;

    Ok(Config {
        lines: lines.unwrap(),
        bytes: bytes,
        files: matches.values_of_lossy("file").unwrap(),
    })
}

// --------------------------------------------------
fn parse_int(
    matches: &clap::ArgMatches,
    arg_name: &str,
) -> MyResult<Option<usize>> {
    match matches.value_of(arg_name) {
        Some(v) => match v.trim().parse::<core::num::NonZeroUsize>() {
            Ok(n) => Ok(Some(usize::from(n))),
            //Ok(n) => match n > 0 {
            //    true => Ok(Some(n)),
            //    _ => Err(From::from(format!(
            //        "1. illegal {} count: {}",
            //        arg_name, n
            //    ))),
            //},
            Err(_) => {
                Err(From::from(format!("illegal {} count: {}", arg_name, v)))
            }
        },
        None => Ok(None),
    }
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    let print_separators = num_files > 1;

    for (file_num, filename) in config.files.iter().enumerate() {
        if print_separators {
            println!("==> {} <==", &filename);
        }

        match File::open(filename) {
            Ok(file) => {
                let mut file = BufReader::new(file);

                if let Some(num_bytes) = config.bytes {
                    let handle = &mut file.take(num_bytes as u64);
                    let mut buffer = String::new();
                    handle.read_to_string(&mut buffer)?;
                    if buffer.len() > 0 {
                        print!("{}", buffer);
                    }
                } else {
                    // Doesn't work, strips line ending.
                    //for line in file.lines().take(config.lines) {
                    //    println!("{}", line?.trim());
                    //}
                    let mut line = String::new();
                    let mut line_num = 0;
                    loop {
                        if line_num == config.lines {
                            break;
                        }
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line_num += 1;
                        line.clear();
                    }
                }
            }
            Err(err) => eprintln!("{}: {}", filename, err),
        }

        if print_separators && file_num + 1 != num_files {
            println!("");
        }
    }

    Ok(())
}
