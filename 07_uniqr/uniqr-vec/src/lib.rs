extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
//use std::iter::Peekable;

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

    let lines: Vec<String> = file
        .lines()
        .filter_map(|e| e.ok().and_then(|v| Some(v.to_string())))
        .collect();

    if lines.len() > 0 {
        let pairs = lines.windows(2);
        let num_pairs = pairs.len();

        if num_pairs == 0 {
            print(&lines[0], &1);
        } else {
            let mut count: u64 = 1;
            for (i, pair) in pairs.enumerate() {
                //dbg!(&pair);
                if let [line1, line2] = pair {
                    if line1 == line2 {
                        count += 1;
                        if i + 1 == num_pairs {
                            print(&line1, &count);
                        }
                    } else {
                        print(&line1, &count);
                        count = 1;
                    }
                }
            }
        }
    }

    Ok(())
}
