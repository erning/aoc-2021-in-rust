#[derive(Debug, Copy, Clone)]
struct Segment {
    a: i64,
    b: i64,
    size: i64,
}

impl Segment {
    fn new(a: i64, b: i64) -> Segment {
        assert!(a <= b);
        let size = b - a + 1;
        Segment { a, b, size }
    }

    fn intersect(&self, other: &Segment) -> Option<Segment> {
        let i = std::cmp::max(self.a, other.a);
        let j = std::cmp::min(self.b, other.b);
        if i <= j {
            Some(Segment::new(i, j))
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cube {
    x: Segment,
    y: Segment,
    z: Segment,
    size: i64,
}

impl Cube {
    fn new(x: Segment, y: Segment, z: Segment) -> Cube {
        let size = x.size * y.size * z.size;
        Cube { x, y, z, size }
    }

    fn intersect(&self, other: &Cube) -> Option<Cube> {
        let xi = self.x.intersect(&other.x);
        let yi = self.y.intersect(&other.y);
        let zi = self.z.intersect(&other.z);
        match (xi, yi, zi) {
            (Some(xi), Some(yi), Some(zi)) => Some(Cube::new(xi, yi, zi)),
            _ => None,
        }
    }
}

type Instruction = (bool, Cube);

#[derive(Debug)]
struct Reactor {
    instructions: Vec<Instruction>,
}

impl Reactor {
    fn new() -> Reactor {
        Reactor {
            instructions: Vec::new(),
        }
    }

    fn size(&self) -> i64 {
        self.instructions
            .iter()
            .map(|(operator, operand)| match operator {
                true => operand.size,
                false => -operand.size,
            })
            .sum()
    }

    fn turn(&mut self, is_on: bool, cube: Cube) {
        let mut additions: Vec<Instruction> = Vec::new();
        for (operator, operand) in self.instructions.iter() {
            if let Some(isect) = cube.intersect(operand) {
                additions.push((!operator, isect));
            }
        }
        if is_on {
            self.instructions.push((true, cube))
        }
        self.instructions.append(&mut additions);
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (operator, line) = line.split_once(' ').unwrap();
            let operator = operator == "on";
            let v: Vec<i64> = line
                .split(',')
                .map(|s| s.split('=').nth(1).unwrap())
                .flat_map(|s| s.split(".."))
                .map(|s| s.parse().unwrap())
                .collect();
            let x = Segment::new(v[0], v[1]);
            let y = Segment::new(v[2], v[3]);
            let z = Segment::new(v[4], v[5]);
            let operand = Cube::new(x, y, z);
            (operator, operand)
        })
        .collect()
}

pub fn part_one(input: &str) -> i64 {
    let steps = parse_input(input);
    let mut reactor = Reactor::new();
    for (operator, operand) in steps {
        if operand.x.a < -50 || operand.x.b > 50 {
            continue;
        }
        if operand.y.a < -50 || operand.y.b > 50 {
            continue;
        }
        if operand.z.a < -50 || operand.z.b > 50 {
            continue;
        }
        reactor.turn(operator, operand);
    }
    reactor.size()
}

pub fn part_two(input: &str) -> i64 {
    let steps = parse_input(input);
    let mut reactor = Reactor::new();
    for (operator, operand) in steps {
        reactor.turn(operator, operand);
    }
    reactor.size()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_example, read_as_string};

    #[test]
    fn example_one() {
        let input = read_example(22);
        assert_eq!(part_one(&input), 590784);
    }

    #[test]
    fn example_two() {
        let input = read_as_string(22, "example2");
        assert_eq!(part_two(&input), 2758514936282235);
    }
}
