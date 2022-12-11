use std::{collections::HashMap, fs};

struct Monkey {
    items: Vec<i64>,
    // part2: Store modulo of (2, 3, 5, 7, 11, 13, 17, 19)
    items2: Vec<HashMap<i64, i64>>,
    test: (i64, (usize, usize)),
    op: Box<dyn Fn(i64) -> i64>,
    score: i64,
}

fn into_divisors(x: &i64) -> HashMap<i64, i64> {
    let mut m = HashMap::new();
    m.insert(2, *x % 2);
    m.insert(3, *x % 3);
    m.insert(5, *x % 5);
    m.insert(7, *x % 7);
    m.insert(11, *x % 11);
    m.insert(13, *x % 13);
    m.insert(17, *x % 17);
    m.insert(19, *x % 19);
    return m;
}

impl Monkey {
    fn new(ss: &str) -> Self {
        // Monkey 0:
        //   Starting items: 74, 73, 57, 77, 74
        //   Operation: new = old * 11
        //   Test: divisible by 19
        //     If true: throw to monkey 6
        //     If false: throw to monkey 7

        let lines: Vec<&str> = ss.split("\n").collect();
        let parts = lines[1].split_once(":").unwrap();
        let items: Vec<i64> = parts
            .1
            .trim()
            .split(",")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        let items2: Vec<HashMap<i64, i64>> = items.iter().map(into_divisors).collect();
        let op_parts: Vec<&str> = lines[2].split(" ").collect();
        let op_fn = op_parts[op_parts.len() - 2];
        let op: Box<dyn Fn(i64) -> i64> = match (op_fn, op_parts[op_parts.len() - 1]) {
            (x, "old") => match x {
                "*" => Box::new(|x| x * x),
                _ => panic!("Unknown operation"),
            },
            (x, y) => {
                let y = y.parse::<i64>().unwrap();
                let z: Box<dyn Fn(i64) -> i64> = match x {
                    "+" => Box::new(move |a: i64| a + y),
                    "*" => Box::new(move |a: i64| a * y),
                    _ => panic!("Unknown operation"),
                };
                z
            }
        };
        let get_last_number = |x: &str| {
            return x.split(" ").last().unwrap().trim().parse::<i64>().unwrap();
        };
        let divisor = get_last_number(lines[3]);
        let when_true = get_last_number(lines[4]) as usize;
        let when_false = get_last_number(lines[5]) as usize;

        Self {
            items: items,
            items2: items2,
            test: (divisor, (when_true, when_false)),
            op: op,
            score: 0,
        }
    }
    fn do_turn2(&mut self) -> Vec<(HashMap<i64, i64>, usize)> {
        self.score += self.items2.len() as i64;
        let copy = self.items2.clone();
        self.items2 = vec![];
        return copy
            .into_iter()
            .map(|mut x| {
                for (k, v) in x.iter_mut() {
                    *v = (self.op)(*v) % *k;
                }
                if x[&self.test.0] == 0 {
                    return (x, self.test.1 .0);
                } else {
                    return (x, self.test.1 .1);
                }
            })
            .collect();
    }

    fn do_turn(&mut self) -> Vec<(i64, usize)> {
        self.score += self.items.len() as i64;
        let copy = self.items.clone();
        self.items = vec![];
        return copy
            .iter()
            .map(|x| {
                let y = ((self.op)(*x)) / 3;
                if y % self.test.0 == 0 {
                    return (y, self.test.1 .0);
                } else {
                    return (y, self.test.1 .1);
                }
            })
            .collect();
    }
}

fn main() {
    let s: String = fs::read_to_string("./src/input11.txt").unwrap();
    // Only parse the items and hardcode the monkey instead
    let mut ss: Vec<Monkey> = s.trim_end().split("\n\n").map(Monkey::new).collect();

    part1(&mut ss);

    let mut ss: Vec<Monkey> = s.trim_end().split("\n\n").map(Monkey::new).collect();
    part2(&mut ss);
}

fn part1(monkeys: &mut Vec<Monkey>) -> () {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let active = &mut monkeys[i];
            let actions = active.do_turn();
            for (i, a) in actions.iter() {
                monkeys[*a].items.push(*i);
            }
        }
    }

    let mut scores: Vec<i64> = monkeys.iter().map(|x| x.score).collect();
    scores.sort_by(|a, b| b.cmp(a));
    println!("{:?}", scores[0] * scores[1]);
}

fn part2(monkeys: &mut Vec<Monkey>) -> () {
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let active = &mut monkeys[i];
            let actions = active.do_turn2();
            for (i, a) in actions.into_iter() {
                monkeys[a].items2.push(i);
            }
        }
    }

    let mut scores: Vec<i64> = monkeys.iter().map(|x| x.score).collect();
    scores.sort_by(|a, b| b.cmp(a));
    println!("{:?}", scores[0] * scores[1]);
}
