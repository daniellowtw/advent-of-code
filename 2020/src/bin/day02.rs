use std::{env, fs};

fn part1(pi: Vec<Vec<&str>>) -> i32 {
    let mut ans = 0;
    for line in pi {
        let limits: Vec<i32> = line[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let ch = line[1].chars().next().unwrap();
        let password = line[2];
        let count = password.chars().filter(|&c| c == ch).count();
        if count as i32 >= limits[0] && count as i32 <= limits[1] {
            ans += 1;
        }
    }
    return ans;
}

fn part2(pi: Vec<Vec<&str>>) -> i32 {
    let mut ans = 0;
    for line in pi {
        let limits: Vec<i32> = line[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let ch = line[1].chars().next().unwrap();
        let password = line[2];
        if (password.chars().nth((limits[0] - 1) as usize).unwrap() == ch) ^
           (password.chars().nth((limits[1] - 1) as usize).unwrap() == ch) {
            ans += 1;
        }
    }
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
        "1e" | "2e" => fs::read_to_string("./input/example-02.txt").unwrap(),
        "1" | "2" => fs::read_to_string("./input/02.txt").unwrap(),
        _ => {
            eprintln!("Invalid argument. Use 1e, 1, 2e, or 2.");
            std::process::exit(1);
        }
    };
    let ss: Vec<Vec<&str>> = s.split("\n")
    .filter(|x| !x.is_empty())
    .map(|x| x.split_whitespace().collect())
    .collect();

    
    if sel == "1e" || sel == "1" {
        println!("{}", part1(ss));
    } else {
        println!("{}", part2(ss));
    }
}
