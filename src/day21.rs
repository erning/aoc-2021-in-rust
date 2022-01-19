use std::collections::HashMap;

fn parse_input(input: &str) -> [u32; 2] {
    let positions: Vec<u32> = input
        .lines()
        .map(|line| line.chars().last().unwrap().to_digit(10).unwrap())
        .collect();
    [positions[0], positions[1]]
}

pub fn part_one(input: &str) -> u32 {
    let mut positions = parse_input(input);
    let mut scores: [u32; 2] = [0, 0];
    let mut player = 0;
    let mut times = 0;

    loop {
        times += 1;
        let dice = times * 3 - 1;
        let step = dice * 3;
        let position = (positions[player] - 1 + step) % 10 + 1;
        let score = scores[player] + position;
        positions[player] = position;
        scores[player] = score;

        player = (player + 1) % 2;
        if score >= 1000 {
            break times * 3 * scores[player];
        }
    }
}

pub fn part_two(input: &str) -> u64 {
    type Status = [(u32, u32); 2]; // [(player 1 position, score), (player 2 position, score)]
    type CacheKey = (usize, Status); // (player, status)
    type Winners = [u64; 2]; // [count of player 1 win, count of player 2 win]
    type Cache = HashMap<CacheKey, Winners>;

    let positions = parse_input(input);
    let mut cache: Cache = HashMap::new();

    fn play(cache: &mut Cache, player: usize, status: Status) -> Winners {
        let mut winners: Winners = [0, 0];
        for a in 1..=3 {
            for b in 1..=3 {
                for c in 1..=3 {
                    let step = a + b + c;
                    let position = (status[player].0 - 1 + step) % 10 + 1;
                    let score = status[player].1 + position;
                    if score >= 21 {
                        winners[player] += 1;
                    } else {
                        let mut next_status = status;
                        next_status[player] = (position, score);
                        let key = (player, next_status);
                        let next_winner = match cache.get(&key) {
                            Some(v) => *v,
                            None => {
                                play(cache, (player + 1) % 2, next_status)
                            }
                        };
                        cache.insert(key, next_winner);
                        winners[0] += next_winner[0];
                        winners[1] += next_winner[1];
                    }
                }
            }
        }
        winners
    }

    let winners = play(&mut cache, 0, [(positions[0], 0), (positions[1], 0)]);
    *winners.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(21);
        assert_eq!(part_one(&input), 739785);
    }

    #[test]
    fn example_two() {
        let input = read_example(21);
        assert_eq!(part_two(&input), 444356092776315);
    }
}
