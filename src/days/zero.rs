use crate::helpers;
use std::io;

pub fn run(data_type: String) -> io::Result<()> {
    let day_num: u8 = 0;
    println!("This is Day {}", day_num);
    let in_lines = helpers::data_reader::read_data(day_num, data_type)?;
    part_one(&in_lines);
    part_two(&in_lines);

    Ok(())
}

fn part_one(in_lines: &[String]) {
    println!("This is Part 1!");
    for line in in_lines {
        println!("{line}");
    }
}
fn part_two(in_lines: &[String]) {
    println!("This is Part 2!");
    for line in in_lines {
        println!("{line}");
    }
}
