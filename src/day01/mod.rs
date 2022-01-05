fn parse_input(input: &str) -> Vec<i32> {
    return input.lines().map(|a| a.parse().unwrap()).collect();
}

pub fn part_one(input: &str) -> usize {
    let input = parse_input(input);
    return input
        .windows(2)
        .map(|it| it[0] < it[1])
        .filter(|it| *it)
        .count();
}

pub fn part_two(input: &str) -> usize {
    let input = parse_input(input);
    return input
        .windows(4)
        .map(|it| it[0] < it[3])
        .filter(|it| *it)
        .count();
}
