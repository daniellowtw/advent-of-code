use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

fn part1(pi: &str) -> String {
    let (a, b, c, d) = parse(pi);
    let mut sm = StateMachine {
        a,
        b,
        c,
        pi: 0,
        instructions: &d,
        output: vec![],
        done: false,
    };
    while !sm.done {
        sm.consume();
    }
    let mut sb = String::new();
    for i in sm.output {
        sb.push_str(&i.to_string());
        sb.push_str(",");
    }
    return sb[0..sb.len() - 1].to_string();
}

#[derive(Debug)]
struct StateMachine<'a> {
    a: i64,
    b: i64,
    c: i64,
    pi: usize,
    instructions: &'a Vec<i8>,
    output: Vec<i8>,
    done: bool,
}

impl<'a> StateMachine<'a> {
    fn combo(&self, val: i8) -> i64 {
        if val <= 3 {
            return val as i64;
        }
        match val {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid register"),
        }
    }
    fn consume(&mut self) {
        if self.pi >= self.instructions.len() {
            self.done = true;
            return;
        }
        let op = self.instructions[self.pi];
        let val = self.instructions[self.pi + 1];
        self.pi += 2;
        match op {
            0 => {
                let exp = &self.combo(val);
                let denom = 1 << *exp;
                self.a = self.a / denom;
            }
            1 => {
                // Bitwise or
                self.b = self.b ^ val as i64;
            }
            2 => self.b = self.combo(val) % 8,
            3 => {
                if self.a != 0 {
                    self.pi = val as usize;
                }
            }
            4 => self.b = self.b ^ self.c,
            5 => {
                let vv = self.combo(val) % 8;
                self.output.push(vv as i8);
            }
            6 => {
                let exp = &self.combo(val);
                let denom = 1 << *exp;
                self.b = self.a / denom;
            }
            7 => {
                let exp = &self.combo(val);
                let denom = 1 << *exp;
                self.c = self.a / denom;
            }
            _ => panic!("Invalid opcode"),
        }
    }
}

fn part2(pi: &str) -> i64 {
    // For my input, there is exactly one instruction that does the following:
    // Shift value of register A by the mod 8 of the value of register B and store in register C.
    // or cdv 5. The worse case here is we need register A to have at least 7 slots to move,
    // and still have enough bits to cover all values mod 8 (3 more bits)

    let (_, _, _, d) = parse(pi);

    let mut heap = BinaryHeap::new();
    // In theory, this should be 10 to cover the possibility of shifting left by 7 bits.
    for i in 0..1 << 6 {
        heap.push(-i);
    }

    for i in 0..d.len() {
        let mut heap2 = BinaryHeap::new();
        let mut seen: HashSet<i64> = HashSet::new();
        // println!("Checking {}", heap.len());
        while !heap.is_empty() {
            let s = -heap.pop().unwrap();
            if seen.contains(&s) {
                continue;
            }
            seen.insert(s);
            let mut sm = StateMachine {
                a: s,
                b: 0,
                c: 0,
                pi: 0,
                instructions: &d,
                output: vec![],
                done: false,
            };
            let mut too_long = false;
            while !sm.done {
                sm.consume();
                if sm.output.len() > d.len() {
                    too_long = true;
                    break;
                }
            }
            if too_long {
                continue;
            }
            if sm.output == d {
                return s;
            }
            if sm.output.len() <= i {
                continue;
            }
            if !satisfy(&d, i, &sm.output) {
                continue;
            }
            // In theory, this should be 10 to cover the possibility of shifting left by 7 bits.
            for j in 0..1 << 9 {
                let v = j << (s.ilog2() + 1) | s;
                heap2.push(-v);
            }
        }
        heap = heap2;
    }
    return 0;
}

fn satisfy(d: &Vec<i8>, i: usize, sm: &Vec<i8>) -> bool {
    let mut i = i;
    // This is an optimization to end early. We won't be able to change the last 2 values once they are emitted.
    if sm.len() > 2 {
        i = i.max(sm.len() - 2);
    }
    for j in 0..=i {
        if sm[j] != d[j] {
            return false;
        }
    }
    return true;
}

fn parse(input: &str) -> (i64, i64, i64, Vec<i8>) {
    let lines: Vec<&str> = input.split("\n\n").collect();
    let registers: Vec<i64> = lines[0]
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .last()
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap()
        })
        .collect();

    let instructions: Vec<i8> = lines[1]
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|c| c.parse::<i8>().unwrap())
        .collect();
    return (registers[0], registers[1], registers[2], instructions);
}

fn main() {
    let s: String = fs::read_to_string("./input/17.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-17.txt").unwrap();
    println!("{}", part1(&s));
    println!("{}", part2(&s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_a() {
        let mut sm = StateMachine {
            a: 0,
            b: 0,
            c: 9,
            pi: 0,
            instructions: &vec![2, 6],
            output: vec![],
            done: false,
        };
        while !sm.done {
            sm.consume();
        }
        assert!(sm.b == 1)
    }

    #[test]
    fn test_part1_b() {
        let mut sm = StateMachine {
            a: 10,
            b: 0,
            c: 0,
            pi: 0,
            instructions: &vec![5, 0, 5, 1, 5, 4],
            output: vec![],
            done: false,
        };
        while !sm.done {
            sm.consume();
        }
        assert!(sm.output == vec![0, 1, 2]);
    }
    #[test]
    fn test_part1_c() {
        let mut sm = StateMachine {
            a: 2024,
            b: 0,
            c: 0,
            pi: 0,
            instructions: &vec![0, 1, 5, 4, 3, 0],
            output: vec![],
            done: false,
        };
        while !sm.done {
            sm.consume();
        }
        assert!(sm.output == vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert!(sm.a == 0)
    }
    #[test]
    fn test_part1_d() {
        // If register B contains 29, the program 1,7 would set register B to 26.
        let mut sm = StateMachine {
            a: 0,
            b: 29,
            c: 9,
            pi: 0,
            instructions: &vec![1, 7],
            output: vec![],
            done: false,
        };
        while !sm.done {
            sm.consume();
        }
        assert!(sm.b == 26)
    }
    #[test]
    fn test_part1_e() {
        let mut sm = StateMachine {
            a: 0,
            b: 2024,
            c: 43690,
            pi: 0,
            instructions: &vec![4, 0],
            output: vec![],
            done: false,
        };
        while !sm.done {
            sm.consume();
        }
        assert!(sm.b == 44354)
    }
}
