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

fn search(map: &HashMap<&str, Vec<&str>>, enable_twice: bool) -> usize {
    struct Env<'a> {
        map: &'a HashMap<&'a str, Vec<&'a str>>,
        visited: HashSet<&'a str>,
        enable_twice: bool,
        twiced: Option<&'a str>,
        count: usize,
    }

    fn dfs(mut env: &mut Env, node: &str) {
        if node == "end" {
            env.count += 1;
            return;
        }
        for next in env.map.get(node).unwrap() {
            if env.visited.contains(next) {
                if !env.enable_twice || env.twiced.is_some() {
                    continue;
                }
                env.twiced = Some(next);
            }
            if next.chars().next().unwrap().is_lowercase() {
                env.visited.insert(next);
            }
            dfs(env, next);
            if env.enable_twice && env.twiced == Some(next) {
                env.twiced = None
            } else {
                env.visited.remove(next);
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert("start");

    let mut env = Env {
        map,
        visited,
        enable_twice,
        twiced: None,
        count: 0,
    };

    dfs(&mut env, "start");
    env.count
}

pub fn part_one(input: &str) -> usize {
    let map = parse_input(input);
    search(&map, false)
}

pub fn part_two(input: &str) -> usize {
    let map = parse_input(input);
    search(&map, true)
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
