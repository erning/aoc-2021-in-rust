use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(i32, i32, i32, i32)> {
    input
        .lines()
        .map(|v| {
            v.split(&[',', '-', '>', ' '][..])
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .map(|v: Vec<i32>| (v[0], v[1], v[2], v[3]))
        .collect()
}

fn overlap_points(input: &[(i32, i32, i32, i32)], diagonal: bool) -> usize {
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    fn delta(a: i32, b: i32) -> i32 {
        match a - b {
            v if v > 0 => 1,
            v if v < 0 => -1,
            _ => 0,
        }
    }

    for &(x1, y1, x2, y2) in input {
        let dx = delta(x2, x1);
        let dy = delta(y2, y1);
        let d = match (dx, dy) {
            (_, 0) => 1,
            (0, _) => 2,
            (_, _) if diagonal => 3,
            _ => {
                continue;
            }
        };

        let mut x = x1;
        let mut y = y1;
        loop {
            let p = (x, y);
            match points.get_mut(&p) {
                Some(v) => *v += 1,
                None => {
                    points.insert(p, 1);
                }
            }
            if d == 1 && x == x2 {
                break;
            }
            if d == 2 && y == y2 {
                break;
            }
            if d == 3 && (x == x2 || y == y2) {
                break;
            }
            x += dx;
            y += dy;
        }
    }

    points.values().filter(|&v| v > &1).count()
}

pub fn part_one(input: &str) -> usize {
    let input = parse_input(input);
    overlap_points(&input, false)
}
pub fn part_two(input: &str) -> usize {
    let input = parse_input(input);
    overlap_points(&input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(5);
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn example_two() {
        let input = read_example(5);
        assert_eq!(part_two(&input), 12);
    }
}
