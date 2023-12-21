use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

#[derive(Debug, Clone)]
struct PuzzleInput {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
    start: (usize, usize),
}

impl PuzzleInput {
    fn directional_neighbour(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < self.height - 1 {
            neighbours.push((x, y + 1));
        }
        return neighbours;
    }
    pub fn directional_neighbour_with_wrap(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y));
        } else {
            neighbours.push((self.width - 1, y));
        }
        if x < self.width - 1 {
            neighbours.push((x + 1, y));
        } else {
            neighbours.push((0, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        } else {
            neighbours.push((x, self.height - 1));
        }
        if y < self.height - 1 {
            neighbours.push((x, y + 1));
        } else {
            neighbours.push((x, 0));
        }
        return neighbours;
    }
}

fn parse(s: &str) -> PuzzleInput {
    let grid: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().chars().collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut start = (0, 0);
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'S' {
                start = (i, j);
            }
        }
    }
    return PuzzleInput {
        grid,
        start,
        height,
        width,
    };
}

fn succ_1(pi: &PuzzleInput, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    for pos in pi.directional_neighbour(x, y) {
        let c = pi.grid[pos.0][pos.1];
        if c == '#' {
            continue;
        }
        res.insert(pos);
    }
    return res;
}

fn succ_2(pi: &PuzzleInput, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    for pos in pi.directional_neighbour_with_wrap(x, y) {
        let c = pi.grid[pos.0][pos.1];
        if c == '#' {
            continue;
        }
        res.insert(pos);
    }
    return res;
}

fn part2(pi: &PuzzleInput, times: i32) -> i64 {
    // Doesn't work yet. Currently considering finding all paths from all cells, then only count the boundary maps.
    let mut cum_score = HashMap::new();
    cum_score.insert(pi.start, 1 as i64);
    let mut curr_pos = HashSet::new();
    curr_pos.insert(pi.start);
    for _ in 0..times {
        let mut next_pos = HashSet::new();
        for pos in curr_pos.iter() {
            let count = *cum_score.get(pos).unwrap();
            for i in succ_2(&pi, pos.0, pos.1) {
                // dbg!(i);
                let entry = cum_score.entry(i).or_insert(0);
                *entry += count;
                next_pos.insert(i);
            }
        }
        curr_pos = next_pos;
    }
    return cum_score.values().sum::<i64>() as i64;
}

fn part1(pi: &PuzzleInput, times: i32) -> i64 {
    let mut res = HashSet::new();
    res.insert(pi.start);
    for _ in 0..times {
        let mut res2 = HashSet::new();
        for pos in res.iter() {
            for i in succ_1(&pi, pos.0, pos.1) {
                // dbg!(i);
                res2.insert(i);
            }
        }

        // for i in 0..pi.grid.height {
        //     for j in 0..pi.grid.width {
        //         if res2.contains(&(i,j)) {
        //             print!("O");
        //         } else if pi.grid.cells[i][j] == '#' {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        //     println!();
        res = res2;
        // dbg!(&res);
    }

    return res.len() as i64;
}

fn main() {
    let s: String = fs::read_to_string("./src/input21.txt").unwrap();
    let pi: PuzzleInput = parse(s.trim());
    let ans = part1(&pi, 64);
    println!("{}", ans);
    // let ans = part2(pi);
    // println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn test_part1_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim());
        let ans = part1(&pi, 6);
        println!("{}", ans);
    }

    #[test]
    fn test_part2_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim());
        let ans = part2(&pi, 10);
        println!("{}", ans);
    }
}
