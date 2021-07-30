use clap::{App, Arg};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: Option<String>,
    rotate: usize,
    chunk: usize,
    width: usize,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wc")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)"),
        )
        .arg(
            Arg::with_name("rotate")
                .value_name("ROTATE")
                .help("Rotate value")
                .short("r")
                .long("rotate")
                .default_value("13"),
        )
        .arg(
            Arg::with_name("chunk")
                .value_name("CHUNK")
                .help("Chunk size")
                .short("c")
                .long("chunk")
                .default_value("5"),
        )
        .arg(
            Arg::with_name("width")
                .value_name("WIDTH")
                .help("Text width")
                .short("w")
                .long("width")
                .default_value("70"),
        )
        .get_matches();

    let file = matches.value_of("file").map(|v| v.to_string());

    if let Some(filename) = &file {
        if let Some(error) = File::open(filename).err() {
            return Err(From::from(format!("{}: {}", filename, error)));
        }
    }

    let rotate = matches
        .value_of("rotate")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(13);

    if !(1..=26).contains(&rotate) {
        return Err(From::from(format!(
            "--rotate \"{}\" must be between 1 and 26",
            rotate
        )));
    }

    let chunk = matches
        .value_of("chunk")
        .and_then(|c| c.parse::<usize>().ok())
        .unwrap_or(5);

    if chunk < 1 {
        return Err(From::from(format!(
            "--chunk \"{}\" must be positive",
            chunk
        )));
    }

    let width = matches
        .value_of("width")
        .and_then(|w| w.parse::<usize>().ok())
        .unwrap_or(70);

    if width < 1 {
        return Err(From::from(format!(
            "--width \"{}\" must be positive",
            width
        )));
    }

    Ok(Config {
        file,
        rotate,
        chunk,
        width,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let file: Box<dyn BufRead> = match &config.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => {
            Box::new(BufReader::new(File::open(filename).unwrap()))
        }
    };

    let lines = io::BufReader::new(file).lines();
    let mut text: String = "".to_string();
    for line in lines {
        let line = line?;
        text += &line;
    }

    let rotated = rot(&text, &config.rotate);
    let chunks = chunk_text(&rotated, &config.chunk).join(" ");

    println!("{}", textwrap::wrap(&chunks, config.width).join("\n"));

    Ok(())
}

// --------------------------------------------------
fn rot(input_text: &str, rotate: &usize) -> String {
    let mut text = input_text.to_uppercase();
    let nums = [
        ("1", "ONE"),
        ("2", "TWO"),
        ("3", "THREE"),
        ("4", "FOUR"),
        ("5", "FIVE"),
        ("6", "SIX"),
        ("7", "SEVEN"),
        ("8", "EIGHT"),
        ("9", "NINE"),
        ("0", "ZERO"),
    ];
    for (numeral, number) in &nums {
        text = text.replace(numeral, number);
    }

    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let shifted = vec![
        letters[*rotate..].to_string(),
        letters[..*rotate].to_string(),
    ]
    .join("");

    let mut translate: HashMap<String, String> = HashMap::new();
    for (c1, c2) in letters.chars().zip(shifted.chars()) {
        translate.insert(c1.to_string(), c2.to_string());
    }

    text.chars()
        .filter_map(|c| translate.get(&c.to_string()))
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}

// --------------------------------------------------
fn chunk_text(text: &str, size: &usize) -> Vec<String> {
    text.chars()
        .collect::<Vec<char>>()
        .chunks(*size)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot() {
        assert_eq!(rot("", &0), "".to_string());
        assert_eq!(rot("foo bar", &1), "GPPCBS".to_string());
        assert_eq!(rot("foo bar", &13), "SBBONE".to_string());
        assert_eq!(rot("123", &1), "POFUXPUISFF".to_string());
    }

    #[test]
    fn test_chunk() {
        assert_eq!(
            chunk_text("ABCDEFG", &3),
            vec!["ABC".to_string(), "DEF".to_string(), "G".to_string()]
        );

        assert_eq!(
            chunk_text("ABCDEFG", &5),
            vec!["ABCDE".to_string(), "FG".to_string()]
        );
    }
}
