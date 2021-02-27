extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("picnic")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Print items for the picnic")
        .arg(
            Arg::with_name("items")
                .value_name("STR")
                .help("Item(s) to bring")
                .required(true)
                .multiple(true),
            //.min_values(1),
        )
        .get_matches();

    if let Some(items) = matches.values_of("items") {
        //let items: Vec<str> = items.collect();
        println!("{:?}!", items);
    }
}
