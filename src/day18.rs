use std::fmt::Display;
use std::ops::Add;

#[derive(Debug)]
enum Node {
    V(u32),
    P(Box<Node>, Box<Node>),
    Empty,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Empty => write!(f, ""),
            Node::V(v) => write!(f, "{}", v),
            Node::P(lhs, rhs) => write!(f, "[{},{}]", lhs, rhs),
        }
    }
}

impl Node {
    fn is_empty(&self) -> bool {
        matches!(self, Node::Empty)
    }

    fn is_value(&self) -> bool {
        matches!(self, Node::V(_))
    }

    fn is_value_pair(&self) -> bool {
        matches!(self, Node::P(lhs, rhs) if lhs.is_value() && rhs.is_value())
    }

    fn value(&self) -> u32 {
        match self {
            Node::V(v) => *v,
            _ => panic!("not value"),
        }
    }

    fn pair_value(&self) -> (u32, u32) {
        match self {
            Node::P(a, b) => (a.value(), b.value()),
            _ => panic!("not pair value"),
        }
    }

    fn explode(&mut self) -> bool {
        let mut en: Option<&mut Node> = None;
        let mut ln: Option<&mut Node> = None;
        let mut rn: Option<&mut Node> = None;

        let mut stack: Vec<(i32, &mut Node)> = vec![(0, self)];

        while let Some((depth, node)) = stack.pop() {
            if en.is_none() && depth >= 4 && node.is_value_pair() {
                en = Some(node);
            } else {
                match node {
                    Node::P(lhs, rhs) => {
                        stack.push((depth + 1, rhs));
                        stack.push((depth + 1, lhs));
                    }
                    Node::V(_) => {
                        if en.is_none() {
                            ln = Some(node);
                        } else if rn.is_none() {
                            rn = Some(node);
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(en) = en {
            let (a, b) = en.pair_value();
            if let Some(ln) = ln {
                *ln = Node::V(ln.value() + a);
            }
            if let Some(rn) = rn {
                *rn = Node::V(rn.value() + b);
            }
            *en = Node::V(0);
            return true;
        }
        false
    }

    fn split(&mut self) -> bool {
        match self {
            Node::V(v) if *v >= 10 => {
                let a = *v / 2;
                let b = *v - a;
                let lhs = Node::V(a);
                let rhs = Node::V(b);
                *self = Node::P(Box::new(lhs), Box::new(rhs));
                true
            }
            Node::P(lhs, rhs) => lhs.split() || rhs.split(),
            _ => false,
        }
    }

    fn reduce(&mut self) -> bool {
        let mut count = 0;
        while self.explode() || self.split() {
            count += 1;
        }
        count > 0
    }

    fn magnitude(&self) -> u32 {
        match self {
            Node::V(v) => *v,
            Node::P(lhs, rhs) => lhs.magnitude() * 3 + rhs.magnitude() * 2,
            _ => 0,
        }
    }
}

impl Add for Node {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.is_empty() {
            return other;
        }
        if other.is_empty() {
            return self;
        }
        let mut node = Node::P(Box::new(self), Box::new(other));
        node.reduce();
        node
    }
}

impl From<&str> for Node {
    fn from(item: &str) -> Self {
        let mut stack: Vec<Node> = Vec::new();
        let mut iter = item.chars().peekable();
        while let Some(c) = iter.next() {
            match c {
                '[' => {}
                ']' => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    let node = Node::P(Box::new(lhs), Box::new(rhs));
                    stack.push(node);
                }
                ',' => {}
                '0'..='9' => {
                    let mut v = c.to_digit(10).unwrap();
                    while let Some(c) = iter.peek() {
                        if !c.is_digit(10) {
                            break;
                        }
                        v = v * 10 + c.to_digit(10).unwrap();
                        iter.next();
                    }
                    let node = Node::V(v);
                    stack.push(node);
                }
                _ => {}
            }
        }
        stack.pop().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Node> {
    input.lines().map(Node::from).collect()
}

pub fn part_one(input: &str) -> u32 {
    let nodes = parse_input(input);
    let mut sum = Node::Empty;
    for node in nodes {
        sum = sum + node;
    }
    sum.magnitude()
}

pub fn part_two(input: &str) -> u32 {
    let mut max = u32::MIN;
    for (ia, a) in input.lines().enumerate() {
        for (ib, b) in input.lines().enumerate() {
            if ia == ib {
                continue;
            }
            let a = Node::from(a);
            let b = Node::from(b);
            let c = a + b;
            let m = c.magnitude();
            if m > max {
                max = m;
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
        let input = read_example(18);
        assert_eq!(part_one(&input), 4140);
    }

    #[test]
    fn example_two() {
        let input = read_example(18);
        assert_eq!(part_two(&input), 3993);
    }
}
