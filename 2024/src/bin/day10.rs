use std::{
    collections::{HashMap, HashSet},
    fs,
};
fn part1(pi: &Vec<Vec<i32>>) -> i32 {
    let h = pi.len();
    let w = pi[0].len();
    let mut stack = vec![];
    let mut score: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            if pi[y][x] == 0 {
                stack.push(((x, y), (x, y), 0));
            }
        }
    }

    // BFS
    while let Some(((x, y), (a, b), val)) = stack.pop() {
        if val == 9 {
            score.entry((a, b)).or_default().insert((x, y));
        }

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                continue;
            }
            if pi[ny as usize][nx as usize] == val + 1 {
                stack.push(((nx as usize, ny as usize), (a, b), val + 1));
            }
        }
    }
    // for (k, v) in score.iter() {
    //     println!("{:?} -> {:?}", k, v);
    // }
    return score.iter().map(|x| x.1.len()).sum::<usize>() as i32;
}

fn part2(pi: &Vec<Vec<i32>>) -> i32 {
    // Basically part 1 but changing the hashSet to a Vec.
    let h = pi.len();
    let w = pi[0].len();
    let mut stack = vec![];
    let mut score: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            if pi[y][x] == 0 {
                stack.push(((x, y), (x, y), 0));
            }
        }
    }

    while let Some(((x, y), (a, b), val)) = stack.pop() {
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                continue;
            }
            if pi[ny as usize][nx as usize] == val + 1 {
                if val == 8 {
                    score.entry((a, b)).or_default().push((x, y));
                } else {
                    stack.push(((nx as usize, ny as usize), (a, b), val + 1));
                }
            }
        }
    }
    // for (k, v) in score.iter() {
    //     println!("{:?} -> {:?}", k, v);
    // }
    return score.iter().map(|x| x.1.len()).sum::<usize>() as i32;
}

fn main() {
    let s: String = fs::read_to_string("./input/10.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-10.txt").unwrap();
    let ss: Vec<Vec<i32>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect())
        .collect();
    println!("{}", part1(&ss));
    println!("{}", part2(&ss));
}
