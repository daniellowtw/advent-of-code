use std::{collections::HashSet, env, fs};

fn num_digits(m: i64) -> i32 {
    let mut ans = 0;
    let mut n = m;
    while n > 0 {
        ans += 1;
        n /= 10;
    }
    return ans;
}

fn expand(n: i64, repeat: i32) -> i64 {
    n.to_string().repeat(repeat as usize).parse::<i64>().unwrap()
}

fn find_invalids_n((a, b): (i64, i64), repeat: i32) -> HashSet<i64> {
    let n_a = num_digits(a);
    let n_b = num_digits(b);
    let mut want_n_b = n_b;
    if n_b % repeat != 0 {
        want_n_b += repeat;
    }
    // println!("{}-{}: n_a={}, n_b={}", a, b, n_a, n_b);
    let lower = a / 10i64.pow((n_a - n_a / repeat) as u32);
    let upper = b / 10i64.pow((n_b - want_n_b / repeat) as u32);
    // println!("Finding invalids between {} and {}", lower, upper);

    let mut ans = HashSet::<i64>::new();

    for i in lower..=upper {
        let candidate = expand(i, repeat);
        // println!("{}-{}: Candidate: {}", a, b, candidate);
        if (a <= candidate) && (candidate <= b) {
            println!("n={} {}", repeat, candidate);
            ans.insert(candidate);
        }
        if candidate > b {
            break;
        }
    }
    return ans;
}

fn find_invalids((a, b): (i64, i64)) -> HashSet<i64> {
    let n_b = num_digits(b);
    let mut ans = HashSet::<i64>::new();
    for repeat in 2..=n_b {
        ans.extend(find_invalids_n((a, b), repeat));
    }
    return ans;
}

fn part1(pi: Vec<&str>) -> i64 {
    return pi
        .iter()
        .map(|x| {
            x.split_once("-")
                .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
                .unwrap()
        })
        .map(|range| find_invalids_n(range, 2).into_iter().sum::<i64>())
        .sum();
}

fn part2(pi: Vec<&str>) -> i64 {
    return pi
        .iter()
        .map(|x| {
            x.split_once("-")
                .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
                .unwrap()
        })
        .map(|range| find_invalids(range).into_iter().sum::<i64>())
        .sum();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: [1e, 1, 2e, 2]");
        std::process::exit(1);
    }

    let sel = args[1].as_str();

    let s: String = match sel {
        "1e" | "2e" => fs::read_to_string("./input/example-02.txt").unwrap(),
        "1" | "2" => fs::read_to_string("./input/02.txt").unwrap(),
        _ => {
            eprintln!("Invalid argument. Use 1e, 1, 2e, or 2.");
            std::process::exit(1);
        }
    };
    let ss: Vec<&str> = s.split(",").filter(|x| !x.is_empty()).map(|x| x.trim()).collect();

    if sel == "1e" || sel == "1" {
        println!("{}", part1(ss));
    } else {
        println!("{}", part2(ss));
    }
}
