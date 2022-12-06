use std::{collections::HashSet, fs};

fn main() {
    let s: String = fs::read_to_string("./src/input6.txt").unwrap();
    let ss: i32 = s.trim_end().split("\n").map(|a| solve(a, 4) as i32).sum();
    println!("Part 1: {}", ss);
    let ss: i32 = s.trim_end().split("\n").map(|a| solve(a, 14) as i32).sum();
    println!("Part 2: {}", ss);
}

fn solve(s: &str, n: usize) -> usize {
    // PS: I didn't bother to optimize. If I were to redo, I would use two iterators, one fast and one slow, and kept a char counter.
    for i in 0..s.len() - n - 1 {
        let tmp: HashSet<char> = s[i..i + n].chars().collect();
        if tmp.len() == n {
            // println!("{}", &s[i..i+n]);
            return i + n;
        }
    }
    return 0;
}
