use aoc::*;
use std::env;

fn main() {
    let puzzles: Vec<(u8, &str, fn(&str) -> String, fn(&str) -> String)> = vec![
        (1, "Sonar Sweep", day01::part_one, day01::part_two),
        (2, "Dive!", day02::part_one, day02::part_two),
    ];

    let filename = match env::args().find(|a| a == "--example") {
        None => "input",
        Some(_) => "example",
    };

    let mut days: Vec<u8> =
        env::args().filter_map(|a| a.parse().ok()).collect();

    if days.is_empty() {
        for (day, _, _, _) in &puzzles {
            days.push(*day)
        }
    }

    for day in days {
        let (_, title, part1, part2) = puzzles[day as usize - 1];
        let input = read_as_string(day, filename);
        let input = input.as_str();

        println!("--- Day {}: {} ---", day, title);
        println!("Part One: {}", part1(input));
        println!("Part Two: {}", part2(input));
        println!()
    }
}
