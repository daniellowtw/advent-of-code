use std::fs;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

struct PuzzleInput {
    pub rules: Vec<(i32, i32)>,
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

    return PuzzleInput {
        rules,
        rows,
        rules_by_id,
    };
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
            return valid;
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
            return Ordering::Equal;
        });
        if correct_order != *row {
            // println!("{:?} -> {:?}", &r2, &current_order);
            count + *correct_order.get(correct_order.len() / 2).unwrap()
        } else {
            count
        }
    });
}

fn part2_old(pi: &PuzzleInput) -> i32 {
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

fn benchmark_day5(c: &mut Criterion) {
    let s: String = fs::read_to_string("./input/05.txt").unwrap();
    let pi = parse(&s);
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("part2");
    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("old", i), i, |b, i| {
            b.iter(|| part2_old(&pi))
        });
        group.bench_with_input(BenchmarkId::new("insertion", i), i, |b, i| {
            b.iter(|| part2(&pi))
        });
        group.bench_with_input(BenchmarkId::new("sort", i), i, |b, i| {
            b.iter(|| part2_alternative(&pi))
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_day5);
criterion_main!(benches);
