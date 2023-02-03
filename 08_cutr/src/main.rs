use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cut`
pub struct Args {
    /// Input file(s)
    files: Vec<String>,

    /// Field delimiter
    delimiter: u8,

    ///
    extract: Extract,
}

// --------------------------------------------------
fn main() {
    let args = Args::parse();
    println!("{args:?}");

    //if let Err(e) = cutr::get_args().and_then(cutr::run) {
    //    eprintln!("{}", e);
    //    std::process::exit(1);
    //}
}
