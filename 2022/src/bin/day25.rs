use std::collections::{VecDeque};
use std::fs;

fn parse(s: &str) -> Vec<Vec<i64>> {
    s.trim_end()
        .split("\n")
        .map(|r| {
            r.chars().rev()
                .map(|c| {
                    match c {
                        '=' => -2,
                        '-' => -1,
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        _ => panic!("Unknown char: {}", c),
                    }
                })
                .collect()
        })
        .collect()
}

fn main() {
    let s: String = fs::read_to_string("./src/input25.txt").unwrap();
    let ss = parse(&s);

    let part1 = solve1(ss);
    dbg!(part1);
}

fn solve1(ss: Vec<Vec<i64>>) -> String {
    let x = ss.iter().map(|r| parse_base_5(r)).sum();
    dbg!(x);
    into_snafu(x)
}

fn parse_base_5(r: &[i64]) -> i64 {
    let mut res = 0;
    for (i, c) in r.iter().enumerate() {
        res += c * 5_i64.pow(i as u32);
    }
    res
}

fn into_snafu(mut n: i64) -> String{
    let mut res = VecDeque::new();
    while n > 0 {
        let r = n % 5;
        match r {
            0 => res.push_front('0'),
            1 => res.push_front('1'),
            2 => res.push_front('2'),
            3 => {
                n += 2;
                res.push_front('=')
            }
            4 => {
                n += 1;
                res.push_front('-')
            }
            _ => panic!("Unknown char: {}", r),
        }
        n /= 5;
    }
    res.into_iter().collect()
}
