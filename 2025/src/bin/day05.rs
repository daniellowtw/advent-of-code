use std::{collections::HashSet, env, fs};

use itertools::Itertools;

fn part1(pi: (Vec<&str>, Vec<i64>)) -> i32 {
    let ranges =
        pi.0.iter()
            .map(|l| {
                l.split_once("-")
                    .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
                    .unwrap()
            })
            .collect::<Vec<(i64, i64)>>();

    let mut ans = 0;

    for i in pi.1.iter() {
        for (a, b) in ranges.iter() {
            if *i >= *a && *i <= *b {
                ans += 1;
                break;
            }
        }
    }

    return ans;
}

fn part2(pi: Vec<&str>) -> i64 {
    // let mut ranges = HashSet::new();
    let ranges = pi
        .iter()
        .map(|l| {
            l.split_once("-")
                .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
                .unwrap()
        })
        .sorted_by(|(a1, b1), (a2, b2)| a1.cmp(a2))
        .collect::<Vec<(i64, i64)>>();

    let ans = ranges
        .iter()
        .fold((0, (0, 0)), |(accounted, (x, y)), &(a, b)| {
            if a > y {
                return (accounted + (y - x + 1), (a, b));
            }

            if b <= y {
                return (accounted, (x, y));
            }

            return (accounted, (x, b));
        });
    return ans.0 + ans.1.1 - ans.1.0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: [1e, 1, 2e, 2]");
        std::process::exit(1);
    }

    let sel = args[1].as_str();

    let s: String = match sel {
        "1e" | "2e" => fs::read_to_string("./input/example-05.txt").unwrap(),
        "1" | "2" => fs::read_to_string("./input/05.txt").unwrap(),
        _ => {
            eprintln!("Invalid argument. Use 1e, 1, 2e, or 2.");
            std::process::exit(1);
        }
    };
    let ss: (Vec<&str>, Vec<i64>) = s
        .split_once("\n\n")
        .map(|(a, b)| {
            (
                a.lines().collect(),
                b.lines().map(|x| x.parse::<i64>().unwrap()).collect(),
            )
        })
        .unwrap();

    if sel == "1e" || sel == "1" {
        println!("{}", part1(ss));
    } else {
        println!("{}", part2(ss.0));
    }
}
