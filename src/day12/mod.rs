use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let input: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split('-').collect())
        .map(|nodes: Vec<&str>| (nodes[0], nodes[1]))
        .collect();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (a, b) in input {
        match (a, b) {
            ("start", _) | (_, "end") => {
                map.entry(a).or_default().push(b);
            }
            ("end", _) | (_, "start") => {
                map.entry(b).or_default().push(a);
            }
            _ => {
                map.entry(a).or_default().push(b);
                map.entry(b).or_default().push(a);
            }
        }
    }
    map
}

fn dfs(
    map: &HashMap<&str, Vec<&str>>,
    visited: &HashSet<&str>,
    node: &str,
    times: i8,
) -> i32 {
    if node == "end" {
        return 1;
    }
    let mut count = 0;
    for next in map.get(node).unwrap() {
        let mut times = times;
        if visited.contains(next) {
            if times <= 0 {
                continue;
            }
            times -= 1;
        }
        let mut visited = visited.clone();
        if next.chars().next().unwrap().is_lowercase() {
            visited.insert(next);
        }
        count += dfs(map, &visited, next, times);
    }
    count
}

pub fn part_one(input: &str) -> i32 {
    let map = parse_input(input);
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert("start");
    dfs(&map, &visited, "start", 0)
}

pub fn part_two(input: &str) -> i32 {
    let map = parse_input(input);
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert("start");
    dfs(&map, &visited, "start", 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    const LARGER: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EVER_LARGER: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn example_one() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 10);
        assert_eq!(part_one(LARGER), 19);
        assert_eq!(part_one(EVER_LARGER), 226);
    }

    #[test]
    fn example_two() {
        let input = read_example(12);
        assert_eq!(part_two(&input), 36);
        assert_eq!(part_two(LARGER), 103);
        assert_eq!(part_two(EVER_LARGER), 3509);
    }
}
