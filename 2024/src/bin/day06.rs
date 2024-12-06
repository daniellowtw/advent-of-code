use std::{collections::HashSet, fs};

fn next_move(pi: &PuzzleInput, x: i32, y: i32, dir: Direction) -> Option<(i32, i32, Direction)> {
    let (a, b) = dir.to_offset();
    let next_pos = (x + a, y + b);
    if !pi.is_valid(next_pos.0, next_pos.1) {
        return None;
    }
    if pi.val(next_pos.0, next_pos.1) == '#' {
        let dir = dir.next();
        let (a, b) = dir.to_offset();
        let next_pos = (x + a, y + b);
        if !pi.is_valid(next_pos.0, next_pos.1) {
            return None;
        }
        return Some((next_pos.0, next_pos.1, dir));
    } else {
        return Some((next_pos.0, next_pos.1, dir));
    }
}

#[derive(Debug, Clone)]
struct PuzzleInput {
    pub width: usize,
    pub height: usize,
    pub start: (usize, usize),
    pub grid: Vec<Vec<char>>,
}

impl PuzzleInput {
    fn is_valid(&self, x: i32, y: i32) -> bool {
        return x >= 0 && y >= 0 && x < self.height as i32 && y < self.width as i32;
    }
    fn val(&self, x: i32, y: i32) -> char {
        self.grid[x as usize][y as usize]
    }
}

fn parse(s: &str) -> PuzzleInput {
    // Copied from last year.
    let grid: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().chars().collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    let start = (0..height)
        .flat_map(|x| (0..width).map(move |y| (x, y)))
        .find(|(x, y)| grid[*x as usize][*y as usize] == '^')
        .unwrap();
    return PuzzleInput {
        grid,
        start,
        height,
        width,
    };
}
fn is_loop(pi: &PuzzleInput) -> bool {
    let mut curr_pos = (pi.start.0 as i32, pi.start.1 as i32);
    let mut current_dir = Direction::Up;
    // My first attempt used a new enum type for the 4 directions.
    // Not using Enum cuts from 5s -> 1s
    // let mut visited_with_dir: HashSet<(i32, i32, i8)> = HashSet::new();
    // This is fast but doing the rotation is annoying. Complex numbers gives some ergonomics but is slightly slower.
    let mut visited_with_dir: HashSet<(i32, i32, i8)> = HashSet::new();
    loop {
        if !pi.is_valid(curr_pos.0, curr_pos.1) {
            return false;
        }
        match next_move(&pi, curr_pos.0, curr_pos.1, current_dir) {
            Some((nx, ny, next_dir)) => {
                if current_dir != next_dir {
                    if visited_with_dir.contains(&(curr_pos.0, curr_pos.1, next_dir as i8)) {
                        return true;
                    }
                    // This is a huge optimization. Only store the corners.
                    // Storing every single history takes about 20s. Storing corners only takes 5s
                    visited_with_dir.insert((curr_pos.0, curr_pos.1, next_dir as i8));
                    // Very important to only change direction. Because we might not be able to move if the new pos is a #
                    current_dir = next_dir;
                } else {
                    curr_pos = (nx, ny);
                }
            }
            None => return false,
        }
    }
}

fn move_until_out(pi: &PuzzleInput) -> HashSet<(i32, i32)> {
    let mut curr_pos = (pi.start.0 as i32, pi.start.1 as i32);
    let mut current_dir = Direction::Up;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    while pi.is_valid(curr_pos.0, curr_pos.1) {
        visited.insert(curr_pos);
        match next_move(&pi, curr_pos.0, curr_pos.1, current_dir) {
            Some((nx, ny, next_dir)) => {
                curr_pos = (nx, ny);
                current_dir = next_dir;
            }
            None => break,
        }
    }
    return visited;
}

fn part1(pi: &PuzzleInput) -> i32 {
    // Originally I inlined this function. But part 2 reuses this, so I refactored out.
    return move_until_out(pi).len() as i32;
}

fn part2(pi: &mut PuzzleInput) -> i32 {
    let visited = move_until_out(&pi);
    return visited
        .into_iter()
        .filter(|&(x, y)| {
            pi.grid[x as usize][y as usize] = '#';
            let valid = is_loop(&pi);
            pi.grid[x as usize][y as usize] = '.';
            return valid;
        })
        .count() as i32;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
    fn next(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

// Ignore
fn _part2_old(pi: &PuzzleInput) -> HashSet<(i32, i32)> {
    // This was the naive approach I tried first. Took 90s.
    // This tried way too many positions that are inconsequential.
    let mut pi = pi.clone();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..pi.height {
        for j in 0..pi.width {
            if pi.grid[i][j] == '.' {
                pi.grid[i][j] = '#';
                if part1(&pi) == 0 {
                    visited.insert((i as i32, j as i32));
                }
                pi.grid[i][j] = '.';
            }
        }
    }
    return visited;
}

fn main() {
    let s: String = fs::read_to_string("./input/06.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-06.txt").unwrap();
    let mut pi = parse(&s);
    println!("{}", part1(&pi));
    println!("{}", part2(&mut pi));
}
