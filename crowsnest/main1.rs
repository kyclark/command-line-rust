extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("crowsnest")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Pick the article")
        .arg(
            Arg::with_name("word")
                .value_name("WORD")
                .help("Input word")
                .required(true),
        )
        .get_matches();

    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    if let Some(word) = matches.value_of("word") {
        if let Some(letter) = word.chars().next() {
            let article = if vowels.contains(&letter) { "an" } else { "a" };
            println!(
                "Ahoy, Captain, {} {} off the larbord bow!",
                article, word
            );
        }
    }
}
