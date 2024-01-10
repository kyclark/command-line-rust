use anyhow::Result;
use clap::Parser;
use rand::{distributions::Alphanumeric, Rng};
use std::{fs::File, io::Write};
use thousands::Separable;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Make big text files
struct Args {
    /// Output filename
    #[arg(short, long, value_name = "FILE", default_value = "out.txt")]
    outfile: String,

    /// Number of lines
    #[arg(
        short,
        long,
        default_value = "100000",
        value_name = "LINES",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("Error: {e}");
        std::process::abort();
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    let mut file = File::create(&args.outfile)?;
    for _ in 0..args.lines {
        let num_words = rand::thread_rng().gen_range(7..15);
        let mut words = vec![];
        for _ in 0..num_words {
            words.push(random_string());
        }
        writeln!(file, "{}", words.join(" "))?;
    }

    println!(
        r#"Done, wrote {} line{} to "{}"."#,
        args.lines.separate_with_commas(),
        if args.lines == 1 { "" } else { "s" },
        args.outfile
    );

    Ok(())
}

// --------------------------------------------------
fn random_string() -> String {
    let length = rand::thread_rng().gen_range(2..8);
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
