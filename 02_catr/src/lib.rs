use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

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
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .help("Number lines")
                .short("n")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .help("Number non-blank lines")
                .short("b")
                .takes_value(false),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();
    //for file in &files {
    //    if file != "-" {
    //        if !Path::new(&file).exists() {
    //            return Err(From::from(format!(
    //                "\"{}\" is not a valid file.",
    //                file
    //            )));
    //        }
    //    }
    //}

    //for file in files.iter().filter(|file| file != &"-") {
    //    if !Path::new(&file).exists() {
    //        return Err(From::from(format!(
    //            "\"{}\" is not a valid file.",
    //            file
    //        )));
    //    }
    //}

    for file in files
        .iter()
        .filter(|file| file != &"-" && !Path::new(&file).exists())
    {
        return Err(From::from(format!("\"{}\" is not a valid file.", file)));
    }

    Ok(Config {
        files: files,
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    // Cf https://stackoverflow.com/questions/36088116/
    // how-to-do-polymorphic-io-from-either-a-file-or-stdin-in-rust/49964042
    for filename in config.files {
        //let file = match filename.as_str() {
        //    "-" => BufReader::new(io::stdin()),
        //    _ => BufReader::new(File::open(filename)?),
        //};

        let file: Box<dyn BufRead> = match filename.as_str() {
            "-" => Box::new(BufReader::new(io::stdin())),
            _ => Box::new(BufReader::new(File::open(filename)?)),
        };

        let mut last_num = 0;
        for (line_num, line_result) in file.lines().enumerate() {
            let line = line_result?;
            if config.number_lines {
                println!("{:6}\t{}", line_num + 1, line);
            } else if config.number_nonblank_lines {
                if line.len() > 0 {
                    last_num += 1;
                    println!("{:6}\t{}", last_num, line);
                } else {
                    println!("");
                }
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
