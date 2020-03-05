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

    let word = matches.value_of("word").unwrap();
    println!(
        "Ahoy, Captain, {} {} off the larbord bow!",
        article(word.to_string()),
        word
    );

    //if let Some(word) = matches.value_of("word") {
    //    println!(
    //        "Ahoy, Captain, {} {} off the larbord bow!",
    //        article(word.to_string()),
    //        word
    //    );
    //} else {
    //    println!("What went wrong?");
    //}
}

fn article(word: String) -> &'static str {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    if let Some(letter) = word.chars().next() {
        if vowels.contains(&letter) {
            "an"
        } else {
            "a"
        }
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article() {
        assert_eq!(article("aviso".to_string()), "an");
        assert_eq!(article("Aviso".to_string()), "an");
        assert_eq!(article("boat".to_string()), "a");
        assert_eq!(article("Boat".to_string()), "a");
    }
}
