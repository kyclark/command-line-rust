#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, SeekFrom};

type MyResult<T> = Result<T, Box<dyn Error>>;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"^([+-])?(\d+)$").unwrap();
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: i64,
    bytes: Option<i64>,
    quiet: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tailr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust tail")
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
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Suppress headers"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_num)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_num)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        lines: lines.unwrap(),
        bytes: bytes,
        files: matches.values_of_lossy("files").unwrap(),
        quiet: matches.is_present("quiet"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    //println!("{:#?}", config);

    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        if !config.quiet && num_files > 1 {
            println!(
                "{}==> {} <==",
                if file_num > 0 { "\n" } else { "" },
                &filename
            );
        }

        match File::open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let file = BufReader::new(file);
                if let Some(num_bytes) = config.bytes {
                    read_bytes(file, num_bytes)?;
                } else {
                    read_lines(file, config.lines)?;
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn read_bytes<T: Read + Seek>(mut file: T, num_bytes: i64) -> MyResult<()> {
    let direction = match num_bytes.cmp(&0) {
        Ordering::Less => Some(SeekFrom::End(num_bytes)),
        Ordering::Greater => Some(SeekFrom::Start(num_bytes as u64)),
        _ => None,
    };

    if let Some(seek_from) = direction {
        if file.seek(seek_from).is_ok() {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            if !buffer.is_empty() {
                print!("{}", String::from_utf8_lossy(&buffer));
            }
        }
    }
    Ok(())
}

// --------------------------------------------------
fn read_lines<T: BufRead + Seek>(
    mut file: T,
    num_lines: i64,
) -> MyResult<()> {
    if num_lines > 0 {
        let mut line = String::new();
        let mut line_num = 0;
        loop {
            let bytes = file.read_line(&mut line)?;
            if bytes == 0 {
                break;
            }
            line_num += 1;
            if line_num >= num_lines {
                print!("{}", line);
            }
            line.clear();
        }
    } else if num_lines < 0 {
        for line in last_lines(file, num_lines.abs() as usize)? {
            print!("{}", line);
        }
    }

    Ok(())
}

// --------------------------------------------------
fn parse_num(val: &str) -> MyResult<i64> {
    let (sign, num) = match NUM_RE.captures(val) {
        Some(caps) => (
            caps.get(1).map_or("", |c| c.as_str()),
            caps.get(2).unwrap().as_str(),
        ),
        _ => return Err(From::from(val)),
    };

    match num.parse() {
        Ok(n) => Ok(if sign == "+" { n } else { -1 * n }),
        _ => Err(From::from(val)),
    }
}

// --------------------------------------------------
fn last_lines<T: BufRead>(
    mut file: T,
    num_lines: usize,
) -> MyResult<VecDeque<String>> {
    let mut last: VecDeque<String> = VecDeque::with_capacity(num_lines);
    let mut line = String::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        last.push_back(line.to_string());
        if last.len() > num_lines {
            last.pop_front();
        }
        line.clear();
    }

    Ok(last)
}

// --------------------------------------------------
#[cfg(test)]
mod test {
    use super::{last_lines, parse_num};
    use std::io::Cursor;

    #[test]
    fn test_parse_num() {
        let res0 = parse_num("3");
        assert!(res0.is_ok());
        assert_eq!(res0.unwrap(), -3);

        let res1 = parse_num("+3");
        assert!(res1.is_ok());
        assert_eq!(res1.unwrap(), 3);

        let res2 = parse_num("-3");
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap(), -3);

        let res3 = parse_num("3.14");
        assert!(res3.is_err());
        assert_eq!(res3.unwrap_err().to_string(), "3.14".to_string());

        let res4 = parse_num("foo");
        assert!(res4.is_err());
        assert_eq!(res4.unwrap_err().to_string(), "foo".to_string());
    }

    #[test]
    fn test_last_lines() {
        let lines = b"lorem\nipsum\r\ndolor";
        let res1 = last_lines(Cursor::new(lines), 1);
        assert!(res1.is_ok());
        if let Ok(vec) = res1 {
            assert_eq!(vec, vec!["dolor"]);
        }

        let res2 = last_lines(Cursor::new(lines), 2);
        assert!(res2.is_ok());
        if let Ok(vec) = res2 {
            assert_eq!(vec, vec!["ipsum\r\n", "dolor"]);
        }
    }
}
