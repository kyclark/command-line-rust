extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("hello")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Hello with positional argument")
        .arg(
            Arg::with_name("name")
                .value_name("NAME")
                .help("Name to greet")
                .required(true),
        )
        .get_matches();

    let name = matches.value_of("name").unwrap();
    println!("Hello, {}!", name);
}
