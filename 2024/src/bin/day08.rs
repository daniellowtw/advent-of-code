use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn adjacent_points(a: &(i32, i32), b: &(i32, i32)) -> ((i32, i32), (i32, i32)) {
    if a.0 > b.0 {
        return adjacent_points(b, a);
    }
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    return ((a.0 - dx, a.1 - dy), (b.0 + dx, b.1 + dy));
}

fn points_extended(a: &(i32, i32), b: &(i32, i32), w: i32, h: i32) -> Vec<(i32, i32)> {
    if a.0 > b.0 {
        return points_extended(b, a, w, h);
    }
    let mut res = vec![];
    res.push(*a);
    res.push(*b);
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    for i in 1.. {
        let nx = a.0 - dx * i;
        let ny = a.1 - dy * i;
        if nx < 0 || ny < 0 || nx >= w || ny >= h {
            break;
        }
        res.push((nx, ny));
    }
    for i in 1.. {
        let nx = b.0 + dx * i;
        let ny = b.1 + dy * i;
        if nx < 0 || ny < 0 || nx >= w || ny >= h {
            break;
        }
        res.push((nx, ny));
    }
    return res;
}

fn _print_board(pi: &Vec<Vec<char>>, points: &HashSet<(i32, i32)>) {
    let w = pi[0].len();
    let h = pi.len();
    for y in 0..h {
        for x in 0..w {
            if points.contains(&(x as i32, y as i32)) {
                print!("X");
            } else {
                print!("{}", pi[y][x]);
            }
        }
        println!();
    }
}

fn part1(pi: &Vec<Vec<char>>) -> i32 {
    let w = pi[0].len();
    let h = pi.len();
    let mut groups: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let c = pi[y][x];
            if c != '.' {
                let group = groups.entry(c).or_insert(HashSet::new());
                group.insert((x as i32, y as i32));
            }
        }
    }

    let mut points: HashSet<(i32, i32)> = HashSet::new();
    for (_, group) in groups.iter() {
        // get all combinations of pairs of points
        for combination in group.into_iter().combinations(2) {
            let (a, b) = adjacent_points(combination[0], combination[1]);
            if !(a.0 < 0 || a.1 < 0 || a.0 >= w as i32 || a.1 >= h as i32) {
                points.insert(a);
            }
            if !(b.0 < 0 || b.1 < 0 || b.0 >= w as i32 || b.1 >= h as i32) {
                points.insert(b);
            }
        }
    }

    // print_board(&pi, &points);
    return points.len() as i32;
}

fn part2(pi: &Vec<Vec<char>>) -> i32 {
    let w = pi[0].len();
    let h = pi.len();
    let mut groups: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let c = pi[y][x];
            if c != '.' {
                let group = groups.entry(c).or_insert(HashSet::new());
                group.insert((x as i32, y as i32));
            }
        }
    }

    let mut points: HashSet<(i32, i32)> = HashSet::new();
    for (_, group) in groups.iter() {
        for combination in group.into_iter().combinations(2) {
            points_extended(combination[0], combination[1], w as i32, h as i32)
                .iter()
                .for_each(|p| {
                    points.insert(*p);
                });
        }
    }

    // _print_board(&pi, &points);
    return points.len() as i32;
}

fn main() {
    let s: String = fs::read_to_string("./input/08.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-08.txt").unwrap();
    let ss: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    println!("{}", part1(&ss));
    println!("{}", part2(&ss));
}
