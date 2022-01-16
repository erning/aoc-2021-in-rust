fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|s| s.bytes().map(|b| b - 48).collect())
        .collect()
}

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn flash(grid: &mut Vec<Vec<u8>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut queue: Vec<(usize, usize)> = Vec::new();
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, v) in row.iter_mut().enumerate() {
            *v += 1;
            if *v > 9 {
                queue.push((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop() {
        for (dx, dy) in &NEIGHBORS {
            let (x, y) = (x as i32 + dx, y as i32 + dy);
            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            let v = &mut grid[y][x];
            *v += 1;
            if *v == 10 {
                queue.push((x, y));
            }
        }
    }

    let mut count = 0;
    for row in grid.iter_mut() {
        for v in row.iter_mut() {
            if *v > 9 {
                *v = 0;
                count += 1;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut count = 0;
    for _ in 0..100 {
        count += flash(&mut grid);
    }
    count
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    let mut step = 1;
    while flash(&mut grid) < width * height {
        step += 1;
    }
    step
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(11);
        assert_eq!(part_one(&input), 1656);
    }

    #[test]
    fn example_two() {
        let input = read_example(11);
        assert_eq!(part_two(&input), 195);
    }
}
