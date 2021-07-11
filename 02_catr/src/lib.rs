use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .help("Number lines")
                .short("n")
                .long("number")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .help("Number non-blank lines")
                .short("b")
                .long("number-nonblank")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

// --------------------------------------------------
//pub fn run(config: Config) -> MyResult<()> {
//    for filename in config.files {
//        let file: MyResult<Box<dyn BufRead>> = match filename.as_str() {
//            "-" => Ok(Box::new(BufReader::new(io::stdin()))),
//            _ => match File::open(&filename) {
//                Ok(file) => Ok(Box::new(BufReader::new(file))),
//                Err(e) => Err(From::from(format!("{}: {}", &filename, e))),
//            },
//        };

//        if let Err(e) = file {
//            eprintln!("{}", e);
//            continue;
//        }

//        let mut last_num = 0;
//        for (line_num, line_result) in file.unwrap().lines().enumerate() {
//            let line = line_result?;
//            if config.number_lines {
//                println!("{:6}\t{}", line_num + 1, line);
//            } else if config.number_nonblank_lines {
//                if !line.is_empty() {
//                    last_num += 1;
//                    println!("{:6}\t{}", last_num, line);
//                } else {
//                    println!();
//                }
//            } else {
//                println!("{}", line);
//            }
//        }
//    }

//    Ok(())
//}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        if let Err(e) =
            cat(&filename, config.number_lines, config.number_nonblank_lines)
        {
            eprintln!("{}: {}", filename, e);
        }
    }

    Ok(())
}

// --------------------------------------------------
fn cat(
    filename: &str,
    number_lines: bool,
    number_nonblank_lines: bool,
) -> MyResult<()> {
    let file: Box<dyn BufRead> = match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(&filename)?)),
    };

    let mut last_num = 0;
    for (line_num, line_result) in file.lines().enumerate() {
        let line = line_result?;
        if number_lines {
            println!("{:6}\t{}", line_num + 1, line);
        } else if number_nonblank_lines {
            if !line.is_empty() {
                last_num += 1;
                println!("{:6}\t{}", last_num, line);
            } else {
                println!();
            }
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
