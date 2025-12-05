use std::{env, fs};

use itertools::Itertools;

fn to_num(xs: &Vec<i8>) -> i64 {
    let mut ans = 0;
    for x in xs {
        ans = ans * 10 + (*x as i64);
    }
    return ans;
}

fn aux(xs: &Vec<i8>, n: i32) -> Option<Vec<i8>> {
    let nn = n as usize;
    if xs.len() < nn {
        return None;
    }
    if xs.len() == nn {
        return Some(xs.clone());
    }
    let lxs = xs.len();
    let candidates = &xs[0..lxs - (nn - 1)];
    let x = candidates.iter().max().unwrap();
    if n == 1 {
        return Some(vec![*x]);
    } else {
        let max_pos = candidates.iter().positions(|y| y == x);
        return max_pos
            .flat_map(|start| {
                aux(&xs[start + 1..].to_vec(), n - 1).map(|mut v| {
                    let mut res = vec![*x];
                    res.append(&mut v);
                    res
                })
            })
            .max_by(|a, b| {
                if to_num(a) < to_num(b) {
                    std::cmp::Ordering::Less
                } else if to_num(a) > to_num(b) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
    }
}

fn find_max_joltz_12(s: &str) -> i64 {
    let values = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect::<Vec<i8>>();
    let ans = aux(&values, 12);
    return to_num(&ans.unwrap());
}

fn find_max_joltz(s: &str) -> i32 {
    // Only choose 2.
    let values = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    let mut max = 0;
    for i in 0..values.len() {
        for j in i + 1..values.len() {
            let c = values[i] * 10 + values[j];
            if max < c {
                max = c
            }
        }
    }
    return max;
}

fn part1(pi: Vec<&str>) -> i32 {
    return pi.into_iter().map(find_max_joltz).sum();
}

fn part2(pi: Vec<&str>) -> i64 {
    return pi.into_iter().map(find_max_joltz_12).sum();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: [1e, 1, 2e, 2]");
        std::process::exit(1);
    }

    let sel = args[1].as_str();

    let s: String = match sel {
        "1e" | "2e" => fs::read_to_string("./input/example-03.txt").unwrap(),
        "1" | "2" => fs::read_to_string("./input/03.txt").unwrap(),
        _ => {
            eprintln!("Invalid argument. Use 1e, 1, 2e, or 2.");
            std::process::exit(1);
        }
    };
    let ss: Vec<&str> = s.split("\n").filter(|x| !x.is_empty()).collect();

    if sel == "1e" || sel == "1" {
        println!("{}", part1(ss));
    } else {
        println!("{}", part2(ss));
    }
}
