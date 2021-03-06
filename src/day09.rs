use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|s| s.bytes().map(|v| (v - 48) as i32).collect())
        .collect()
}

const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn low_points(grid: &[&[i32]]) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut points: Vec<(usize, usize)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            let neighbors: Vec<(usize, usize)> = NEIGHBORS
                .iter()
                .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                .filter(|&(x, y)| {
                    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
                })
                .map(|(x, y)| (x as usize, y as usize))
                .collect();
            let mut is_low_point = true;
            for (x, y) in neighbors {
                if &grid[y][x] <= v {
                    is_low_point = false;
                    break;
                }
            }
            if is_low_point {
                points.push((x, y))
            }
        }
    }
    points
}

pub fn part_one(input: &str) -> i32 {
    let grid = parse_input(input);
    let grid: Vec<&[i32]> = grid.iter().map(|row| row.as_slice()).collect();
    low_points(&grid)
        .iter()
        .map(|(x, y)| grid[*y][*x] + 1)
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    let grid = parse_input(input);
    let grid: Vec<&[i32]> = grid.iter().map(|row| row.as_slice()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let points = low_points(&grid);
    let mut basins: Vec<i32> = Vec::new();
    for (x, y) in points {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: Vec<(usize, usize)> = vec![(x, y)];
        while let Some((x, y)) = queue.pop() {
            let range = grid[y][x]..9;
            visited.insert((x, y));
            let neighbors: Vec<(usize, usize)> = NEIGHBORS
                .iter()
                .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                .filter(|&(x, y)| {
                    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
                })
                .map(|(x, y)| (x as usize, y as usize))
                .filter(|&(x, y)| !visited.contains(&(x, y)))
                .filter(|&(x, y)| range.contains(&grid[y][x]))
                .collect();
            queue.extend(neighbors)
        }
        basins.push(visited.len() as i32);
    }
    basins.sort_unstable();
    basins.reverse();
    basins[0] * basins[1] * basins[2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn example_two() {
        let input = read_example(9);
        assert_eq!(part_two(&input), 1134);
    }
}
