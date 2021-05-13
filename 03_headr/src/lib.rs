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
                .min_values(1),
        )
        .get_matches();

    let lines = parse_int(matches.value_of("lines"));
    if let Err(bad_lines) = lines {
        return Err(From::from(format!(
            "illegal line count -- {}",
            bad_lines
        )));
    }

    let bytes = parse_int(matches.value_of("bytes"));
    if let Err(bad_bytes) = bytes {
        return Err(From::from(format!(
            "illegal byte count -- {}",
            bad_bytes
        )));
    }

    Ok(Config {
        lines: lines?.unwrap(),
        bytes: bytes?,
        files: matches.values_of_lossy("files").unwrap(),
    })
}

// --------------------------------------------------
fn parse_int(val: Option<&str>) -> MyResult<Option<usize>> {
    match val {
        Some(v) => match v.trim().parse::<core::num::NonZeroUsize>() {
            Ok(n) => Ok(Some(usize::from(n))),
            Err(_) => Err(From::from(v.to_string())),
        },
        None => Ok(None),
    }
}

// --------------------------------------------------
#[test]
fn test_parse_int() {
    // 3 is an OK integer
    let t1 = parse_int(&Some("3"));
    assert!(t1.is_ok());
    assert_eq!(t1.unwrap(), Some(3usize));

    // No value is OK
    let t2 = parse_int(&None);
    assert!(t2.is_ok());
    assert!(t2.unwrap().is_none());

    // Any string is an error
    let t3 = parse_int(&Some("foo"));
    assert!(t3.is_err());
    if let Err(e3) = t3 {
        assert_eq!(e3.to_string(), "foo".to_string());
    }

    // A zero is an error
    let t4 = parse_int(&Some("0"));
    assert!(t4.is_err());
    if let Err(e4) = t4 {
        assert_eq!(e4.to_string(), "0".to_string());
    }

    // A negative number is an error
    let t5 = parse_int(&Some("-1"));
    assert!(t5.is_err());
    if let Err(e5) = t5 {
        assert_eq!(e5.to_string(), "-1".to_string());
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
