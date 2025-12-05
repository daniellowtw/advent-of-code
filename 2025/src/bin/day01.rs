use std::{env, fs};

fn part1(pi: Vec<&str>) -> i32 {
    let mut start = 50;
    let mut ans = 0;
    let instructions: Vec<(char, i32)> = pi
        .iter()
        .map(|l| {
            let parts = l.split_at(1);
            return (parts.0.chars().next().unwrap(), parts.1.parse::<i32>().unwrap());
        })
        .collect();
    instructions.iter().for_each(|(a, b)| {
        match a {
            'L' => start = (start - b) % 100,
            'R' => start = (start + b) % 100,
            _ => {}
        }
        if start == 0 {
            ans += 1;
        }
    });
    return ans;
}

fn part2(pi: Vec<&str>) -> i32 {
    let mut start = 50;
    let mut ans = 0;
    let instructions: Vec<(char, i32)> = pi
        .iter()
        .map(|l| {
            let parts = l.split_at(1);
            return (parts.0.chars().next().unwrap(), parts.1.parse::<i32>().unwrap());
        })
        .collect();
    instructions.iter().for_each(|(a, b)| {
        match a {
            'L' => {
                start = (-start + 100) % 100;
                start += b;
                ans += start / 100;
                start = start % 100;
                start = (100-start) % 100;
            }
            'R' => {
                start += b;
                ans += start / 100;
                start = start % 100;
            }
            _ => {}
        }
    });
    return ans;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: [1e, 1, 2e, 2]");
        std::process::exit(1);
    }

    let sel = args[1].as_str();

    let s: String = match sel {
        "1e" | "2e" => fs::read_to_string("./input/example-01.txt").unwrap(),
        "1" | "2" => fs::read_to_string("./input/01.txt").unwrap(),
        _ => {
            eprintln!("Invalid argument. Use 1e, 1, 2e, or 2.");
            std::process::exit(1);
        }
    };
    let ss: Vec<&str> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    if sel == "1e" || sel == "1" {
        println!("{}", part1(ss));
    } else {
        println!("{}", part2(ss));
    }
}
