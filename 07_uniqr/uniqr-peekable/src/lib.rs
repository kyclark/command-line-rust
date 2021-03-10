extern crate clap;

use clap::{App, Arg};
//use itertools::Itertools;
//use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: Option<String>,
    count: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniq")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)"),
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .help("Show counts")
                .short("c")
                .long("counts")
                .takes_value(false),
        )
        .get_matches();

    let file = matches.value_of("file").and_then(|v| Some(v.to_string()));

    if let Some(filename) = &file {
        if let Some(error) = File::open(filename).err() {
            return Err(From::from(format!("{}: {}", filename, error)));
        }
    }

    Ok(Config {
        file: file,
        count: matches.is_present("count"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let file: Box<dyn BufRead> = match &config.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => {
            Box::new(BufReader::new(File::open(filename).unwrap()))
        }
    };

    let print = |line: &String, count: &u64| {
        if line.len() > 0 && count > &0 {
            if config.count {
                println!("{:4} {}", &count, &line);
            } else {
                println!("{}", &line);
            }
        }
    };

    let mut count: u64 = 1;
    let mut iter = io::BufReader::new(file).lines().peekable();
    loop {
        let line = iter.next();
        if line.is_none() {
            break;
        }

        let line = line.unwrap();
        let line = line?;

        if let Some(next_line) = iter.peek() {
            if let Ok(val) = next_line {
                if &line == val {
                    count += 1;
                } else {
                    print(&line, &count);
                    count = 1;
                }
            }
        } else {
            print(&line, &count);
            break;
        }
    }

    Ok(())
}
