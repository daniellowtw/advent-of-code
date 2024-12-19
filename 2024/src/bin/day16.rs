use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

struct PuzzleInput {
    grid: Vec<Vec<char>>,
    start: (i32, i32),
}

fn _display(grid: &Vec<Vec<char>>, seen: &HashSet<(i32, i32)>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if seen.contains(&(i as i32, j as i32)) {
                print!("X");
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!();
    }
}

fn parse(input: &str) -> PuzzleInput {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut start = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }
    PuzzleInput { grid, start }
}

fn to_offset(d: Direction) -> (i32, i32, Direction, Direction) {
    match d {
        Direction::N => (-1, 0, Direction::W, Direction::E),
        Direction::S => (1, 0, Direction::E, Direction::W),
        Direction::W => (0, -1, Direction::S, Direction::N),
        Direction::E => (0, 1, Direction::N, Direction::S),
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Eq, PartialEq)]
struct Data {
    x: i32,
    y: i32,
    score: i32,
    dir: Direction,
    history: Vec<(i32, i32)>,
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score).reverse())
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

fn part1(pi: &PuzzleInput) -> i32 {
    let mut queue: BinaryHeap<Data> = BinaryHeap::new();
    let mut scores = vec![vec![99999999; pi.grid[0].len()]; pi.grid.len()];
    let (x, y) = pi.start;
    queue.push(Data {
        x,
        y,
        score: 0,
        dir: Direction::E,
        history: vec![],
    });

    while !queue.is_empty() {
        let Data {
            x,
            y,
            score,
            dir,
            history: _,
        } = queue.pop().unwrap();
        // println!("{} {} {} {:?}", x, y, score, dir);

        if pi.grid[x as usize][y as usize] == '#' {
            continue;
        }
        if pi.grid[x as usize][y as usize] == 'E' {
            return score;
        }

        if scores[x as usize][y as usize] < score {
            continue;
        } else {
            scores[x as usize][y as usize] = score;
        }

        let (a, b, n1, n2) = to_offset(dir);
        let (nx, ny) = (x + a, y + b);
        queue.push(Data {
            x: nx,
            y: ny,
            score: score + 1,
            dir,
            history: vec![],
        });
        let (a, b, _, _) = to_offset(n1);
        let (nx, ny) = (x + a, y + b);
        queue.push(Data {
            x: nx,
            y: ny,
            score: score + 1 + 1000,
            dir: n1,
            history: vec![],
        });
        let (a, b, _, _) = to_offset(n2);
        let (nx, ny) = (x + a, y + b);
        queue.push(Data {
            x: nx,
            y: ny,
            score: score + 1 + 1000,
            dir: n2,
            history: vec![],
        });
    }

    // _display(&pi.grid, &HashSet::new());
    0
}

fn part2(pi: &PuzzleInput) -> i32 {
    let mut queue: BinaryHeap<Data> = BinaryHeap::new();
    let mut scores = vec![vec![99999999; pi.grid[0].len()]; pi.grid.len()];
    let mut best_path_score = 99999999;
    let (x, y) = pi.start;
    queue.push(Data {
        x,
        y,
        score: 0,
        dir: Direction::E,
        history: vec![(x, y)],
    });
    let mut best_path = vec![];

    while !queue.is_empty() {
        let Data {
            x,
            y,
            score,
            dir,
            history,
        } = queue.pop().unwrap();

        if pi.grid[x as usize][y as usize] == '#' {
            continue;
        }
        if pi.grid[x as usize][y as usize] == 'E' {
            best_path_score = score;
            best_path.push(history.clone());
        }
        if scores[x as usize][y as usize] + 1000 < score {
            continue;
        } else {
            scores[x as usize][y as usize] = score;
        }

        if score > best_path_score {
            break;
        }

        let (a, b, n1, n2) = to_offset(dir);
        let (nx, ny) = (x + a, y + b);
        let mut new_history = history.clone();
        new_history.push((nx, ny));
        queue.push(Data {
            x: nx,
            y: ny,
            score: score + 1,
            dir,
            history: new_history,
        });
        let (a, b, _, _) = to_offset(n1);
        let (nx, ny) = (x + a, y + b);
        let mut new_history = history.clone();
        new_history.push((nx, ny));
        queue.push(Data {
            x: nx,
            y: ny,
            score: score + 1 + 1000,
            dir: n1,
            history: new_history,
        });
        let (a, b, _, _) = to_offset(n2);
        let (nx, ny) = (x + a, y + b);
        let mut new_history = history.clone();
        new_history.push((nx, ny));
        queue.push(Data {
            x: nx,
            y: ny,
            score: score + 1 + 1000,
            dir: n2,
            history: new_history,
        });
    }

    let seen: HashSet<(i32, i32)> = best_path.into_iter().flatten().collect();
    // _display(&pi.grid, &seen);
    seen.len() as i32
}

fn main() {
    let s: String = fs::read_to_string("./input/16.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-16.txt").unwrap();
    let ss = parse(&s);
    println!("{}", part1(&ss));
    println!("{}", part2(&ss));
}
