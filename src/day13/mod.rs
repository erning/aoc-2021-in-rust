use std::collections::HashSet;

type Point = (i32, i32);

fn parse_input(input: &str) -> (HashSet<Point>, Vec<(char, i32)>) {
    let mut points: HashSet<Point> = HashSet::new();
    let mut folds: Vec<(char, i32)> = Vec::new();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let p: Vec<i32> =
            line.split(',').map(|v| v.parse().unwrap()).collect();
        let p = (p[0], p[1]);
        points.insert(p);
    }
    for line in lines.by_ref() {
        let f = &line["fold along ".len()..].split_once('=').unwrap();
        let f: (char, i32) =
            (f.0.chars().next().unwrap(), f.1.parse().unwrap());
        folds.push(f);
    }
    (points, folds)
}

fn fold(
    points: &HashSet<(i32, i32)>,
    at: (char, i32),
) -> HashSet<(i32, i32)> {
    let mut new_points: HashSet<(i32, i32)> = HashSet::new();
    for &(x, y) in points.iter() {
        match at {
            ('x', z) if x < z => {
                new_points.insert((x, y));
            }
            ('x', z) if x > z => {
                new_points.insert((z + z - x, y));
            }
            ('y', z) if y < z => {
                new_points.insert((x, y));
            }
            ('y', z) if y > z => {
                new_points.insert((x, z + z - y));
            }
            _ => {
                unreachable!()
            }
        }
    }
    new_points
}

fn points_as_string(points: &HashSet<(i32, i32)>) -> String {
    let w = points.iter().map(|(x, _)| x).max().unwrap() + 1;
    let h = points.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut s = String::with_capacity((w * h) as usize);
    s.push('\n');
    for y in 0..h {
        for x in 0..w {
            s.push(match points.contains(&(x, y)) {
                true => '#',
                false => ' ',
            })
        }
        s.push('\n')
    }
    s
}

pub fn part_one(input: &str) -> usize {
    let (mut points, folds) = parse_input(input);
    points = fold(&points, folds[0]);
    points.len()
}

pub fn part_two(input: &str) -> String {
    let (mut points, folds) = parse_input(input);
    for at in folds {
        points = fold(&points, at);
    }
    points_as_string(&points)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(13);
        assert_eq!(part_one(&input), 17);
    }
}
