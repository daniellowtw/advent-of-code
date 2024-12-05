use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct PuzzleInput {
    // pub rules: Vec<(i32, i32)>,
    pub rows: Vec<Vec<i32>>,
    pub rules_by_id: HashMap<i32, Vec<i32>>,
}

fn parse(s: &str) -> PuzzleInput {
    let parts: Vec<&str> = s.trim_end().split("\n\n").collect();
    let rules: Vec<(i32, i32)> = parts
        .get(0)
        .unwrap()
        .lines()
        .map(|l| {
            let parsed: Vec<i32> = l.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            (*parsed.get(0).unwrap(), *parsed.get(1).unwrap())
        })
        .collect();

    let rows: Vec<Vec<i32>> = parts
        .get(1)
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();
    let mut rules_by_id: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in rules.iter() {
        rules_by_id
            .entry(i.0)
            .and_modify(|xs| xs.push(i.1))
            .or_insert(vec![i.1]);
    }

    return PuzzleInput { rows, rules_by_id };
}

fn part1(pi: &PuzzleInput) -> i32 {
    let mut count = 0;
    for r in pi.rows.iter() {
        let mut set: HashSet<i32> = HashSet::new();
        let mut valid = true;
        for i in r {
            if let Some(rule) = pi.rules_by_id.get(&i) {
                for x in rule {
                    if set.contains(&x) {
                        valid = false;
                        break;
                    }
                }
                if valid == false {
                    break;
                }
            }
            if valid == false {
                break;
            }
            set.insert(*i);
        }
        if valid {
            count += r.get(r.len() / 2).unwrap();
        }
    }
    return count;
}

fn part2(pi: &PuzzleInput) -> i32 {
    let mut count = 0;
    for r2 in pi.rows.iter() {
        let mut current_order: Vec<i32> = vec![];
        let mut r = r2.clone();
        while !r.is_empty() {
            let i = r.pop().unwrap();
            if let Some(rule) = pi.rules_by_id.get(&i) {
                for x in rule {
                    if current_order.contains(&x) {
                        current_order.retain(|y| *y != *x);
                        r.push(*x);
                    }
                }
            }
            current_order.push(i)
        }
        if current_order != r2.clone() {
            // println!("{:?} -> {:?}", &r2, &current_order);
            count += current_order.get(current_order.len() / 2).unwrap();
        }
    }
    return count;
}

fn main() {
    let s: String = fs::read_to_string("./input/05.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-05.txt").unwrap();
    let pi = parse(&s);
    println!("{}", part1(&pi));
    println!("{}", part2(&pi));
}
