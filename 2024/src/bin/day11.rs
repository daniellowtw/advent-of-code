use std::{collections::HashMap, fs};

fn _part1(pi: &Vec<i64>, steps: i64) -> i64 {
    let mut curr_stack: Vec<i64> = pi.clone();
    for _ in 0..steps {
        let mut next_stack: Vec<i64> = Vec::new();
        for i in 0..curr_stack.len() {
            let n = curr_stack[i];
            for i in f(n) {
                next_stack.push(i);
            }
        }
        // println!("{:?}", &next_stack);
        curr_stack = next_stack;
    }
    return curr_stack.len() as i64;
}

fn f(n: i64) -> Vec<i64> {
    if n == 0 {
        return vec![1];
    } else {
        let n_len = n.to_string().len();
        if n_len % 2 == 0 {
            let ss = 10_i64.pow((n_len / 2) as u32);
            // println!("{} -> {} {}", n, n / ss, n & ss);
            return vec![n / ss, n % ss];
        } else {
            return vec![n * 2024];
        }
    }
}

fn part2(pi: &Vec<i64>, steps: i64) -> i64 {
    let mut cache: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut curr_freq: HashMap<i64, i64> = pi.iter().map(|x| (*x, 1)).collect();
    for _step in 0..steps {
        let mut next_freq: HashMap<i64, i64> = HashMap::new();
        for (n, freq) in curr_freq {
            for j in cache.entry(n).or_insert(f(n)) {
                let count = next_freq.entry(*j).or_insert(0);
                *count += freq;
            }
        }
        // println!("{} -> {:?}", _step + 1, freq_map);
        // println!("{} -> {:?}", _step + 1, total);
        curr_freq = next_freq;
    }
    let total = curr_freq.iter().fold(0, |acc, x| acc + x.1);
    return total;
}

fn main() {
    // let s = include_str!("../../input/11.txt");
    let s: String = fs::read_to_string("./input/11.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-11.txt").unwrap();
    let ss: Vec<i64> = s
        .trim()
        .split_whitespace()
        .map(|y| y.parse::<i64>().unwrap())
        .collect();
    println!("{}", part2(&ss, 25));
    println!("{}", part2(&ss, 75));
}
