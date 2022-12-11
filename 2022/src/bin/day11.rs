use std::{collections::HashMap, fs};

struct Monkey {
    items: Vec<i64>,
    // part2: Store modulo of (2, 3, 5, 7, 11, 13, 17, 19)
    items2: Vec<HashMap<i64, i64>>,
    test: (i64, (usize, usize)),
    op: Box<dyn Fn(i64) -> i64>,
    score: i64,
}

impl Monkey {
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

fn hardcoded_monkeys() -> Vec<Monkey> {
    let monkeys = vec![
        Monkey {
            items: vec![],
            items2: vec![],
            test: (19, (6, 7)),
            op: Box::new(|x| x * 11),
            score: 0,
        },
        Monkey {
            items: vec![],
            items2: vec![],
            test: (2, (6, 0)),
            op: Box::new(|x| x + 8),
            score: 0,
        },
        Monkey {
            items: vec![],
            items2: vec![],
            test: (3, (5, 3)),
            op: Box::new(|x| x + 1),
            score: 0,
        },
        Monkey {
            items: vec![],
            items2: vec![],
            test: (17, (5, 4)),
            op: Box::new(|x| x * 7),
            score: 0,
        },
        Monkey {
            items: vec![],
            items2: vec![],
            test: (13, (0, 1)),
            op: Box::new(|x| x + 4),
            score: 0,
        },
        Monkey {
            items: vec![],
            items2: vec![],
            test: (7, (1, 4)),
            op: Box::new(|x| x + 7),
            score: 0,
        },
        Monkey {
            items: vec![],
            items2: vec![],
            test: (5, (7, 2)),
            op: Box::new(|x| x * x),
            score: 0,
        },
        Monkey {
            items: vec![],
            items2: vec![],
            test: (11, (2, 3)),
            op: Box::new(|x| x + 6),
            score: 0,
        },
    ];
    return monkeys;
}

fn main() {
    let s: String = fs::read_to_string("./src/input11.txt").unwrap();
    // Only parse the items and hardcode the monkey instead
    let ss: Vec<Vec<i64>> = s
        .trim_end()
        .split("\n\n")
        .map(|x| {
            let lines: Vec<&str> = x.split("\n").collect();
            let parts = lines[1].split_once(":").unwrap();
            return parts
                .1
                .trim()
                .split(",")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect();
        })
        .collect();

    part1(&ss);
    part2(&ss);
}

fn part1(ss: &Vec<Vec<i64>>) -> () {
    let mut monkeys = hardcoded_monkeys();
    for i in 0..ss.len() {
        monkeys[i].items = ss[i].clone();
    }
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

fn part2(ss: &Vec<Vec<i64>>) -> () {
    let mut monkeys = hardcoded_monkeys();
    for i in 0..ss.len() {
        monkeys[i].items2 = ss[i]
            .iter()
            .map(|x| {
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
            })
            .collect();
    }
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
