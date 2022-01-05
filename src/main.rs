use std::any::Any;
use std::fmt::Display;
use aoc::*;

fn main() {
    type PuzzleFunction = fn(&str) -> usize;
    type Puzzle = (u8, &'static str, PuzzleFunction, PuzzleFunction);

    let puzzles: Vec<Puzzle> = vec![
        (1, "Sonar Sweep", day01::part_one, day01::part_two),
        (2, "Dive!", day02::part_one, day02::part_two),
    ];

    for puzzle in &puzzles {
        let (day, title, part1, part2) = puzzle;
        let input = read_input(1);

        println!("--- Day {}: {} ---", day, title);
        println!("Part One: {}", part1(input.as_str()));
        println!("Part Two: {}", part2(input.as_str()));
        println!()
    }
}
