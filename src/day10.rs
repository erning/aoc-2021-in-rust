fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> i32 {
    let input = parse_input(input);
    let mut score = 0;
    for line in &input {
        let mut stack: Vec<char> = Vec::new();
        macro_rules! check_corruption {
            ($a:expr, $p:expr) => {
                if stack.pop().unwrap() != $a {
                    score += $p;
                    break;
                }
            };
        }
        for ch in line {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(*ch),
                ')' => check_corruption!('(', 3),
                ']' => check_corruption!('[', 57),
                '}' => check_corruption!('{', 1197),
                '>' => check_corruption!('<', 25137),
                _ => unreachable!(),
            };
        }
    }
    score
}

pub fn part_two(input: &str) -> i64 {
    let input = parse_input(input);
    let mut scores: Vec<i64> = Vec::new();
    for line in &input {
        let mut score = 0;
        let mut stack: Vec<char> = Vec::new();
        let mut corrupted = false;
        macro_rules! check_corruption {
            ($a:expr) => {
                if stack.pop().unwrap() != $a {
                    corrupted = true;
                    break;
                }
            };
        }
        for ch in line {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(*ch),
                ')' => check_corruption!('('),
                ']' => check_corruption!('['),
                '}' => check_corruption!('{'),
                '>' => check_corruption!('<'),
                _ => unreachable!(),
            };
        }
        if corrupted {
            continue;
        };
        while let Some(ch) = stack.pop() {
            score = score * 5
                + match ch {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
        }
        scores.push(score);
    }
    scores.sort_unstable();
    let score = scores.get(scores.len() / 2).unwrap();
    *score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(10);
        assert_eq!(part_one(&input), 26397);
    }

    #[test]
    fn example_two() {
        let input = read_example(10);
        assert_eq!(part_two(&input), 288957);
    }
}
