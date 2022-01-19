use std::fmt::Display;

struct Image {
    width: usize,
    height: usize,
    data: Vec<Vec<u8>>,
    default: u8,
}

impl Image {
    fn new(data: Vec<Vec<u8>>, default: u8) -> Image {
        let height = data.len();
        let width = data[0].len();
        Image {
            width,
            height,
            data,
            default,
        }
    }

    fn enhance(&self, enhancement: &[u8]) -> Image {
        let mut output: Vec<Vec<u8>> = Vec::new();

        for y in -1..=self.height as i32 + 1 {
            let mut row: Vec<u8> = Vec::new();
            for x in -1..=self.width as i32 + 1 {
                let mut idx: usize = 0;
                for i in 0..9 {
                    let dx = i % 3 - 1;
                    let dy = i / 3 - 1;
                    let nx = x + dx;
                    let ny = y + dy;
                    let v = if nx < 0
                        || nx >= self.width as i32
                        || ny < 0
                        || ny >= self.height as i32
                    {
                        self.default
                    } else {
                        self.data[ny as usize][nx as usize]
                    };
                    idx = idx << 1 | v as usize;
                }
                row.push(enhancement[idx]);
            }
            output.push(row);
        }

        let outsize: u8 = if enhancement.first().unwrap() == &0 {
            0
        } else if self.default == 1 {
            *enhancement.last().unwrap()
        } else {
            1
        };
        Image::new(output, outsize)
    }

    fn lights(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|&&c| c == 1).count())
            .sum()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.data.iter() {
            for c in row.iter() {
                s.push(if c == &1 { '#' } else { '.' });
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Image) {
    let mut iter = input.lines();
    let enhancement: Vec<u8> = iter
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    iter.next();
    let data: Vec<Vec<u8>> = iter
        .map(|line| {
            line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect()
        })
        .collect();

    let image = Image::new(data, 0);

    (enhancement, image)
}

pub fn part_one(input: &str) -> usize {
    let (enhancement, mut image) = parse_input(input);
    for _ in 0..2 {
        image = image.enhance(&enhancement);
    }
    image.lights()
}

pub fn part_two(input: &str) -> usize {
    let (enhancement, mut image) = parse_input(input);
    for _ in 0..50 {
        image = image.enhance(&enhancement);
    }
    image.lights()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(20);
        assert_eq!(part_one(&input), 35);
    }

    #[test]
    fn example_two() {
        let input = read_example(20);
        assert_eq!(part_two(&input), 3351);
    }
}
