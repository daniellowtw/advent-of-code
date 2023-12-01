use std::{collections::HashMap, fs, io::empty};

fn part1(s: &str) -> i32 {
    let x: Vec<char> = s.chars().filter(|x| !x.is_alphabetic()).collect();
    // println!("{:?}", x);
    let a = x.first().unwrap().to_digit(10).unwrap() as i32;
    let b = x.last().unwrap().to_digit(10).unwrap() as i32;
    return a * 10 + b;
}

fn find_num_at_pos_i(s: &str, i: usize) -> Option<u32> {
    let map: HashMap<&str, &str> = [
        ("zero", "9990"), // Maybe won't hit
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();
    let c = s.chars().nth(i).unwrap();
    if !c.is_alphabetic() {
        return Some(c.to_digit(10).unwrap());
    }
    for (&k, &v) in map.iter() {
        if s[i..].starts_with(k) {
            return Some(v.chars().nth(0).unwrap().to_digit(10).unwrap());
        }
    }
    return None;
}

fn part2(s: &str) -> i32 {
    // Originally started with just iterating through the hashmap 
    // and replacing the string with the number. But this ran into issues
    // such as "eightwo1" which should return 81 but will become 21.
    // Then I tried to do replacement one the first occurrence, but that will
    // affect values like "eightwo". This should return 82, but will become 88.
    // Hence, replacement is the wrong idea. So instead, just search and return 
    // the first and last without replacement.
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    for i in 0..s.len() {
        if let Some(x) = find_num_at_pos_i(s, i) {
            first = x;
            break;
        }
    }

    for i in (0..s.len()).rev() {
        if let Some(x) = find_num_at_pos_i(s, i) {
            last = x;
            break;
        }
    }
    // println!("{} -> {} {}", s, first, last);
    let ans = first * 10 + last;
    return ans as i32;
}

fn main() {
    let s: String = fs::read_to_string("./src/input1.txt").unwrap();
    let ss: i32 = s.split("\n").filter(|x| !x.is_empty()).map(part1).sum();
    println!("{}", ss);
    let ss: i32 = s.split("\n").filter(|x| !x.is_empty()).map(part2).sum();
    println!("{}", ss);
}
