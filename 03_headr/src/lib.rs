use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
//use std::str::FromStr;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: u64,
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
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .get_matches();

    let lines = parse_int(matches.value_of("lines"))
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = parse_int(matches.value_of("bytes"))
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        lines: lines.unwrap() as u64,
        bytes: bytes,
        files: matches.values_of_lossy("files").unwrap(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Ok(file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let n = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..n]));
                } else {
                    let mut file = BufReader::new(file);
                    let mut line = String::new();
                    //let mut line_num = 0;
                    for line_num in 0.. {
                        if line_num == config.lines {
                            break;
                        }
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        //line_num += 1;
                        line.clear();
                    }
                }
            }
            Err(err) => eprintln!("{}: {}", filename, err),
        }
    }
    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// --------------------------------------------------
fn parse_int(val: Option<&str>) -> MyResult<Option<usize>> {
    match val {
        //Some(v) => match core::num::NonZeroUsize::from_str(v.trim()) {
        Some(v) => match v.trim().parse::<core::num::NonZeroUsize>() {
            Ok(n) => Ok(Some(usize::from(n))),
            Err(_) => Err(From::from(v)),
        },
        None => Ok(None),
    }
}

// --------------------------------------------------
#[test]
fn test_parse_int() {
    // No value is OK
    let res1 = parse_int(None);
    assert!(res1.is_ok());
    assert!(res1.unwrap().is_none());

    // 3 is an OK integer
    let res2 = parse_int(Some("3"));
    assert!(res2.is_ok());
    assert_eq!(res2.unwrap(), Some(3usize));

    // Any string is an error
    let res3 = parse_int(Some("foo"));
    assert!(res3.is_err());
    if let Err(e) = res3 {
        assert_eq!(e.to_string(), "foo".to_string());
    }

    // A zero is an error
    let res4 = parse_int(Some("0"));
    assert!(res4.is_err());
    if let Err(e) = res4 {
        assert_eq!(e.to_string(), "0".to_string());
    }
}
