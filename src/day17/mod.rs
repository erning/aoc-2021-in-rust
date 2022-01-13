fn parse_input(input: &str) -> (i32, i32, i32, i32) {
    let values: Vec<i32> = input.trim()[13..]
        .split(", ")
        .map(|s| {
            s[2..]
                .split("..")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    (values[0], values[1], values[2], values[3])
}

pub fn part_one(input: &str) -> i32 {
    let (_, _, y1, y2) = parse_input(input);
    for velocity in y1..0 {
        let mut y = 0;
        let mut vy = -velocity;
        while y > y2 {
            y += vy;
            vy -= 1;
        }
        if y < y1 {
            continue;
        }
        return velocity * (velocity - 1) / 2;
    }
    unimplemented!()
}

pub fn part_two(input: &str) -> i32 {
    fn next(x: &mut i32, y: &mut i32, vx: &mut i32, vy: &mut i32) {
        *x += *vx;
        *y += *vy;
        *vx = if *vx > 1 { *vx - 1 } else { 0 };
        *vy -= 1;
    }

    let (x1, x2, y1, y2) = parse_input(input);
    let mut count = 0;
    for velocity_x in 0..=x2 {
        for velocity_y in y1..-y1 {
            let (mut x, mut y) = (0, 0);
            let (mut vx, mut vy) = (velocity_x, velocity_y);
            while x < x1 && y > y2 {
                next(&mut x, &mut y, &mut vx, &mut vy);
            }
            while x <= x2 && y >= y1 {
                if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
                    count += 1;
                    break;
                }
                next(&mut x, &mut y, &mut vx, &mut vy);
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_one() {
        let input = read_example(17);
        assert_eq!(part_one(&input), 45);
    }

    #[test]
    fn example_two() {
        let input = read_example(17);
        assert_eq!(part_two(&input), 112);
    }
}
