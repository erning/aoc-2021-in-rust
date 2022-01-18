use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    fn rotate(&self, orientation: u8) -> Point {
        let (x, y, z) = (self.x, self.y, self.z);
        match orientation {
            0 => Point::new(x, y, z),
            1 => Point::new(x, -z, y),
            2 => Point::new(x, -y, -z),
            3 => Point::new(x, z, -y),

            4 => Point::new(-x, y, -z),
            5 => Point::new(-x, -z, -y),
            6 => Point::new(-x, -y, z),
            7 => Point::new(-x, z, y),

            8 => Point::new(y, x, -z),
            9 => Point::new(y, -z, -x),
            10 => Point::new(y, -x, z),
            11 => Point::new(y, z, x),

            12 => Point::new(-y, x, z),
            13 => Point::new(-y, -z, x),
            14 => Point::new(-y, -x, -z),
            15 => Point::new(-y, z, -x),

            16 => Point::new(z, y, -x),
            17 => Point::new(z, x, y),
            18 => Point::new(z, -y, x),
            19 => Point::new(z, -x, -y),

            20 => Point::new(-z, y, x),
            21 => Point::new(-z, -x, y),
            22 => Point::new(-z, -y, -x),
            23 => Point::new(-z, x, -y),

            _ => unimplemented!(),
        }
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        let x = (other.x - self.x).abs();
        let y = (other.y - self.y).abs();
        let z = (other.z - self.z).abs();
        x + y + z
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    position: Point,
    beacons: HashSet<Point>,
}

impl Scanner {
    fn new(position: Point, beacons: &[Point]) -> Scanner {
        let beacons: HashSet<Point> = beacons.to_vec().into_iter().collect();
        Scanner { position, beacons }
    }

    fn move_to(&self, position: Point) -> Scanner {
        let delta = position - self.position;
        let beacons: Vec<Point> =
            self.beacons.iter().map(|p| *p + delta).collect();
        Scanner::new(position, &beacons)
    }

    fn rotate(&self, orientation: u8) -> Scanner {
        let beacons: Vec<Point> =
            self.beacons.iter().map(|p| p.rotate(orientation)).collect();
        Scanner::new(self.position, &beacons)
    }

    fn intersect(&self, other: &Scanner) -> usize {
        self.beacons.intersection(&other.beacons).count()
    }

    fn find_match(&self, other: &Scanner) -> Option<Scanner> {
        for beacon in self.beacons.iter() {
            for orientation in 0..24 {
                let rotated = other.rotate(orientation);
                for rotated_beacon in &rotated.beacons {
                    let p = self.position + *beacon - *rotated_beacon;
                    let scanner = rotated.move_to(p);
                    if self.intersect(&scanner) >= 12 {
                        return Some(scanner);
                    }
                }
            }
        }
        None
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = Vec::new();
    let mut iter = input.lines();

    fn parse_beacons(iter: &mut dyn Iterator<Item = &str>) -> Vec<Point> {
        let mut beacons: Vec<Point> = Vec::new();
        for line in iter {
            if line.is_empty() {
                break;
            }
            let v = line
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>();
            beacons.push(Point::new(v[0], v[1], v[2]));
        }
        beacons
    }

    while let Some(line) = iter.next() {
        if line.starts_with("---") {
            let beacons = parse_beacons(&mut iter);
            let scanner = Scanner::new(Point::new(0, 0, 0), &beacons);
            scanners.push(scanner);
        }
    }

    scanners
}

pub fn part_one(input: &str) -> usize {
    let scanners = parse_input(input);
    let mut remaining: VecDeque<usize> = (1..scanners.len()).collect();
    let mut base = scanners[0].clone();
    while let Some(other) = remaining.pop_front() {
        if let Some(scanner) = base.find_match(&scanners[other]) {
            let beacons = base
                .beacons
                .union(&scanner.beacons)
                .copied()
                .collect::<Vec<Point>>(); //.map(|p| *p).collect();
            base = Scanner::new(Point::new(0, 0, 0), &beacons);
        } else {
            remaining.push_back(other);
        }
    }
    base.beacons.len()
}

pub fn part_two(input: &str) -> i32 {
    let scanners = parse_input(input);
    let mut remaining: VecDeque<usize> = (1..scanners.len()).collect();
    let mut base = scanners[0].clone();
    let mut positions: Vec<Point> = Vec::new();
    while let Some(other) = remaining.pop_front() {
        if let Some(scanner) = base.find_match(&scanners[other]) {
            let beacons = base
                .beacons
                .union(&scanner.beacons)
                .copied()
                .collect::<Vec<Point>>(); //.map(|p| *p).collect();
            base = Scanner::new(Point::new(0, 0, 0), &beacons);
            positions.push(scanner.position);
        } else {
            remaining.push_back(other);
        }
    }

    let mut max = i32::MIN;
    for (i, a) in positions.iter().enumerate() {
        for b in positions[i + 1..].iter() {
            let distance = a.manhattan_distance(b);
            if distance > max {
                max = distance;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(19);
        assert_eq!(part_one(&input), 79);
    }

    #[test]
    fn example_two() {
        let input = read_example(19);
        assert_eq!(part_two(&input), 3621);
    }
}
