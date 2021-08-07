#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};
use regex::Regex;
use std::{
    cmp::Ordering::*,
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

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
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
        quiet: matches.is_present("quiet"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match File::open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if !config.quiet && num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }

                let file = BufReader::new(file);
                if let Some(num_bytes) = config.bytes {
                    print_bytes(file, num_bytes)?;
                } else {
                    print_lines(file, config.lines)?;
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn print_bytes<T: Read + Seek>(mut file: T, num_bytes: i64) -> MyResult<()> {
    let direction = match num_bytes.cmp(&0) {
        Less => Some(SeekFrom::End(num_bytes)),
        Greater => Some(SeekFrom::Start(num_bytes as u64 - 1)),
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
fn parse_num(val: &str) -> MyResult<i64> {
    let (sign, num) = match NUM_RE.captures(val) {
        Some(caps) => (
            caps.get(1).map_or("", |c| c.as_str()),
            caps.get(2).unwrap().as_str(),
        ),
        _ => return Err(From::from(val)),
    };

    match num.parse() {
        Ok(n) => Ok(if sign == "+" { n } else { -n }),
        _ => Err(From::from(val)),
    }
}

// --------------------------------------------------
fn print_lines(mut file: impl BufRead, num_lines: i64) -> MyResult<()> {
    match num_lines.cmp(&0) {
        Greater => {
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
        }
        Less => {
            for line in last_lines(file, num_lines.abs() as usize)? {
                print!("{}", line);
            }
        }
        _ => {}
    };

    Ok(())
}

// --------------------------------------------------
fn last_lines(
    mut file: impl BufRead,
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
mod tests {
    use super::{last_lines, parse_num};
    use std::io::Cursor;

    #[test]
    fn test_parse_num() {
        let res0 = parse_num("3");
        assert!(res0.is_ok());
        assert_eq!(res0.unwrap(), -3);

        let res = parse_num("+3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 3);

        let res = parse_num("-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -3);

        let res = parse_num("3.14");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14".to_string());

        let res = parse_num("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
    }

    #[test]
    fn test_last_lines() {
        let lines = b"lorem\nipsum\r\ndolor";
        let res = last_lines(Cursor::new(lines), 1);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec!["dolor"]);

        let res = last_lines(Cursor::new(lines), 2);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec!["ipsum\r\n", "dolor"]);
    }
}
