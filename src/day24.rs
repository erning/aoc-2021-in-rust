use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Regs(i64, i64, i64, i64);

impl Regs {
    fn set(&mut self, n: &str, v: i64) {
        match n {
            "w" => self.0 = v,
            "x" => self.1 = v,
            "y" => self.2 = v,
            "z" => self.3 = v,
            _ => unreachable!(),
        }
    }

    fn get(&self, n: &str) -> i64 {
        match n {
            "w" => self.0,
            "x" => self.1,
            "y" => self.2,
            "z" => self.3,
            _ => unreachable!(),
        }
    }
}

fn search(program: Vec<&str>, digits: Vec<u8>) -> u64 {
    struct Env<'a> {
        program: Vec<&'a str>,
        digits: Vec<u8>,
        visited: HashSet<(usize, Regs)>,
    }

    fn exec(env: &mut Env, pc: usize, regs: Regs) -> Option<Vec<u8>> {
        if env.visited.contains(&(pc, regs)) {
            return None;
        }
        env.visited.insert((pc, regs));
        if regs.3 > 1_000_000 {
            return None;
        }
        if pc >= env.program.len() {
            if regs.3 == 0 {
                return Some(Vec::with_capacity(14));
            }
            return None;
        }

        let mut new_regs = regs;
        let inst = env.program[pc];
        let op = &inst[..3];
        let a = &inst[4..5];
        if op == "inp" {
            for d in env.digits.clone() {
                new_regs.set(a, d as i64);
                if let Some(mut rv) = exec(env, pc + 1, new_regs) {
                    rv.push(d);
                    return Some(rv);
                }
            }
            return None;
        }
        let b = &inst[6..];
        let av = regs.get(a);
        let bv = match b.parse::<i64>() {
            Ok(v) => v,
            _ => regs.get(b),
        };
        let v = match op {
            "add" => av + bv,
            "mul" => av * bv,
            "div" => av / bv,
            "mod" => av % bv,
            "eql" => {
                if av == bv {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        };
        new_regs.set(a, v);
        exec(env, pc + 1, new_regs)
    }

    let mut env = Env {
        program,
        digits,
        visited: HashSet::new(),
    };

    exec(&mut env, 0, Regs(0, 0, 0, 0))
        .unwrap()
        .iter()
        .rev()
        .fold(0, |acc, &x| acc * 10 + x as u64)
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> u64 {
    let program = parse_input(input);
    let digits: Vec<u8> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    search(program, digits)
}

pub fn part_two(input: &str) -> u64 {
    let program = parse_input(input);
    let digits: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    search(program, digits)
}
