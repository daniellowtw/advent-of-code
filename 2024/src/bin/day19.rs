use std::{collections::HashMap, fs};

fn count_solvable(
    cache: &mut HashMap<String, i64>,
    towels: &Vec<&str>,
    pattern: &str,
) -> i64 {
    if pattern.is_empty() {
        return 1;
    }
    if cache.contains_key(pattern) {
        return cache[pattern];
    }

    let mut count = 0;
    for t in towels {
        if pattern.starts_with(t) {
            // Perhaps starting from the back means the stack size is smaller.
            // println!("{} -> {}, {}", t, pattern, count);
            let new_pattern = &pattern[t.len()..];
            count += count_solvable(cache, towels, new_pattern)
        }
    }
    cache.insert(pattern.to_string(), count);
    count
}

fn is_solvable(towels: &Vec<&str>, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }

    for t in towels {
        if pattern.starts_with(t) {
            let new_pattern = &pattern[t.len()..];
            if is_solvable(towels, new_pattern) {
                return true;
            }
        }
    }

    false
}

fn part1(s: &str) -> i32 {
    let (towels, patterns) = parse(s);
    let mut count = 0;
    for p in patterns {
        if is_solvable(&towels, p) {
            count += 1;
        }
    }
    count
}

fn part2(s: &str) -> i64 {
    // The key idea here is caching.
    let (towels, patterns) = parse(s);
    // println!("{:?} {}", towels, towels.len());
    let mut count = 0;
    let mut cache = HashMap::new();
    for p in patterns {
        let x = count_solvable(&mut cache, &towels, p);
        count += x;
        // println!("{} -> {}", p, x);
    }
    count
}

fn parse(s: &str) -> (Vec<&str>, Vec<&str>) {
    let ss = s.trim().split("\n\n").collect::<Vec<&str>>();
    let towels = ss[0].split(",").map(|x| x.trim()).collect::<Vec<&str>>();
    let patterns = ss[1].lines().collect();
    (towels, patterns)
}

fn main() {
    let s: String = fs::read_to_string("./input/19.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-19.txt").unwrap();
    println!("{}", part1(&s));
    println!("{}", part2(&s));
}
