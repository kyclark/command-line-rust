use clap::{App, Arg};
use csv::StringRecord;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<usize>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    fields: Option<PositionList>,
    bytes: Option<PositionList>,
    chars: Option<PositionList>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cut")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
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
                .conflicts_with_all(&["chars", "bytes"]),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Selected bytes")
                .short("b")
                .long("bytes")
                .conflicts_with_all(&["fields", "chars"]),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Selected characters")
                .short("c")
                .long("chars")
                .conflicts_with_all(&["fields", "bytes"]),
        )
        .get_matches();

    let delim = matches.value_of("delimiter").unwrap_or("\t");
    let delim_bytes = delim.as_bytes();
    if delim.len() > 1 {
        return Err(From::from(format!(
            "--delim \"{}\" must be a single byte",
            delim
        )));
    }

    let fields = get_positions(matches.value_of("fields"))?;
    let bytes = get_positions(matches.value_of("bytes"))?;
    let chars = get_positions(matches.value_of("chars"))?;
    if vec![&fields, &bytes, &chars]
        .into_iter()
        .all(|v| v.is_none())
    {
        return Err(From::from("Must have --fields, --bytes, or --chars"));
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        delimiter: delim_bytes[0],
        fields,
        bytes,
        chars,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    //println!("{:#?}", &config);
    for filename in &config.files {
        if let Err(err) = cut(&filename, &config) {
            eprintln!("{}: {}", filename, err);
        }
    }

    Ok(())
}

// --------------------------------------------------
pub fn cut(filename: &str, config: &Config) -> MyResult<()> {
    let file: Box<dyn BufRead> = match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(filename)?)),
    };

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
            .filter_map(|i| headers.get(*i).and_then(|v| Some(v.to_string())))
            .collect();

        println!("{}", out_headers.join(&out_delim));

        for result in reader.records() {
            let record = result?;
            let vals = extract_fields(&record, &get_fields);
            println!("{}", vals.join(&out_delim));
        }
    } else if let Some(bytes) = &config.bytes {
        let buf = BufReader::new(file);
        for line in buf.lines() {
            let line = line?;
            println!("{}", extract_bytes(&line, bytes));
        }
    } else if let Some(char_pos) = &config.chars {
        let buf = BufReader::new(file);
        for line in buf.lines() {
            let line = line?;
            println!("{}", extract_chars(&line, char_pos));
        }
    }
    Ok(())
}

// --------------------------------------------------
fn get_positions(range: Option<&str>) -> MyResult<Option<PositionList>> {
    match range {
        Some(range_val) => {
            let mut fields: Vec<usize> = vec![];
            let comma_re = Regex::new(r"(\d+)-(\d+)").unwrap();
            for val in range_val.split(",") {
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
                    match val.parse::<usize>() {
                        Ok(n) => fields.push(n.clone()),
                        Err(_) => {
                            return Err(From::from(format!(
                                "illegal list value: \"{}\"",
                                val
                            )))
                        }
                    }
                }
            }

            // Subtract one for field indexes
            Ok(Some(fields.into_iter().map(|i| i - 1).collect()))
        }
        _ => Ok(None),
    }
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
fn extract_bytes(line: &str, byte_pos: &PositionList) -> String {
    let bytes: Vec<u8> = line.bytes().collect();
    let valid = 0..line.len();
    String::from_utf8_lossy(
        &byte_pos
            .into_iter()
            .filter(|i| valid.contains(i))
            .cloned()
            .map(|i| bytes[i])
            .collect::<Vec<u8>>(),
    )
    .into_owned()
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
    use super::{
        extract_bytes, extract_chars, extract_fields, get_positions,
    };
    use csv::StringRecord;

    #[test]
    fn test_get_positions() {
        let res1 = get_positions(None);
        assert!(res1.is_ok());
        if let Ok(val1) = res1 {
            assert!(val1.is_none());
        }

        assert!(get_positions(Some("")).is_err());
        assert!(get_positions(Some("a")).is_err());
        assert!(get_positions(Some("1,a")).is_err());
        assert!(get_positions(Some("2-1")).is_err());

        let res2 = get_positions(Some("1"));
        assert!(res2.is_ok());
        if let Some(val2) = res2.unwrap() {
            assert_eq!(val2, vec![0]);
        }

        let res3 = get_positions(Some("1,3"));
        assert!(res3.is_ok());
        if let Some(val3) = res3.unwrap() {
            assert_eq!(val3, vec![0, 2]);
        }

        let res4 = get_positions(Some("1-3"));
        assert!(res4.is_ok());
        if let Some(val4) = res4.unwrap() {
            assert_eq!(val4, vec![0, 1, 2]);
        }

        let res5 = get_positions(Some("1,7,3-5"));
        assert!(res5.is_ok());
        if let Some(val5) = res5.unwrap() {
            assert_eq!(val5, vec![0, 6, 2, 3, 4]);
        }
    }

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Ken", "Captain", "12345"]);
        assert_eq!(extract_fields(&rec, &vec![0 as usize]), vec!["Ken"]);
        assert_eq!(extract_fields(&rec, &vec![1 as usize]), vec!["Captain"]);
        assert_eq!(extract_fields(&rec, &vec![0, 2]), vec!["Ken", "12345"]);
        assert_eq!(extract_fields(&rec, &vec![0, 3]), vec!["Ken"]);
        assert_eq!(extract_fields(&rec, &vec![1, 0]), vec!["Captain", "Ken"]);
    }

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &vec![0]), "".to_string());
        assert_eq!(extract_chars("ábc", &vec![0]), "á".to_string());
        assert_eq!(extract_chars("ábc", &vec![0, 2]), "ác".to_string());
        assert_eq!(extract_chars("ábc", &vec![0, 1, 2]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &vec![2, 1]), "cb".to_string());
        assert_eq!(extract_chars("ábc", &vec![0, 1, 4]), "áb".to_string());
    }

    #[test]
    fn test_extract_bytes() {
        assert_eq!(extract_bytes("ábc", &vec![0]), "�".to_string());
        assert_eq!(extract_bytes("ábc", &vec![0, 1]), "á".to_string());
        assert_eq!(extract_bytes("ábc", &vec![0, 1, 2]), "áb".to_string());
        assert_eq!(
            extract_bytes("ábc", &vec![0, 1, 2, 3]),
            "ábc".to_string()
        );
        assert_eq!(extract_bytes("ábc", &vec![3, 2]), "cb".to_string());
        assert_eq!(extract_bytes("ábc", &vec![0, 1, 5]), "á".to_string());
    }
}
