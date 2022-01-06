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
            ("forward", x) => {
                pos += x;
            }
            ("up", x) => {
                depth -= x;
            }
            ("down", x) => {
                depth += x;
            }
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
                depth += aim * x;
            }
            ("up", x) => {
                aim -= x;
            }
            ("down", x) => {
                aim += x;
            }
            _ => {}
        }
    }
    return pos * depth;
}
