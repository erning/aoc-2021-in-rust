use std::collections::HashSet;

const WIDTH: usize = 5;

fn parse_input(input: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let mut lines = input.lines();
    let steps: Vec<u8> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    let mut boards: Vec<Vec<u8>> = vec![];
    while lines.next().is_some() {
        let mut board: Vec<u8> = vec![];
        for _ in 0..WIDTH {
            let mut row: Vec<u8> = lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            board.append(&mut row);
        }
        boards.push(board)
    }
    (steps, boards)
}

fn simulate(steps: Vec<u8>, boards: Vec<Vec<u8>>, win: usize) -> i32 {
    let mut won: HashSet<usize> = HashSet::new();
    let mut marked: Vec<Vec<u8>> = vec![vec![]; boards.len()];
    let mut rows: Vec<Vec<u8>> = vec![vec![0; WIDTH]; boards.len()];
    let mut cols: Vec<Vec<u8>> = vec![vec![0; WIDTH]; boards.len()];

    for step in steps {
        for (i, board) in boards.iter().enumerate() {
            if won.contains(&i) {
                continue;
            }
            let found = board.iter().enumerate().find(|&(_, &x)| x == step);
            if found.is_none() {
                continue;
            }
            let (j, &v) = found.unwrap();
            marked[i].push(v as u8);
            rows[i][j / WIDTH] += 1;
            cols[i][j % WIDTH] += 1;
            if rows[i][j / WIDTH] < 5 && cols[i][j % WIDTH] < 5 {
                continue;
            }
            won.insert(i);
            if won.len() < win {
                continue;
            }
            let t1: i32 = board.iter().map(|&v| v as i32).sum();
            let t2: i32 = marked[i].iter().map(|&v| v as i32).sum();
            return (t1 - t2) * v as i32;
        }
    }

    0
}

pub fn part_one(input: &str) -> i32 {
    let (steps, boards) = parse_input(input);
    simulate(steps, boards, 1)
}

pub fn part_two(input: &str) -> i32 {
    let (steps, boards) = parse_input(input);
    let win = boards.len();
    simulate(steps, boards, win)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(4);
        assert_eq!(part_one(&input), 4512);
    }

    #[test]
    fn example_two() {
        let input = read_example(4);
        assert_eq!(part_two(&input), 1924);
    }
}
