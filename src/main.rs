use std::env;
use std::fmt::Display;

fn main() {
    macro_rules! puzzle {
        ($mod:ident, $title:expr) => {
            (
                $title,
                |input| Box::new(aoc::$mod::part_one(input)),
                |input| Box::new(aoc::$mod::part_two(input)),
            )
        };
    }

    type SolverFn = fn(&str) -> Box<dyn Display>;

    let puzzles: Vec<(&str, SolverFn, SolverFn)> = vec![
        puzzle!(day01, "Sonar Sweep"),
        puzzle!(day02, "Dive!"),
        puzzle!(day03, "Binary Diagnostic"),
        puzzle!(day04, "Giant Squid"),
        puzzle!(day05, "Hydrothermal Venture"),
        puzzle!(day06, "Lanternfish"),
        puzzle!(day07, "The Treachery of Whales"),
        puzzle!(day08, "Seven Segment Search"),
        puzzle!(day09, "Smoke Basin"),
        puzzle!(day10, "Syntax Scoring"),
        puzzle!(day11, "Dumbo Octopus"),
        puzzle!(day12, "Passage Pathing"),
        puzzle!(day13, "Transparent Origami"),
        puzzle!(day14, "Extended Polymerization"),
        puzzle!(day15, "Chiton"),
        puzzle!(day16, "Packet Decoder"),
        puzzle!(day17, "Trick Shot"),
        puzzle!(day18, "Snailfish"),
        puzzle!(day19, "Beacon Scanner"),
        puzzle!(day20, "Trench Map"),
        puzzle!(day21, "Dirac Dice"),
        puzzle!(day22, "Reactor Reboot"),
        puzzle!(day23, "Amphipod"),
        puzzle!(day24, "Arithmetic Logic Unit"),
        puzzle!(day25, "Sea Cucumber"),
    ];

    let filename = match env::args().find(|a| a == "--example") {
        None => "input",
        Some(_) => "example",
    };

    let mut days: Vec<usize> =
        env::args().filter_map(|a| a.parse().ok()).collect();

    if days.is_empty() {
        days = (1..=puzzles.len()).collect();
    }

    for day in days {
        let (title, part1, part2) = &puzzles[day - 1];
        let input = aoc::read_as_string(day as u8, filename);
        let input = input.as_str();

        println!("--- Day {}: {} ---", day, title);
        println!("Part One: {}", part1(input));
        println!("Part Two: {}", part2(input));
        println!();
    }
}
