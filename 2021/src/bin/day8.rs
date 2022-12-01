use std::collections::{HashMap, HashSet};
use std::fs;
use std::process::exit;

use itertools::Itertools;

fn main() {
    let s: String = fs::read_to_string("./src/input8.txt").unwrap();
    let ss: Vec<(&str, &str)> = s.split("\n").
        filter(|x| { !x.is_empty() }).
        map(|x| {
            let xs: Vec<&str> = x.split(" | ").collect();
            (xs[0], xs[1])
        }).
        collect(); // collect();

    // Part 1 is straight forward since we can easily identify those by the size of the string.
    // let mut count = 0;
    // for (_, x) in ss {
    //     count += x.split(" ").filter(|y| {
    //         match y.len() {
    //             2 | 7 | 3 | 4 => true,
    //             _ => false
    //         }
    //     }).fold(0, |acc, _| { acc + 1 });
    // }
    // dbg!(count);

    // Part 2 requires solving it by by encoding the heuristic as code.
    let mut final_score= 0;
    for (i, (clue, puzzle)) in ss.iter().enumerate() {
        let lookup = solve(clue);
        let s = puzzle.split(" ").map(|x| {
            let key = x.chars().sorted().collect::<String>();
            lookup.get(&key).unwrap()
        });
        let score = (s).fold(0, |acc, b| { (acc + *b) * 10 }) / 10;
        dbg!(i, score);
        final_score += score;
    };
    dbg!(final_score);
}

// Originally I also wrote code to identify the mapping of each input's char to the char in the
// canonical diagram. But then I realized that the heuristic is actually much simpler if we just
// compare the set differences of the input.
fn solve(input: &str) -> HashMap<String, i32> {
    let mut s: HashMap<String, i32> = input.split(" ").map(|x| {
        let key = x.chars().sorted().collect::<String>();
        (key, -1)
    }).collect();
    // Solve for 1, 7, 4, 8
    let mut seven = HashSet::new();
    let mut four = HashSet::new();
    let mut one = HashSet::new();
    let mut eight = HashSet::new();
    let to_set = |one: &str| { one.chars().collect::<HashSet<char>>() };

    for (k, v) in &mut s {
        let _ = match k.len() {
            2 => {
                *v = 1;
                one = to_set(k);
            }
            4 => {
                *v = 4;
                four = to_set(k);
            }
            7 => {
                *v = 8;
                eight = to_set(k);
            }
            3 => {
                *v = 7;
                seven = to_set(k);
            }
            _ => ()
        };
    }

    // 4 can identify 9 from the cluster {0, 6, 9}
    let mut nine = HashSet::new();
    for (k, v) in &mut s {
        if *v != -1 || k.len() != 6 { continue; }
        let s = to_set(k).difference(&four).collect::<Vec<&char>>().len();
        if s == 2 {
            *v = 9;
            nine = to_set(k);
            break;
        }
    }

    // diff between 8 and 9 can identify e
    let e = **eight.difference(&nine).collect::<Vec<&char>>().first().unwrap();
    let mut two = HashSet::new();

    // {2, 3, 5} + e can identify 2, it's the only one with e
    for (k, v) in &mut s {
        if *v != -1 { continue; }
        if k.len() == 5 && k.contains(e) {
            *v = 2;
            two = to_set(k);
        }
    }

    // We can now distinguish 3 and 5 using 2
    for (k, v) in &mut s {
        if *v != -1 { continue; }
        if k.len() == 5 {
            let l = to_set(k).difference(&two).collect::<Vec<&char>>().len();
            if l == 2 {
                *v = 5;
            } else if l == 1 {
                *v = 3;
            }
        }
    }

    // What's left is 6 and 0. We can differentiate them by checking for intersection with 1
    for (k, v) in &mut s {
        if *v != -1 { continue; }
        let l = to_set(k).intersection(&one).collect::<Vec<&char>>().len();
        if l == 1 {
            *v = 6;
        } else if l == 2 {
            *v = 0;
        }
    }

    // Just to make sure we have everything.
    for (k, v) in &s {
        if *v == -1 {
            dbg!(k);
            exit(-1)
        }
    }

    return s;
}
