use std::{collections::HashMap, collections::VecDeque, fs};

fn part1(pi: &Vec<Vec<char>>) -> i32 {
    let mut scores: Vec<(char, i32, i32)> = Vec::new();
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let h = pi.len();
    let w = pi[0].len();
    for i in 0..h {
        for j in 0..w {
            let mut area = 0;
            let mut parameter = 0;
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back((i, j));
            while queue.len() > 0 {
                let (i, j) = queue.pop_front().unwrap();
                if visited.contains_key(&(i, j)) {
                    continue;
                }
                visited.insert((i, j), true);
                let c = pi[i][j];
                area += 1;

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let y = i as i32 + dy;
                    let x = j as i32 + dx;

                    if y < 0 || y >= h as i32 || x < 0 || x >= w as i32 {
                        parameter += 1;
                        continue;
                    }
                    let cc = pi[y as usize][x as usize];
                    if cc == c {
                        queue.push_back((y as usize, x as usize));
                    } else {
                        parameter += 1;
                    }
                }
            }
            if area > 0 {
                scores.push((pi[i][j], area, parameter));
            }
        }
    }

    let mut score = 0;
    for (_k, v, p) in scores {
        println!("{}: {} * {}", _k, v, p);
        score += v * p
    }

    return score;
}

fn is_valid(i: i32, j: i32, h: usize, w: usize) -> bool {
    return i >= 0 && i < h as i32 && j >= 0 && j < w as i32;
}

fn has_edge(
    a: char,
    i1: i32,
    j1: i32,
    i2: i32,
    j2: i32,
    h: usize,
    w: usize,
    pi: &Vec<Vec<char>>,
) -> bool {
    if !is_valid(i1, j1, h, w) {
        return false;
    } else {
        if a != pi[i1 as usize][j1 as usize] {
            return false;
        }
        if !is_valid(i2, j2, h, w) {
            return true;
        }
        return pi[i1 as usize][j1 as usize] != pi[i2 as usize][j2 as usize];
    }
}

fn part2(pi: &Vec<Vec<char>>) -> i32 {
    let mut scores: Vec<(char, i32, i32)> = Vec::new();
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let h = pi.len();
    let w = pi[0].len();
    for i in 0..h {
        for j in 0..w {
            let mut area = 0;
            let mut parameter = 0;
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back((i, j));
            while queue.len() > 0 {
                let (i, j) = queue.pop_front().unwrap();
                if visited.contains_key(&(i, j)) {
                    continue;
                }
                visited.insert((i, j), true);
                let c = pi[i][j];
                area += 1;
                let ii = i as i32;
                let jj = j as i32;

                // Check left
                if has_edge(c, ii, jj, ii, jj - 1, h, w, pi) {
                    // Check top and ask are we the end of the segment
                    if !has_edge(c, ii - 1, jj, ii - 1, jj - 1, h, w, pi) {
                        parameter += 1;
                    }
                    if !has_edge(c, ii + 1, jj, ii + 1, jj - 1, h, w, pi) {
                        parameter += 1;
                    }
                }
                // check right
                if has_edge(c, ii, jj, ii, jj + 1, h, w, pi) {
                    if !has_edge(c, ii - 1, jj, ii - 1, jj + 1, h, w, pi) {
                        parameter += 1;
                    }
                    if !has_edge(c, ii + 1, jj, ii + 1, jj + 1, h, w, pi) {
                        parameter += 1;
                    }
                }
                // check top
                if has_edge(c, ii, jj, ii - 1, jj, h, w, pi) {
                    if !has_edge(c, ii, jj - 1, ii - 1, jj - 1, h, w, pi) {
                        parameter += 1;
                    }
                    if !has_edge(c, ii, jj + 1, ii - 1, jj + 1, h, w, pi) {
                        parameter += 1;
                    }
                }
                // check bottom
                if has_edge(c, ii, jj, ii + 1, jj, h, w, pi) {
                    if !has_edge(c, ii, jj - 1, ii + 1, jj - 1, h, w, pi) {
                        parameter += 1;
                    }
                    if !has_edge(c, ii, jj + 1, ii + 1, jj + 1, h, w, pi) {
                        parameter += 1;
                    }
                }

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let y = i as i32 + dy;
                    let x = j as i32 + dx;

                    if y < 0 || y >= h as i32 || x < 0 || x >= w as i32 {
                        // parameter += 1;
                        continue;
                    }
                    let cc = pi[y as usize][x as usize];
                    if cc == c {
                        queue.push_back((y as usize, x as usize));
                    }
                }
            }
            if area > 0 {
                scores.push((pi[i][j], area, parameter / 2));
            }
        }
    }

    let mut score = 0;
    for (_k, v, p) in scores {
        println!("{}: {} * {}", _k, v, p);
        score += v * p
    }

    return score;
}

fn main() {
    let s: String = fs::read_to_string("./input/12.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-12.txt").unwrap();
    let ss: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    // println!("{}", part1(&ss));
    println!("{}", part2(&ss));
}
