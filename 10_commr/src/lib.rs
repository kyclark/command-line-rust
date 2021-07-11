use clap::{App, Arg};
use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

enum Column {
    Col1(String),
    Col2(String),
    Col3(String),
}

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    suppress_col1: bool,
    suppress_col2: bool,
    suppress_col3: bool,
    insensitive: bool,
    delimiter: String,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("commr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust comm")
        .arg(
            Arg::with_name("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("suppress_col1")
                .short("1")
                .value_name("COL1")
                .takes_value(false)
                .help("Suppress printing of column 1"),
        )
        .arg(
            Arg::with_name("suppress_col2")
                .short("2")
                .value_name("COL2")
                .takes_value(false)
                .help("Suppress printing of column 2"),
        )
        .arg(
            Arg::with_name("suppress_col3")
                .short("3")
                .value_name("COL3")
                .takes_value(false)
                .help("Suppress printing of column 3"),
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .value_name("INSENSITIVE")
                .takes_value(false)
                .help("Case insensitive comparison of lines"),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        file1: matches.value_of("file1").unwrap().to_string(),
        file2: matches.value_of("file2").unwrap().to_string(),
        suppress_col1: matches.is_present("suppress_col1"),
        suppress_col2: matches.is_present("suppress_col2"),
        suppress_col3: matches.is_present("suppress_col3"),
        insensitive: matches.is_present("insensitive"),
        delimiter: matches
            .value_of("delimiter")
            .map(|v| v.to_string())
            .or_else(|| Some("\t".to_string()))
            .unwrap(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let filename1 = &config.file1;
    let filename2 = &config.file2;

    if filename1.as_str() == "-" && filename2.as_str() == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let mut lines1 = open(filename1)?
        .lines()
        .filter_map(|line| line.ok())
        .map(case);

    let mut lines2 = open(filename2)?
        .lines()
        .filter_map(|line| line.ok())
        .map(case);

    let printer = |col: Column| {
        let default_col1 = if config.suppress_col1 {
            ""
        } else {
            &config.delimiter
        };
        let default_col2 = if config.suppress_col2 {
            ""
        } else {
            &config.delimiter
        };

        let out = match col {
            Column::Col1(val) => {
                if config.suppress_col1 {
                    "".to_string()
                } else {
                    val
                }
            }
            Column::Col2(val) => format!(
                "{}{}",
                default_col1,
                if config.suppress_col2 { "" } else { &val },
            ),
            Column::Col3(val) => format!(
                "{}{}{}",
                default_col1,
                default_col2,
                if config.suppress_col3 { "" } else { &val },
            ),
        };

        if !out.trim().is_empty() {
            println!("{}", out);
        }
    };

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    loop {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Ordering::Equal => {
                    printer(Column::Col3(val1.to_string()));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Ordering::Less => {
                    printer(Column::Col1(val1.to_string()));
                    line1 = lines1.next();
                }
                _ => {
                    printer(Column::Col2(val2.to_string()));
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                printer(Column::Col1(val1.to_string()));
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                printer(Column::Col2(val2.to_string()));
                line2 = lines2.next();
            }
            (None, None) => break,
        }
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename)
                .map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}
