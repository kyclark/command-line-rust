extern crate clap;

use clap::{App, Arg};
use itertools::Itertools;
use rand::{rngs::StdRng, Rng, SeedableRng};
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    word: String,
    max_guesses: u32,
}

#[derive(Debug)]
pub struct State {
    secret: String,
    current: String,
    error: Option<String>,
    num_guesses: u32,
    max_guesses: u32,
    previous_guesses: HashSet<String>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("hangman")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust hangman")
        .arg(
            Arg::with_name("word")
                .value_name("WORD")
                .help("Word to guess")
                .short("w")
                .long("word")
                .conflicts_with("file"),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("File to pick word from")
                .short("f")
                .long("file"),
        )
        .arg(
            Arg::with_name("max_guesses")
                .value_name("MAX")
                .help("Maximum number of guesses")
                .short("m")
                .long("max")
                .default_value("10"),
        )
        .get_matches();

    let seed = matches
        .value_of("seed")
        .and_then(|v| v.trim().parse::<u64>().ok());

    let word = matches.value_of("word");
    let file = matches.value_of("file");
    if word.is_none() && file.is_none() {
        return Err(From::from("Must have either --word or --file"));
    }

    let the_word = match word.is_some() {
        true => word.unwrap().to_string(),
        _ => {
            let w = pick_word(&file.unwrap(), &seed)?;
            w
        }
    };

    let max_guesses = matches
        .value_of("max_guesses")
        .and_then(|c| c.trim().parse::<u32>().ok())
        .unwrap_or(10);

    Ok(Config {
        word: the_word,
        max_guesses: max_guesses,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let state = State {
        secret: config.word.to_string(),
        current: "_".repeat(config.word.len()),
        error: None,
        num_guesses: 0,
        max_guesses: config.max_guesses,
        previous_guesses: HashSet::new(),
    };

    play(state)
}

// --------------------------------------------------
pub fn play(state: State) -> MyResult<()> {
    let num_guess = state.num_guesses + 1;

    if state.current == state.secret {
        let n = state.num_guesses;
        println!(
            "You guessed the \"{}\" in {} turn{}.",
            state.secret,
            n,
            if n == 1 { "" } else { "s" }
        );
        return Ok(());
    }

    if let Some(err) = state.error {
        println!("{}", err);
    }

    if num_guess == state.max_guesses + 1 {
        return Err(From::from(format!(
            "You lose, loser. The word was \"{}\".",
            state.secret
        )));
    }

    println!(
        "Guess #{} (! to quit): {}",
        num_guess,
        state
            .current
            .split("")
            .into_iter()
            .intersperse(" ")
            .collect::<String>()
    );

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read from stdin");
    guess = guess.trim().to_string();

    if guess.len() != 1 {
        return play(State {
            secret: state.secret.to_string(),
            current: state.current.to_string(),
            error: Some(format!("Guess \"{}\" must be 1 character", guess)),
            num_guesses: num_guess,
            max_guesses: state.max_guesses,
            previous_guesses: state.previous_guesses,
        });
    }

    if guess == "!".to_string() {
        return Err(From::from("Now you'll never know the secret."));
    }

    if state.previous_guesses.contains(&guess) {
        return play(State {
            secret: state.secret.to_string(),
            current: state.current.to_string(),
            error: Some(format!("You already guessed \"{}\".", guess)),
            num_guesses: num_guess,
            max_guesses: state.max_guesses,
            previous_guesses: state.previous_guesses,
        });
    }

    let pos: Vec<usize> = state
        .secret
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c.to_string() == guess {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    if pos.len() == 0 {
        println!("The letter \"{}\" is not present.", guess);
    }

    let new_current: String = state
        .current
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if pos.contains(&i) {
                guess.to_string()
            } else {
                c.to_string()
            }
        })
        .collect();

    let mut new_prev: HashSet<String> = HashSet::new();
    for val in state.previous_guesses {
        new_prev.insert(val);
    }
    new_prev.insert(guess);

    play(State {
        secret: state.secret.to_string(),
        current: new_current,
        error: None,
        num_guesses: num_guess,
        max_guesses: state.max_guesses,
        previous_guesses: new_prev,
    })
}

// --------------------------------------------------
fn pick_word(filename: &str, seed: &Option<u64>) -> MyResult<String> {
    let text = fs::read_to_string(filename)?;
    let re = Regex::new(r"[^a-z]").unwrap();
    let mut words: HashSet<String> = HashSet::new();
    for word in text.to_lowercase().split_whitespace() {
        words.insert(re.replace_all(word, "").to_string());
    }
    let uniq: Vec<String> = words.into_iter().collect();

    let range = 0..uniq.len();
    let i: usize = match &seed {
        Some(seed) => StdRng::seed_from_u64(*seed).gen_range(range),
        _ => rand::thread_rng().gen_range(range),
    };

    Ok(uniq[i].clone())
}
