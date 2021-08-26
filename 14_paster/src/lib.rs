use clap::{App, Arg};
use std::error::Error;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiters: Vec<String>,
    serial: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("paster")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust paste")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("delimiters")
                .short("d")
                .long("delimiter")
                .value_name("DELIMITER")
                .help("Delimiter")
                .default_value("\\t")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("serial")
                .short("s")
                .long("serial")
                .help("Concatenate lines of each file serially")
                .takes_value(false),
        )
        .get_matches();

    let delimiters = matches
        .value_of("delimiters")
        .map(parse_delimiters)
        .transpose()?
        .unwrap();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        delimiters,
        serial: matches.is_present("serial"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    //println!("{:#?}", config);

    let mut files = vec![];
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let lines = file.lines().filter_map(|line| line.ok());
                files.push(lines);
            }
        }
    }

    if config.serial {
        let delims = &mut config.delimiters.into_iter().cycle();
        for file in files {
            let mut out = String::new();
            for (i, line) in file.enumerate() {
                if i > 0 {
                    out += &delims.next().unwrap();
                }
                out += &line;
            }
            println!("{}", out);
        }
    } else {
        let delim = config.delimiters.first().unwrap();
        loop {
            let mut lines = vec![];
            for iter in &mut files {
                lines.push(iter.next().unwrap_or_else(|| "".to_string()));
            }
            if lines.join("").is_empty() {
                break;
            }
            println!("{}", lines.join(&delim));
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
fn parse_delimiters(given: &str) -> MyResult<Vec<String>> {
    let mut chrs = given.chars().collect::<Vec<_>>().into_iter().peekable();
    let mut results = vec![];
    while let Some(chr) = chrs.next() {
        if chr == '\\' {
            let esc = match &chrs.peek() {
                None => Err("Lone backslash".to_string()),
                Some('n') => {
                    chrs.next();
                    Ok("\n".to_string())
                }
                Some('t') => {
                    chrs.next();
                    Ok("\t".to_string())
                }
                Some('\\') => {
                    chrs.next();
                    Ok("\\".to_string())
                }
                Some('0') => {
                    chrs.next();
                    Ok("".to_string())
                }
                Some(c) => Err(format!("Unknown escape \"\\{}\"", c)),
            }?;
            results.push(esc);
        } else {
            results.push(chr.to_string());
        }
    }
    Ok(results)
}

// --------------------------------------------------
#[cfg(test)]
mod test {
    use super::parse_delimiters;

    #[test]
    fn test_parse_delimiters() {
        let res = parse_delimiters("\\");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Lone backslash");

        let res = parse_delimiters("\\x");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Unknown escape \"\\x\"");

        let res = parse_delimiters(",");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &[","]);

        let res = parse_delimiters("\\t");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &["\t"]);

        let res = parse_delimiters("\\n");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &["\n"]);

        let res = parse_delimiters("\\\\");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &["\\"]);

        let res = parse_delimiters("\\0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &[""]);

        let res = parse_delimiters("\\t,\\n;\\\\\\0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &["\t", ",", "\n", ";", "\\", ""]);
    }
}
