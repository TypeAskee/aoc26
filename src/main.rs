mod days;
mod helpers;
use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    /// Name of day to run.
    name: String,
    /// Real or fake data.
    data_type: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.name.as_str() {
        "0" => days::zero::run(args.data_type),
        "1" => days::one::run(args.data_type),
        _ => {
            println!("You have to run a specific day!");
            Ok(())
        }
    }
}
