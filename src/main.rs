use std::fs::File;
use std::io::Read;

use aoc::*;

fn read_input(day: i8, filename: &str) -> String {
    let filename = format!("inputs/{:02}-{}.txt", day, filename);
    let mut file = File::open(filename).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    return input;
}

fn main() {
    let puzzles = vec![(1, "Sonar Sweep", day01::part_one, day01::part_two)];

    let puzzle = puzzles[0];

    let day = puzzle.0;
    let title = puzzle.1;
    let input = read_input(day, "input");
    let part1 = puzzle.2;
    let part2 = puzzle.3;

    println!("--- Day {}: {} ---", day, title);
    println!("Part One: {}", part1(input.as_str()));
    println!("Part Two: {}", part2(input.as_str()));
}
