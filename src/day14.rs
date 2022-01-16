use std::collections::HashMap;

type Pair = (char, char);
type Rules = HashMap<Pair, char>;
type Elements = HashMap<char, i64>;
type Cache = HashMap<(Pair, i32), Elements>;

fn parse_input(input: &str) -> (&str, Rules) {
    let mut lines = input.lines();
    let template = lines.next().unwrap();
    lines.next();
    let mut rules: HashMap<Pair, char> = HashMap::new();
    for line in lines.by_ref() {
        let mut chars = line.chars();
        let pair = (chars.next().unwrap(), chars.next().unwrap());
        rules.insert(pair, chars.next_back().unwrap());
    }
    (template, rules)
}

fn merge(to: &mut Elements, from: &Elements) {
    for (&k, &v) in from.iter() {
        let nv = match to.get(&k) {
            Some(x) => v + x,
            None => v,
        };
        to.insert(k, nv);
    }
}

fn grow_pair(
    cache: &mut Cache,
    rules: &Rules,
    pair: Pair,
    depth: i32,
) -> Elements {
    let cache_key = (pair, depth);
    if let Some(elements) = cache.get(&cache_key) {
        return elements.clone();
    }

    let mut elements: Elements = HashMap::new();
    let c = rules.get(&pair);
    if c.is_none() {
        return elements;
    }
    let &c = c.unwrap();
    elements.insert(c, 1);
    if depth <= 1 {
        return elements;
    }

    let (a, b) = pair;
    let lhs = grow_pair(cache, rules, (a, c), depth - 1);
    let rhs = grow_pair(cache, rules, (c, b), depth - 1);
    merge(&mut elements, &lhs);
    merge(&mut elements, &rhs);

    cache.insert(cache_key, elements.clone());
    elements
}

fn grow(template: &str, rules: &Rules, depth: i32) -> i64 {
    let mut cache: Cache = HashMap::new();

    let mut elements: Elements = HashMap::new();

    let mut chars = template.chars();
    let mut a = chars.next().unwrap();
    for b in chars.by_ref() {
        let subelements = grow_pair(&mut cache, rules, (a, b), depth);
        merge(&mut elements, &subelements);
        a = b;
    }

    for c in template.chars() {
        let v = match elements.get(&c) {
            Some(v) => v + 1,
            None => 1,
        };
        elements.insert(c, v);
    }

    let (_, max) = elements.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    let (_, min) = elements.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    max - min
}

pub fn part_one(input: &str) -> i64 {
    let (template, rules) = parse_input(input);
    grow(template, &rules, 10)
}

pub fn part_two(input: &str) -> i64 {
    let (template, rules) = parse_input(input);
    grow(template, &rules, 40)
}
