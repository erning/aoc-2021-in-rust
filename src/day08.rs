use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|line| {
            line[11..]
                .iter()
                .filter(|code| matches!(code.len(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum()
}

fn build_code_map<'a>(codes: &'a [&str]) -> HashMap<&'a str, u8> {
    let mut code_map: HashMap<&str, u8> = HashMap::new();
    let mut number_sets: HashMap<u8, HashSet<char>> = HashMap::new();

    macro_rules! set_code {
        ($code:expr, $num:expr) => {
            code_map.insert($code, $num);
            number_sets.insert($num, HashSet::from_iter($code.chars()));
        };
    }

    for code in codes {
        let num = match code.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => continue,
        };
        set_code!(code, num);
    }

    for code in codes.iter().filter(|code| code.len() == 6) {
        let sc: HashSet<char> = HashSet::from_iter(code.chars());
        if number_sets.get(&4).unwrap().is_subset(&sc) {
            set_code!(code, 9);
        } else if number_sets.get(&7).unwrap().is_subset(&sc) {
            set_code!(code, 0);
        } else {
            set_code!(code, 6);
        }
    }

    for code in codes.iter().filter(|v| v.len() == 5) {
        let sc: HashSet<char> = HashSet::from_iter(code.chars());
        if number_sets.get(&7).unwrap().is_subset(&sc) {
            set_code!(code, 3);
        } else if number_sets.get(&6).unwrap().intersection(&sc).count() == 4
        {
            set_code!(code, 2);
        } else {
            set_code!(code, 5);
        }
    }

    code_map
}

pub fn part_two(input: &str) -> i32 {
    let input = parse_input(input);
    let input: Vec<Vec<String>> = input
        .iter()
        .map(|line| {
            line.iter()
                .map(|code| {
                    // sort each code
                    let mut chs: Vec<char> = code.chars().collect();
                    chs.sort_unstable();
                    String::from_iter(chs)
                })
                .collect()
        })
        .collect();

    input
        .iter()
        .map(|line| {
            // build code_map by the left ten codes
            let lhs: Vec<&str> =
                line[..10].iter().map(|code| code.as_str()).collect();
            let code_map = build_code_map(&lhs);
            // decode the right four codes
            let rhs: Vec<&str> =
                line[11..].iter().map(|code| code.as_str()).collect();
            rhs.iter()
                .map(|code| *(code_map.get(code).unwrap()) as i32)
                .fold(0, |acc, x| acc * 10 + x)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(8);
        assert_eq!(part_one(&input), 26);
        assert_eq!(part_two(&input), 61229);
    }
}
