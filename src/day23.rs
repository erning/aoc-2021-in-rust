use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

const COSTS: [usize; 5] = [0, 1, 10, 100, 1000];

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    burrow: Vec<u8>,
    #[cfg(debug_assertions)]
    log: Vec<Vec<u8>>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    #[cfg(debug_assertions)]
    fn new(cost: usize, burrow: Vec<u8>, log: Vec<Vec<u8>>) -> State {
        State { cost, burrow, log }
    }

    #[cfg(not(debug_assertions))]
    fn new(cost: usize, burrow: Vec<u8>) -> State {
        State { cost, burrow }
    }

    fn is_final(&self) -> bool {
        for (i, &v) in self.burrow[11..].iter().enumerate() {
            if v as usize != (i % 4) + 1 {
                return false;
            }
        }
        true
    }

    fn next_movement(&self) -> Vec<(usize, usize)> {
        let mut movement: Vec<(usize, usize)> = Vec::new();

        let amphipods: Vec<(usize, u8)> = self
            .burrow
            .iter()
            .enumerate()
            .filter(|(_, &v)| v > 0)
            .map(|(i, &v)| (i, v))
            .collect();

        let depth = (self.burrow.len() - 11) / 4;

        // move from hallway to room
        for &(spot, v) in amphipods.iter().filter(|(spot, _)| *spot < 11) {
            for room in 1..=4 {
                if v as usize != room {
                    // that room is not its destination room
                    continue;
                }
                for floor in (0..depth).rev() {
                    let target = 10 + room + floor * 4;
                    let v = self.burrow[target];
                    if v == 0 {
                        movement.push((spot, target));
                        break;
                    }
                    if v != room as u8 {
                        // that room contains amphipod which do not have that room
                        // as their own destination
                        break;
                    }
                }
            }
        }

        // move from room to hallway
        let mut rooms: [Option<usize>; 4] = [None; 4];
        for &(spot, _) in amphipods.iter().filter(|(spot, _)| *spot >= 11) {
            let i = (spot - 11) % 4;
            if rooms[i].is_none() || rooms[i].unwrap() > spot {
                rooms[i] = Some(spot);
            }
        }
        for &spot in rooms.iter().flatten() {
            for target in [0, 1, 3, 5, 7, 9, 10] {
                movement.push((spot, target));
            }
        }

        movement
    }
}

type PathCache = HashMap<(usize, usize), Vec<usize>>;

fn build_path_cache(depth: usize) -> PathCache {
    let mut cache: PathCache = HashMap::new();

    fn path(a: usize, b: usize) -> Vec<usize> {
        if a == b {
            return vec![a];
        }
        if a >= 11 && b >= 11 && (a - 11) % 4 == (b - 11) % 4 {
            let mut spots = vec![a];
            if a < b {
                spots.append(&mut path(a + 4, b));
            } else {
                spots.append(&mut path(a - 4, b));
            }
            return spots;
        }
        if a >= 11 {
            let n = if a >= 15 { a - 4 } else { (a - 11) % 4 * 2 + 2 };
            let mut spots = vec![a];
            spots.append(&mut path(n, b));
            return spots;
        }
        if b >= 11 {
            let n = if b >= 15 { b - 4 } else { (b - 11) % 4 * 2 + 2 };
            let mut spots = path(a, n);
            spots.push(b);
            return spots;
        }
        if a < b {
            (a..=b).collect()
        } else {
            (b..=a).rev().collect()
        }
    }

    let len = depth * 4 + 11;
    for a in 0..len - 1 {
        for b in a..len {
            if cache.contains_key(&(a, b)) {
                continue;
            }
            let mut path = path(a, b);
            cache.insert((a, b), path.clone());
            path.reverse();
            cache.insert((b, a), path);
        }
    }

    cache
}

fn search(initial: Vec<u8>, cache: PathCache) -> usize {
    let mut visited: HashSet<Vec<u8>> = HashSet::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    #[cfg(not(debug_assertions))]
    let state = State::new(0, initial);
    #[cfg(debug_assertions)]
    let state = State::new(0, initial, Vec::new());

    heap.push(state);

    while let Some(state) = heap.pop() {
        if state.is_final() {
            #[cfg(debug_assertions)]
            {
                for s in state.log {
                    print_burrow(&s);
                }
                print_burrow(&state.burrow);
            }

            return state.cost;
        }
        if visited.contains(&state.burrow) {
            continue;
        }
        visited.insert(state.burrow.clone());
        for (from, to) in state.next_movement() {
            let mut burrow = state.burrow.clone();
            let amphipod = burrow[from];
            burrow[to] = amphipod;
            burrow[from] = 0;
            if visited.contains(&burrow) {
                continue;
            }

            let path = cache.get(&(from, to)).unwrap();
            let mut is_blocked = false;
            for i in &path[1..] {
                if state.burrow[*i] != 0 {
                    is_blocked = true;
                    break;
                }
            }
            if is_blocked {
                continue;
            }

            let distance = path.len() - 1;
            let cost = COSTS[amphipod as usize] * distance + state.cost;
            #[cfg(not(debug_assertions))]
            heap.push(State::new(cost, burrow));

            #[cfg(debug_assertions)]
            {
                let mut log = state.log.clone();
                log.push(state.burrow.clone());
                heap.push(State::new(cost, burrow, log));
            }
        }
    }
    0
}

#[cfg(debug_assertions)]
fn print_burrow(burrow: &[u8]) {
    const SYMBOLS: [char; 5] = ['.', 'A', 'B', 'C', 'D'];

    // hallway
    println!("+-----------------------+");
    print!("|");
    for v in &burrow[..11] {
        print!(" {}", SYMBOLS[*v as usize]);
    }
    println!(" |");

    // rooms
    let mut iter = burrow[11..].iter().peekable();
    macro_rules! print_room {
        ($fmt:literal) => {
            let v1 = SYMBOLS[*iter.next().unwrap() as usize];
            let v2 = SYMBOLS[*iter.next().unwrap() as usize];
            let v3 = SYMBOLS[*iter.next().unwrap() as usize];
            let v4 = SYMBOLS[*iter.next().unwrap() as usize];
            println!($fmt, v1, v2, v3, v4);
        };
    }
    print_room!("+---+ {} | {} | {} | {} +---+");
    while iter.peek().is_some() {
        print_room!("    | {} | {} | {} | {} |");
    }
    println!("    +---+---+---+---+");
    println!();
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|s| {
            s.chars().map(|c| match c {
                '.' => Some(0),
                'A' => Some(1),
                'B' => Some(2),
                'C' => Some(3),
                'D' => Some(4),
                _ => None,
            })
        })
        .flatten()
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let burrow = parse_input(input);
    let cache = build_path_cache(2);
    search(burrow, cache)
}

pub fn part_two(input: &str) -> usize {
    let origin = parse_input(input);
    let extend = parse_input(
        "#D#C#B#A#
         #D#B#A#C#",
    );
    let mut burrow = Vec::new();
    burrow.extend_from_slice(&origin[..11]);
    burrow.extend_from_slice(&origin[11..15]);
    burrow.extend_from_slice(&extend[..]);
    burrow.extend_from_slice(&origin[15..]);
    let cache = build_path_cache(4);
    search(burrow, cache)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(23);
        assert_eq!(part_one(&input), 12521);
    }

    #[test]
    fn example_two() {
        let input = read_example(23);
        assert_eq!(part_two(&input), 44169);
    }
}
