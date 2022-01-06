use std::env;
use std::fmt::Display;

fn main() {
    macro_rules! puzzle {
        ($day:expr, $title:expr, $mod:ident) => {
            (
                $day,
                $title,
                |input| Box::new(aoc::$mod::part_one(input)),
                |input| Box::new(aoc::$mod::part_two(input)),
            )
        };
    }

    type SolverFn = fn(&str) -> Box<dyn Display>;

    let puzzles: Vec<(u8, &str, SolverFn, SolverFn)> = vec![
        puzzle!(1, "Sonar Sweep", day01),
        puzzle!(2, "Dive!", day02),
        puzzle!(3, "Binary Diagnostic", day03),
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
        let (_, title, part1, part2) = &puzzles[day as usize - 1];
        let input = aoc::read_as_string(day, filename);
        let input = input.as_str();

        println!("--- Day {}: {} ---", day, title);
        println!("Part One: {}", part1(input));
        println!("Part Two: {}", part2(input));
        println!();
    }
}
