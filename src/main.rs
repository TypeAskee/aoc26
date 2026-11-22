mod days;
mod helpers;
use clap::Parser;
use std::io;

/// A simple greeting program
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Name of day to run.
    name: String,
    /// Real or fake data.
    real: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.name.as_str() {
        "0" => days::zero::run(args.real),
        "1" => days::one::run(args.real),
        _ => {
            println!("You have to run a specific day!");
            Ok(())
        }
    }
}
