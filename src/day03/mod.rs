fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|x| x.bytes().map(|b| b - 48).collect())
        .collect()
}

fn most_common_bit(bits: &[u8]) -> u8 {
    let zero = bits.iter().filter(|&x| *x == 0).count();
    let one: usize = bits.len() - zero;
    match (zero, one) {
        (zero, one) if zero > one => 0,
        (zero, one) if zero < one => 1,
        _ => 1,
    }
}

fn least_common_bit(bits: &[u8]) -> u8 {
    1 - most_common_bit(bits)
}

fn bits_to_int(bits: &[u8]) -> i32 {
    bits.iter().fold(0, |acc, &x| acc << 1 | x as i32)
}

pub fn part_one(input: &str) -> i32 {
    let input = parse_input(input);
    let cols = input[0].len();
    let mut gamma = vec![0u8; cols];
    let mut epsilon = vec![0u8; cols];

    for x in 0..cols {
        let bits: Vec<u8> = input.iter().map(|v| v[x]).collect();
        gamma[x] = most_common_bit(&bits);
        epsilon[x] = 1 - gamma[x];
    }

    let gamma_rate = bits_to_int(&gamma);
    let epsilon_rate = bits_to_int(&epsilon);

    gamma_rate * epsilon_rate
}

pub fn part_two(input: &str) -> i32 {
    let input = parse_input(input);

    fn life_rating(input: &Vec<Vec<u8>>, is_oxygen: bool) -> i32 {
        let mut grid = input.to_vec();
        for x in 0..input[0].len() {
            if grid.len() <= 1 {
                break;
            }
            let bits: Vec<u8> = grid.iter().map(|v| v[x]).collect();
            let mcb = match is_oxygen {
                true => most_common_bit(&bits),
                false => least_common_bit(&bits),
            };
            grid.retain(|v| v[x] == mcb);
        }
        bits_to_int(&grid[0])
    }

    let oxygen = life_rating(&input, true);
    let co2 = life_rating(&input, false);

    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn common_bit() {
        assert_eq!(most_common_bit(&[1, 0, 1]), 1);
        assert_eq!(most_common_bit(&[1, 0]), 1);
        assert_eq!(least_common_bit(&[1, 0, 1]), 0);
        assert_eq!(least_common_bit(&[1, 0]), 0);
    }

    #[test]
    fn example_one() {
        let input = read_example(3);
        assert_eq!(part_one(input.as_str()), 198);
    }

    #[test]
    fn example_two() {
        let input = read_example(3);
        assert_eq!(part_two(input.as_str()), 230);
    }
}
