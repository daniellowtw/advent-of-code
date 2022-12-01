use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let s: String = fs::read_to_string("./src/input20.txt").unwrap();
    let (lookup, input) = s.split_once("\n\n").unwrap();
    let lookup = lookup
        .split("\n")
        .map(|line| line.trim())
        .join("");
    let mut grid = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| line.trim()
            .split("")
            .filter(|x| !x.is_empty())
            .map(|c| c == "#").collect_vec())
        .collect_vec();

    // let num_row = grid.len();
    // let num_col = grid.first().unwrap().len();
    // dbg!(num_row, num_col, lookup.len());

    // Pretty naive solution. Since our input grows with +2 each time, and input size is small,
    // this will do for now.
    // Grid is 100x100, and it grows by 2 each time, it's roughly O(10e6) iterations.
    let start = Instant::now();
    for times in 0..50 {
        // This is tricky! If the first look up is not ., then the outside world "flashes".
        let outside = if times % 2 != 0 { &lookup[0..1] == "#" } else { false };
        grid = simulate(grid, lookup.trim(), outside);
    }
    dbg!(start.elapsed());
    let num_row = grid.len();
    let num_col = grid.first().unwrap().len();

    let mut count = 0;
    for rows in grid {
        for col in rows {
            if col {
                count += 1
            }
        }
    }
    dbg!(num_row, num_col, count);
}

fn simulate(grid: Vec<Vec<bool>>, p1: &str, outside: bool) -> Vec<Vec<bool>> {
    let num_row = grid.len();
    let num_col = grid.first().unwrap().len();

    let mut new_grid: Vec<Vec<bool>> = Vec::with_capacity(num_row + 2);
    for _ in 0..num_row + 2 {
        let mut s: Vec<bool> = Vec::with_capacity(num_col + 2);
        s.resize(num_col + 2, false);
        new_grid.push(s);
    }


    for i in -1..=num_row as isize {
        for j in -1..=num_col as isize {
            let thing = neighbours(i, j).into_iter().map(
                |(ni, nj)| {
                    if ni < 0 || nj < 0 {
                        outside
                    } else {
                        grid.get(ni as usize)
                            .map(|t|
                                *t.get(nj as usize)
                                    .unwrap_or(&outside)).unwrap_or(outside)
                    }
                }
            ).collect_vec();
            let new_val = decode(thing, p1);
            // dbg!(i, j, new_val);
            new_grid[(i + 1) as usize][(j + 1) as usize] = new_val;
        }
    }

    new_grid
}

fn neighbours(i: isize, j: isize) -> Vec<(isize, isize)> {
    let mut candidates: Vec<(isize, isize)> = Vec::with_capacity(9);
    for x in vec![-1, 0, 1] {
        for y in vec![-1, 0, 1] {
            let newi = i + x;
            let newj = j + y;
            candidates.push((newi, newj))
        }
    }
    candidates
}

fn decode(input: Vec<bool>, lookup: &str) -> bool {
    let i = parse_binary_string(input);
    &lookup[i..i + 1] == "#"
}

fn parse_binary_string(input: Vec<bool>) -> usize {
    input.into_iter().fold(0, |b, x1| if x1 { (b << 1) + 1 } else { b << 1 })
}

#[cfg(test)]
mod test {
    use crate::parse_binary_string;

    #[test]
    fn it_works() {
        assert_eq!(parse_binary_string(vec![true, false, false, false]), 8)
    }
}