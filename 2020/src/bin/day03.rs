use std::{env, fs};

fn part1(pi: &Vec<Vec<char>>, right: usize, down: usize) -> i64 {
    let mut initial = (0,0);
    let mut count = 0;
    while initial.1 < pi.len() - down {
        initial.0 = (initial.0 + right) % pi[0].len();
        initial.1 += down;
        if pi[initial.1][initial.0] == '#' {
            count += 1;
        }
    }
    return count;
}

fn part2(pi: Vec<Vec<char>>) -> i64 {
    return vec![(1,1), (3,1), (5,1), (7,1), (1,2)].iter().map(|x|part1(&pi, x.0, x.1)).product();
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
    let ss: Vec<Vec<char>> = s.split("\n")
    .filter(|x| !x.is_empty())
    .map(|x| x.chars().collect())
    .collect();

    
    if sel == "1e" || sel == "1" {
        println!("{}", part1(&ss, 3, 1));
    } else {
        println!("{}", part2(ss));
    }
}
