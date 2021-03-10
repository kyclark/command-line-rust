extern crate clap;
extern crate csv;
extern crate regex;

use clap::{App, Arg};
use csv::StringRecord;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<usize>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    fields: Option<PositionList>,
    chars: Option<PositionList>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cut")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cut")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .help("Field delimiter")
                .short("d")
                .long("delim")
                .default_value("\t"),
        )
        .arg(
            Arg::with_name("fields")
                .value_name("FIELDS")
                .help("Selected fields")
                .short("f")
                .long("fields")
                .conflicts_with("chars"),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Selected characters")
                .short("c")
                .long("chars")
                .conflicts_with("fields"),
        )
        .get_matches();

    let delim = matches.value_of("delimiter").unwrap_or("\t");
    let delim_bytes = delim.as_bytes();
    if delim.len() > 1 {
        return Err(From::from(format!(
            "--delim \"{}\" must be a single character",
            delim
        )));
    }

    // Handle field ranges
    let raw_fields = matches
        .value_of("fields")
        .and_then(|f| Some(get_positions(f)));

    if let Some(Err(e)) = raw_fields {
        return Err(From::from(format!("Failed to parse fields: {}", e)));
    }

    let fields = match raw_fields {
        Some(Ok(val)) => Some(val),
        _ => None,
    };

    // Handle character ranges
    let raw_chars = matches
        .value_of("chars")
        .and_then(|c| Some(get_positions(c)));

    if let Some(Err(e)) = raw_chars {
        return Err(From::from(format!("Failed to parse fields: {}", e)));
    }

    let chars = match raw_chars {
        Some(Ok(val)) => Some(val),
        _ => None,
    };

    if fields.is_none() && chars.is_none() {
        return Err(From::from("Must have either fields -f or chars -c"));
    }

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        delimiter: delim_bytes[0],
        fields: fields,
        chars: chars,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    //println!("{:?}", &config);
    for filename in &config.files {
        let file = File::open(filename)?;

        if let Some(get_fields) = &config.fields {
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(config.delimiter)
                .from_reader(file);

            let out_delim = std::str::from_utf8(&[config.delimiter; 1])
                .unwrap()
                .to_string();

            let headers = reader.headers()?;
            let out_headers: Vec<String> = get_fields
                .iter()
                .filter_map(|i| {
                    headers.get(*i).and_then(|v| Some(v.to_string()))
                })
                .collect();

            println!("{}", out_headers.join(&out_delim));

            for result in reader.records() {
                let record = result?;
                let vals = extract_fields(&record, &get_fields);
                println!("{}", vals.join(&out_delim));
            }
        } else if let Some(char_pos) = &config.chars {
            let buf = BufReader::new(file);
            for line in buf.lines() {
                let line = line?;
                println!("{}", extract_chars(&line, char_pos));
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn get_positions(range_str: &str) -> MyResult<PositionList> {
    let mut fields: Vec<usize> = vec![];
    let comma_re = Regex::new(r"(\d+)-(\d+)").unwrap();
    for val in range_str.split(",") {
        if let Some(cap) = comma_re.captures(val) {
            let n1 = &cap[1].parse::<usize>()?;
            let n2 = &cap[2].parse::<usize>()?;

            if n1 < n2 {
                for n in *n1..=*n2 {
                    fields.push(n.clone());
                }
            } else {
                return Err(From::from(format!(
                    concat!(
                        "First number in range ({}) ",
                        "must be lower than second number ({})"
                    ),
                    n1, n2
                )));
            }
        } else {
            let n: usize = val.parse()?;
            fields.push(n.clone());
        }
    }

    // Subtract one for field indexes
    Ok(fields.into_iter().map(|i| i - 1).collect())
}

// --------------------------------------------------
fn extract_fields(
    record: &StringRecord,
    field_pos: &PositionList,
) -> Vec<String> {
    field_pos
        .iter()
        .filter_map(|i| record.get(*i).and_then(|v| Some(v.to_string())))
        .collect()
}

// --------------------------------------------------
fn extract_chars(line: &str, char_pos: &PositionList) -> String {
    let chars: Vec<char> = line.chars().collect();
    let valid = 0..line.len();
    char_pos
        .into_iter()
        .filter(|i| valid.contains(i))
        .cloned()
        .map(|i| chars[i])
        .collect::<String>()
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use csv::StringRecord;

    #[test]
    fn test_get_positions() {
        assert!(get_positions("").is_err());
        assert!(get_positions("a").is_err());
        assert!(get_positions("1,a").is_err());
        assert!(get_positions("2-1").is_err());

        let res1 = get_positions("1");
        assert!(res1.is_ok());
        if let Ok(res) = res1 {
            assert_eq!(res, vec![0 as usize]);
        }

        let res2 = get_positions("1,3");
        assert!(res2.is_ok());
        if let Ok(res) = res2 {
            let v: Vec<usize> = vec![0, 2];
            assert_eq!(res, v);
        }

        let res3 = get_positions("1,7,3-5");
        assert!(res3.is_ok());
        if let Ok(res) = res3 {
            let v: Vec<usize> = vec![0, 6, 2, 3, 4];
            assert_eq!(res, v);
        }
    }

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Ken", "Captain", "12345"]);
        assert_eq!(extract_fields(&rec, &vec![0 as usize]), vec!["Ken"]);
        assert_eq!(extract_fields(&rec, &vec![1 as usize]), vec!["Captain"]);

        let wanted: Vec<usize> = vec![0, 2];
        assert_eq!(extract_fields(&rec, &wanted), vec!["Ken", "12345"]);

        let wanted: Vec<usize> = vec![0, 3];
        assert_eq!(extract_fields(&rec, &wanted), vec!["Ken"]);

        let wanted: Vec<usize> = vec![1, 0];
        assert_eq!(extract_fields(&rec, &wanted), vec!["Captain", "Ken"]);
    }

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &vec![0]), "".to_string());
        assert_eq!(extract_chars("ABC", &vec![0]), "A".to_string());
        assert_eq!(extract_chars("ABC", &vec![0, 2]), "AC".to_string());
        assert_eq!(extract_chars("ABC", &vec![0, 1, 2]), "ABC".to_string());
        assert_eq!(extract_chars("ABC", &vec![2, 1]), "CB".to_string());
        assert_eq!(extract_chars("ABC", &vec![0, 1, 4]), "AB".to_string());
    }
}
