fn parse_input(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|v| {
            let mut t = v.split(" ");
            (t.next().unwrap(), t.next().unwrap().parse::<i32>().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let input = parse_input(input);
    let mut pos = 0;
    let mut depth = 0;
    for step in input {
        match step {
            ("forward", x) => pos += x,
            ("up", x) => depth -= x,
            ("down", x) => depth += x,
            _ => {}
        }
    }
    return pos * depth;
}

pub fn part_two(input: &str) -> i32 {
    let input = parse_input(input);
    let mut aim = 0;
    let mut pos = 0;
    let mut depth = 0;
    for step in input {
        match step {
            ("forward", x) => {
                pos += x;
                depth += aim * x
            }
            ("up", x) => aim -= x,
            ("down", x) => aim += x,
            _ => {}
        }
    }
    return pos * depth;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 150);
        assert_eq!(part_two(&input), 900);
    }
}
