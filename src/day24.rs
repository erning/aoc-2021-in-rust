use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Registers {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn set(&mut self, n: &str, v: i64) {
        match n {
            "w" => self.w = v,
            "x" => self.x = v,
            "y" => self.y = v,
            "z" => self.z = v,
            _ => unreachable!(),
        }
    }

    fn get(&self, n: &str) -> i64 {
        match n {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            _ => unreachable!(),
        }
    }
}

fn search(program: Vec<&str>, digits: Vec<u8>) -> String {
    struct Env<'a> {
        program: Vec<&'a str>,
        digits: Vec<u8>,
        visited: HashSet<(usize, Registers)>,
    }

    fn validate(
        env: &mut Env,
        pc: usize,
        regs: Registers,
        ds: Vec<u8>,
    ) -> Option<Vec<u8>> {
        if env.visited.contains(&(pc, regs)) {
            return None;
        }
        env.visited.insert((pc, regs));
        if regs.z > 1_000_000 {
            return None;
        }
        if pc >= env.program.len() {
            if regs.z == 0 {
                return Some(ds);
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
                let mut new_ds = ds.clone();
                new_ds.push(d);
                if let Some(rv) = validate(env, pc + 1, new_regs, new_ds) {
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
        validate(env, pc + 1, new_regs, ds)
    }

    let mut env = Env {
        program,
        digits,
        visited: HashSet::new(),
    };

    validate(&mut env, 0, Registers::new(), Vec::new())
        .unwrap()
        .iter()
        .map(|&v| char::from_digit(v as u32, 10).unwrap())
        .collect()
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> String {
    let program = parse_input(input);
    let digits: Vec<u8> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    search(program, digits)
}

pub fn part_two(input: &str) -> String {
    let program = parse_input(input);
    let digits: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    search(program, digits)
}
