fn parse_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect()
}

fn spawn(input: &[u8], days: usize) -> u64 {
    let mut timers = [0_u64; 9];
    for &i in input {
        timers[i as usize] += 1;
    }

    for _ in 0..days {
        let zero = timers[0];
        for i in 1..9 {
            timers[i - 1] = timers[i];
        }
        timers[6] += zero;
        timers[8] = zero;
    }

    timers.iter().sum()
}

pub fn part_one(input: &str) -> u64 {
    let input = parse_input(input);
    spawn(&input, 80)
}

pub fn part_two(input: &str) -> u64 {
    let input = parse_input(input);
    spawn(&input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn spawn() {
        let cases = r"Initial state: 3,4,3,1,2
After  1 day:  2,3,2,0,1
After  2 days: 1,2,1,6,0,8
After  3 days: 0,1,0,5,6,7,8
After  4 days: 6,0,6,4,5,6,7,8,8
After  5 days: 5,6,5,3,4,5,6,7,7,8
After  6 days: 4,5,4,2,3,4,5,6,6,7
After  7 days: 3,4,3,1,2,3,4,5,5,6
After  8 days: 2,3,2,0,1,2,3,4,4,5
After  9 days: 1,2,1,6,0,1,2,3,3,4,8
After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8";
        let cases: Vec<u64> = cases
            .lines()
            .map(|s| s.len() - 15)
            .map(|v| v / 2 + 1)
            .map(|v| v as u64)
            .collect();
        let input = [3,4,3,1,2];
        for day in 0..cases.len() {
            assert_eq!(super::spawn(&input, day), cases[day] as u64);
        }
    }

    #[test]
    fn example() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 5934);
        assert_eq!(part_two(&input), 26984457539);
    }
}
