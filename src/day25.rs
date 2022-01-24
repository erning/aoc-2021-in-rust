use std::collections::HashMap;

type Sea = HashMap<(usize, usize), char>;

fn parse_input(input: &str) -> (usize, usize, Sea) {
    let mut height = 0;
    let mut width = 0;

    let mut sea: Sea = Sea::new();
    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.chars().enumerate() {
            if v == '>' || v == 'v' {
                sea.insert((x, y), v);
            }
        }
        width = line.len();
        height += 1;
    }

    (width, height, sea)
}

pub fn part_one(input: &str) -> i32 {
    let (width, height, mut sea) = parse_input(input);
    let mut i = 0;
    loop {
        i += 1;
        let mut is_moved = false;

        // move east
        let mut new_sea: Sea = sea.clone();
        for (&(x, y), &v) in sea.iter().filter(|(_, &v)| v == '>') {
            let nx = (x + 1) % width;
            if !sea.contains_key(&(nx, y)) {
                new_sea.remove(&(x, y));
                new_sea.insert((nx, y), v);
                is_moved = true;
            }
        }
        sea = new_sea;

        // move south
        let mut new_sea: Sea = sea.clone();
        for (&(x, y), &v) in sea.iter().filter(|(_, &v)| v == 'v') {
            let ny = (y + 1) % height;
            if !sea.contains_key(&(x, ny)) {
                new_sea.remove(&(x, y));
                new_sea.insert((x, ny), v);
                is_moved = true;
            }
        }
        sea = new_sea;

        if !is_moved {
            break i;
        }
    }
}

pub fn part_two(_: &str) -> String {
    String::from("Sleigh keys detected!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(25);
        assert_eq!(part_one(&input), 58);
    }
}
