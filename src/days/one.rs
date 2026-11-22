use crate::helpers;
use std::io;

const DAY_NUM: u8 = 1;

pub fn run(data_type: String) -> io::Result<()> {
    println!("This is Day {}", DAY_NUM);
    let in_lines = helpers::data_reader::read_data(&DAY_NUM, data_type)?;
    part_one(&in_lines);
    part_two(&in_lines);

    Ok(())
}

fn part_one(in_lines: &[String]) {
    let ans: String = "".to_string();
    for _ in in_lines {
        {}
    }
    println!("Part 1 Answer: {ans}");
}
fn part_two(in_lines: &[String]) {
    let ans: String = "".to_string();
    for _ in in_lines {
        {}
    }
    println!("Part 2 Answer: {ans}");
}
