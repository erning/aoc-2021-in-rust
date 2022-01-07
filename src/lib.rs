use std::fs::File;
use std::io::Read;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub fn read_as_string(day: u8, filename: &str) -> String {
    let filename = format!("inputs/{:02}-{}.txt", day, filename);
    let mut file = File::open(filename).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    return input;
}

pub fn read_input(day: u8) -> String {
    return read_as_string(day, "input");
}
pub fn read_example(day: u8) -> String {
    return read_as_string(day, "example");
}
