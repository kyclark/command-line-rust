extern crate clap;
extern crate rand;
extern crate rand_chacha;

use clap::{App, Arg};
use rand::{Rng, SeedableRng};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
//use std::iter;
use titlecase::titlecase;

pub type MyResult<T> = Result<T, Box<dyn Error>>;
pub type Chain = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub k: usize,
    pub num_words: usize,
    pub min_len: usize,
    pub max_len: usize,
    pub seed: Option<u64>,
    pub titlecase: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wc")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("k")
                .value_name("INT")
                .help("K-mer size")
                .short("k")
                .long("kmer")
                .default_value("3"),
        )
        .arg(
            Arg::with_name("num_words")
                .value_name("INT")
                .help("Number of words to generate")
                .short("w")
                .long("words")
                .default_value("5"),
        )
        .arg(
            Arg::with_name("min_len")
                .value_name("INT")
                .help("Minimum length of word")
                .short("m")
                .long("min")
                .default_value("5"),
        )
        .arg(
            Arg::with_name("max_len")
                .value_name("INT")
                .help("Maximum length of word")
                .short("x")
                .long("max")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("titlecase")
                .help("Titlecase words")
                .short("t")
                .long("titlecase")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("seed")
                .value_name("INT")
                .help("Random seed")
                .short("s")
                .long("seed"),
        )
        .get_matches();

    let k = matches
        .value_of("k")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(3);

    if k < 2 {
        return Err(From::from(format!("-k \"{}\" must be > 2", k)));
    }

    let num_words = matches
        .value_of("num_words")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(5);

    if num_words < 1 {
        return Err(From::from(format!(
            "--words \"{}\" must be positive",
            num_words
        )));
    }

    let min_len = matches
        .value_of("min_len")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(5);

    if min_len < 1 {
        return Err(From::from(format!(
            "--min \"{}\" must be positive",
            min_len
        )));
    }

    let max_len = matches
        .value_of("max_len")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(10);

    if max_len < 1 {
        return Err(From::from(format!(
            "--max \"{}\" must be positive",
            max_len
        )));
    }

    if min_len >= max_len {
        return Err(From::from(format!(
            "--min_len \"{}\" must be less than --max_len \"{}\"",
            min_len, max_len
        )));
    }

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        seed: matches.value_of("seed").and_then(|v| v.parse::<u64>().ok()),
        k: k,
        num_words: num_words,
        min_len: min_len,
        max_len: max_len,
        titlecase: matches.is_present("titlecase"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut rng = match &config.seed {
        Some(s) => rand_chacha::ChaCha8Rng::seed_from_u64(*s),
        _ => rand_chacha::ChaCha8Rng::from_entropy(),
    };

    let chain = read_training(&config.files, &config.k)?;

    // This might never halt.
    //let words: Vec<String> = iter::repeat('a')
    //    .filter_map(|_| make_word(&chain, &config))
    //    .take(config.num_words)
    //    .map(|w| titlecase(&w))
    //    .collect();
    //println!("{}", words.join("\n"));

    let mut found = 0;
    for _ in 0..config.num_words {
        if let Ok(word) = make_word(&chain, &mut rng, &config) {
            println!(
                "{}",
                if config.titlecase {
                    titlecase(&word)
                } else {
                    word
                }
            );
            found += 1;
        }
    }

    if found == 0 {
        println!("Sorry, couldn't produce any strings.")
    }

    Ok(())
}

// --------------------------------------------------
pub fn read_training(files: &Vec<String>, k: &usize) -> MyResult<Chain> {
    let mut chain: Chain = HashMap::new();
    let not_wanted = Regex::new(r"[^A-Za-z0-9'_]").unwrap();
    for filename in files {
        let file = File::open(filename)?;
        let file = BufReader::new(file);

        for line in file.lines() {
            let line = line?;
            let words = &line
                .to_lowercase()
                .split_whitespace()
                .map(|s| not_wanted.replace(s, "").to_string())
                .collect::<Vec<String>>();

            for word in words.iter().take(1) {
                let chars: Vec<char> = word.chars().collect();
                for kmer in chars.windows(*k) {
                    let prefix = kmer[0..k - 1].iter().collect::<String>();
                    let suffix = kmer.last().unwrap().to_string();
                    let _entry = &chain
                        .entry(prefix)
                        .and_modify(|e| e.push(suffix.clone()))
                        .or_insert_with(|| vec![suffix.clone()]);
                }
            }
        }
    }

    match chain.len() > 0 {
        true => Ok(chain),
        _ => Err(From::from(format!(
            "Cannot create chains with k \"{}\"",
            &k
        ))),
    }
}

// --------------------------------------------------
pub fn make_word(
    chain: &Chain,
    rng: &mut rand_chacha::ChaCha8Rng,
    config: &Config,
) -> MyResult<String> {
    let mut keys: Vec<&String> = chain.keys().collect();
    keys.sort(); // Sorting is important for testing!
    let start = rng.gen_range(0..keys.len());
    let mut word = keys[start].to_string();
    let wanted = rng.gen_range(config.min_len..config.max_len);

    loop {
        let len = word.len() + 1;
        let key = word[len - &config.k..].to_string();
        if let Some(opts) = chain.get(&key) {
            let mut opts: Vec<&String> = opts.iter().collect();
            opts.sort();

            let next = rng.gen_range(0..opts.len());
            word += &opts[next].to_string();
            if word.len() == wanted {
                break;
            }
        } else {
            break;
        }
    }

    match word.len() >= config.min_len {
        true => Ok(word),
        _ => Err(From::from(word)),
    }
}
