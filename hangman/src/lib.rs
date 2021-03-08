extern crate clap;

use clap::{App, Arg};
use itertools::Itertools;
use std::error::Error;
use std::io;
//use std::io::*;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    word: String,
}

#[derive(Debug)]
pub struct State {
    word: String,
    num_guesses: u32,
    max_guesses: u32,
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
                .required(true),
        )
        .get_matches();

    Ok(Config {
        word: matches.value_of("word").unwrap().to_string(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    //println!("{:?}", config);

    let state = State {
        word: config.word.to_string(),
        num_guesses: 0,
        max_guesses: 3,
    };

    play(state)?;

    Ok(())
}

// --------------------------------------------------
pub fn play(state: State) -> MyResult<()> {
    let num_guess = state.num_guesses + 1;

    if num_guess == state.max_guesses {
        return Err(From::from(format!(
            "You lose, loser! The word was \"{}\"!",
            state.word
        )));
    }

    let blanks = "_".repeat(state.word.len());
    println!(
        "Guess #{}: {}",
        num_guess,
        blanks
            .split("")
            .into_iter()
            .intersperse(" ")
            .collect::<String>()
    );

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read from stdin");
    println!("guess {}", guess);

    play(State {
        word: state.word.to_string(),
        num_guesses: num_guess,
        max_guesses: state.max_guesses,
    })
}
