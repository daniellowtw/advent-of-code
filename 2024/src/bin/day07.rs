use rayon::prelude::*;
use std::{collections::VecDeque, fs};

fn conc(a: i64, b2: i64) -> i64 {
    // Previously I inlined this and was using .to_string().len() and raising 10 to that power. It's slower.
    // This shaved off another 0.5s
    let mut b = b2;
    let mut factor = 1;
    while b > 0 {
        b /= 10;
        factor *= 10;
    }
    a * factor + b2
}

fn solvable2(target: &i64, numbers: &Vec<i64>) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(('+', 0, 0));
    while queue.len() > 0 {
        // dbg!(queue.len());
        let (op, idx, acc) = queue.pop_front().unwrap();

        if idx == numbers.len() {
            if acc == *target {
                return true;
            }
        } else {
            let acc = match op {
                '+' => acc + numbers[idx],
                '*' => acc * numbers[idx],
                '^' => conc(acc, numbers[idx]),
                _ => panic!("Invalid operator"),
            };
            // cuts from 2.3s -> 1.2s
            if acc > *target {
                continue;
            }

            // This is a bit more aggressive pruning from the other direction.
            // The following reduced it by another 0.1s.
            let mut is_possible = false;
            let mut acc2 = acc;
            for x in numbers[idx..].iter() {
                acc2 = conc(acc2, *x);
                if acc2 >= *target {
                    is_possible = true;
                    break;
                }
            }
            if is_possible {
                queue.push_back(('^', idx + 1, acc));
                queue.push_back(('+', idx + 1, acc));
                queue.push_back(('*', idx + 1, acc));
            }
        }
    }

    return false;
}

fn solvable(target: &i64, numbers: &Vec<i64>) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(('+', 0, 0));
    while queue.len() > 0 {
        let (op, idx, acc) = queue.pop_front().unwrap();

        if idx == numbers.len() {
            if acc == *target {
                return true;
            }
        } else {
            let acc = match op {
                '+' => acc + numbers[idx],
                '*' => acc * numbers[idx],
                _ => panic!("Invalid operator"),
            };
            queue.push_back(('+', idx + 1, acc));
            queue.push_back(('*', idx + 1, acc));
        }
    }

    return false;
}

fn part1(pi: &Vec<(i64, Vec<i64>)>) -> i64 {
    pi.iter()
        .filter(|(target, numbers)| solvable(target, numbers))
        .map(|(a, _)| a)
        .sum()
}

fn part2(pi: &Vec<(i64, Vec<i64>)>) -> i64 {
    // The last cherry on top is to use par_iter. This brought it down from 0.6s -> 0.23s
    pi.par_iter()
        .filter(|(target, numbers)| solvable2(target, numbers))
        .map(|(a, _)| a)
        .sum()
}

fn main() {
    let s: String = fs::read_to_string("./input/07.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-07.txt").unwrap();
    let ss: Vec<(i64, Vec<i64>)> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|row| {
            let parts: Vec<&str> = row.split(":").collect();
            let rest = parts
                .get(1)
                .unwrap()
                .split_whitespace()
                .map(|y| y.parse().unwrap())
                .collect();
            (parts.get(0).unwrap().parse().unwrap(), rest)
        })
        .collect();
    println!("{}", part1(&ss));
    println!("{}", part2(&ss));
}
