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
        puzzle!(4, "Giant Squid", day04),
        puzzle!(5, "Hydrothermal Venture", day05),
        puzzle!(6, "Lanternfish", day06),
        puzzle!(7, "The Treachery of Whales", day07),
        puzzle!(8, "Seven Segment Search", day08),
        puzzle!(9, "Smoke Basin", day09),
        puzzle!(10, "Syntax Scoring", day10),
        puzzle!(11, "Dumbo Octopus", day11),
        puzzle!(12, "Passage Pathing", day12),
        puzzle!(13, "Transparent Origami", day13),
        puzzle!(14, "Extended Polymerization", day14),
        puzzle!(15, "Chiton", day15),
        puzzle!(16, "Packet Decoder", day16),
        puzzle!(17, "Trick Shot", day17),
        puzzle!(18, "Snailfish", day18),
        puzzle!(19, "Beacon Scanner", day19),
        puzzle!(20, "Trench Map", day20),
        puzzle!(21, "Dirac Dice", day21),
        puzzle!(22, "Reactor Reboot", day22),
    ];

    let filename = match env::args().find(|a| a == "--example") {
        None => "input",
        Some(_) => "example",
    };

    let mut days: Vec<u8> =
        env::args().filter_map(|a| a.parse().ok()).collect();

    if days.is_empty() {
        for &(day, _, _, _) in &puzzles {
            days.push(day)
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
