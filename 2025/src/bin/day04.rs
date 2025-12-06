use std::{env, fs};

fn neighbours(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < max_x as isize && ny >= 0 && ny < max_y as isize {
                result.push((nx as usize, ny as usize));
            }
        }
    }
    result
}

fn part1(pi: &Vec<Vec<char>>) -> i32 {
    let max_x = pi.len();
    let max_y = pi[0].len();
    let mut ans = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            if pi[x][y] != '@' {
                continue;
            }
            let n = neighbours(x, y, max_x, max_y)
                .into_iter()
                .filter(|&(nx, ny)| pi[nx][ny] == '@')
                .count();
            if n <= 3 {
                ans += 1;
            }
        }
    }

    return ans;
}

fn part2(mut pi: Vec<Vec<char>>) -> i32 {
    let max_x = pi.len();
    let max_y = pi[0].len();
    let mut ans = 0;
    let mut removed = true;
    while removed {
        removed = false;
        for x in 0..max_x {
            for y in 0..max_y {
                if pi[x][y] != '@' {
                    continue;
                }
                let n = neighbours(x, y, max_x, max_y)
                    .into_iter()
                    .filter(|&(nx, ny)| pi[nx][ny] == '@')
                    .count();
                if n <= 3 {
                    ans += 1;
                    pi[x][y] = '.';
                    removed = true;
                }
            }
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
        "1e" | "2e" => fs::read_to_string("./input/example-04.txt").unwrap(),
        "1" | "2" => fs::read_to_string("./input/04.txt").unwrap(),
        _ => {
            eprintln!("Invalid argument. Use 1e, 1, 2e, or 2.");
            std::process::exit(1);
        }
    };
    let ss: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().chars().collect())
        .collect();

    if sel == "1e" || sel == "1" {
        println!("{}", part1(&ss));
    } else {
        println!("{}", part2(ss));
    }
}
