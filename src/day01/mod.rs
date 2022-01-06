fn parse_input(input: &str) -> Vec<i32> {
    return input.lines().map(|v| v.parse().unwrap()).collect();
}

pub fn part_one(input: &str) -> usize {
    let input = parse_input(input);
    return input.windows(2).filter(|v| v[0] < v[1]).count();
}

pub fn part_two(input: &str) -> usize {
    let input = parse_input(input);
    return input.windows(4).filter(|v| v[0] < v[3]).count();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn example() {
        let input = read_example(1);
        let input = input.as_str();
        assert_eq!(part_one(input), 7);
        assert_eq!(part_two(input), 5);
    }
}
