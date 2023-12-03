use std::collections::HashMap;
use std::fs;

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

fn parse_grid(s: String) -> Grid {
    let grid: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().chars().collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();
    return Grid {
        grid,
        width,
        height,
    };
}

fn has_neighbour(i: usize, j: usize, g: &Grid) -> bool {
    let i = i as isize;
    let j = j as isize;
    for x in i - 1..i + 2 {
        for y in j - 1..j + 2 {
            if x == i && y == j {
                continue;
            }
            if x < 0 || y < 0 || x >= g.height as isize || y >= g.width as isize {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if g.grid[x][y].is_numeric() {
                continue;
            }
            if g.grid[x][y] == '.' {
                continue;
            }
            return true;
        }
    }
    return false;
}

fn part1(g: &Grid) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    let mut counted = false;
    let mut number: i32 = 0;
    for i in 0..g.height {
        if counted {
            numbers.push(number);
        }
        number = 0;
        counted = false;

        for j in 0..g.width {
            if !g.grid[i][j].is_numeric() {
                if counted {
                    numbers.push(number);
                }
                number = 0;
                counted = false;
                continue;
            } else {
                number *= 10;
                number += g.grid[i][j].to_digit(10).unwrap() as i32;
                if has_neighbour(i, j, &g) {
                    counted = true;
                }
            }
        }
    }
    return numbers.iter().sum();
}

fn has_neighbour2(i: usize, j: usize, g: &Grid) -> Option<(usize, usize)> {
    // Similar to part1, but returns the position of the neighbour and only if neighhbour is *.
    let i = i as isize;
    let j = j as isize;
    for x in i - 1..i + 2 {
        for y in j - 1..j + 2 {
            if x == i && y == j {
                continue;
            }
            if x < 0 || y < 0 || x >= g.height as isize || y >= g.width as isize {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if g.grid[x][y].is_numeric() {
                continue;
            }
            if g.grid[x][y] == '*' {
                return Some((x, y));
            }
        }
    }
    return None;
}

fn part2(g: &Grid) -> i32 {
    let mut map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut counted = false;
    let mut number: i32 = 0;
    let mut pos: (usize, usize) = (0, 0);

    for i in 0..g.height {
        if counted {
            map.entry(pos).or_insert(vec![]).push(number);
        }
        number = 0;
        pos = (0, 0);
        counted = false;

        for j in 0..g.width {
            if !g.grid[i][j].is_numeric() {
                if counted {
                    map.entry(pos).or_insert(vec![]).push(number);
                }
                number = 0;
                pos = (0, 0);
                counted = false;
                continue;
            } else {
                number *= 10;
                number += g.grid[i][j].to_digit(10).unwrap() as i32;
                match has_neighbour2(i, j, &g) {
                    Some((x, y)) => {
                        counted = true;
                        pos = (x, y);
                    }
                    None => (),
                }
            }
        }
    }

    return map
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product::<i32>())
        .sum();
}

fn main() {
    let s: String = fs::read_to_string("./src/input3.txt").unwrap();
    let grid = parse_grid(s);
    let ss: i32 = part1(&grid);
    println!("{}", ss);
    let ss: i32 = part2(&grid);
    println!("{}", ss);
}
