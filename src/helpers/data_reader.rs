use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn read_data(&day: &u8, data_type: String) -> io::Result<Vec<String>> {
    let kind = if data_type == "real" { "real" } else { "fake" };
    let filename = format!("data/{kind}{day}");

    println!("input filename: {filename}");

    let file = File::open(filename)?;
    BufReader::new(file).lines().collect()
}
