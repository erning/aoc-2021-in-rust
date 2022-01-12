use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|s| s.bytes().map(|b| b - 48).collect())
        .collect()
}

const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(grid: &[Vec<u8>], times: usize) -> Option<usize> {
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let height = grid_height * times;
    let width = grid_width * times;
    let start = (0, 0);
    let end = (width - 1, height - 1);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    visited.insert(start);
    let state = State {
        cost: 0,
        position: start,
    };
    queue.push(state);

    while let Some(state) = queue.pop() {
        if state.position == end {
            return Some(state.cost);
        }
        let (x, y) = state.position;
        let neighbors: Vec<(usize, usize)> = NEIGHBORS
            .iter()
            .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
            .filter(|&(x, y)| {
                x >= 0 && y >= 0 && x < width as i32 && y < height as i32
            })
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|p| !visited.contains(p))
            .collect();
        for (x, y) in neighbors {
            let mut cost = grid[y % grid_height][x % grid_width] as usize;
            cost += x / grid_width;
            cost += y / grid_height;
            while cost > 9 {
                cost -= 9
            }
            visited.insert((x, y));
            let state = State {
                cost: cost + state.cost,
                position: (x, y),
            };
            queue.push(state);
        }
    }
    None
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    shortest_path(&grid, 1).unwrap()
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    shortest_path(&grid, 5).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(15);
        assert_eq!(part_one(&input), 40);
    }

    #[test]
    fn example_two() {
        let input = read_example(15);
        assert_eq!(part_two(&input), 315);
    }
}
