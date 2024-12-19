use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

struct PuzzleInput {
    pub rules: Vec<(i32, i32)>,
    pub rows: Vec<Vec<i32>>,
    pub rules_by_id: HashMap<i32, Vec<i32>>,
}

fn parse(s: &str) -> PuzzleInput {
    let parts: Vec<&str> = s.trim_end().split("\n\n").collect();
    let rules: Vec<(i32, i32)> = parts.first()
        .unwrap()
        .lines()
        .map(|l| {
            let parsed: Vec<i32> = l.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            (*parsed.first().unwrap(), *parsed.get(1).unwrap())
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

    PuzzleInput {
        rules,
        rows,
        rules_by_id,
    }
}

fn part1(pi: &PuzzleInput) -> i32 {
    return pi.rows.iter().fold(0, |count, row| {
        let mut s: HashSet<i32> = HashSet::new();
        if row.iter().all(|x| {
            let valid = pi
                .rules_by_id
                .get(x)
                .unwrap()
                .iter()
                .all(|y| !s.contains(y));
            s.insert(*x);
            valid
        }) {
            count + row.get(row.len() / 2).unwrap()
        } else {
            count
        }
    });
}

fn part2(pi: &PuzzleInput) -> i32 {
    return pi.rows.iter().fold(0, |count, row| {
        let mut correct_order = row.clone();
        for i in 0..correct_order.len() {
            for j in i + 1..correct_order.len() {
                for r in &pi.rules {
                    if r.0 == correct_order[j] && r.1 == correct_order[i] {
                        correct_order.swap(i, j);
                    }
                }
            }
        }

        if correct_order != *row {
            // println!("{:?} -> {:?}", &r2, &current_order);
            count + *correct_order.get(correct_order.len() / 2).unwrap()
        } else {
            count
        }
    });
}

fn part2_alternative(pi: &PuzzleInput) -> i32 {
    return pi.rows.iter().fold(0, |count, row| {
        let mut correct_order = row.clone();
        correct_order.sort_by(|a, b| {
            for r in &pi.rules {
                if r.0 == *b && r.1 == *a {
                    return Ordering::Greater;
                } else if r.0 == *a && r.1 == *b {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
        if correct_order != *row {
            // println!("{:?} -> {:?}", &r2, &current_order);
            count + *correct_order.get(correct_order.len() / 2).unwrap()
        } else {
            count
        }
    });
}

fn main() {
    let s: String = fs::read_to_string("./input/05.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-05.txt").unwrap();
    let pi = parse(&s);
    println!("{}", part1(&pi));
    println!("{}", part2_alternative(&pi));
}
