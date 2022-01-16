fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn find_cheapest<F>(input: &[i32], f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let &max = input.iter().max().unwrap();
    let &min = input.iter().min().unwrap();
    let mut cheapest = i32::MAX;
    for i in min..=max {
        let cost: i32 = input.iter().map(|&v| f(i, v)).sum();
        if cost < cheapest {
            cheapest = cost;
        }
    }
    cheapest
}

pub fn part_one(input: &str) -> i32 {
    let input = parse_input(input);
    find_cheapest(&input, |a, b| (a - b).abs())
}

pub fn part_two(input: &str) -> i32 {
    let input = parse_input(input);
    find_cheapest(&input, |a, b| {
        let n = (a - b).abs();
        n * (n + 1) / 2
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 37);
        assert_eq!(part_two(&input), 168);
    }
}
