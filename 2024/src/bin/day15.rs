use std::{collections::HashSet, fs};

fn to_offset(c: char) -> (i32, i32) {
    match c {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("Invalid direction {}", c),
    }
}

fn part1(mut pi: PuzzleInput) -> i32 {
    // Just implementing what it says on the tin.
    // Probably can be written in a more succinct way.
    let mut curr_pos = pi.start;
    for i in pi.instructions.clone() {
        // println!("{} {:?}", i, curr_pos);
        let (a, b) = to_offset(i);
        let next_pos = (curr_pos.0 + a, curr_pos.1 + b);
        if pi.is_wall(next_pos.0, next_pos.1) {
            continue;
        }
        if pi.val(next_pos.0, next_pos.1) == 'O' {
            // Follow the chain until a wall
            let mut n_pos = (next_pos.0, next_pos.1);
            let mut n_val = pi.val(n_pos.0, n_pos.1);
            while pi.val(n_pos.0, n_pos.1) == 'O' {
                n_pos = (n_pos.0 + a, n_pos.1 + b);
                n_val = pi.val(n_pos.0, n_pos.1);
            }
            match n_val {
                '#' => {}
                '.' => {
                    pi.grid[n_pos.0 as usize][n_pos.1 as usize] = 'O';
                    pi.grid[next_pos.0 as usize][next_pos.1 as usize] = '.';
                    curr_pos = next_pos;
                }
                _ => {
                    panic!("unexpected")
                }
            }
        } else {
            curr_pos = next_pos;
        }
        // _display(&pi, curr_pos);
    }
    let mut score = 0;
    for i in 0..pi.grid.len() {
        for j in 0..pi.grid[0].len() {
            if pi.grid[i][j] == 'O' {
                score += 100 * i + j;
            }
        }
    }
    score as i32
}

fn move_box(pi: &mut PuzzleInput, b_idx: usize, dir: (i32, i32)) -> bool {
    let b = pi.boxes[b_idx];
    let new_pos = (b.0 + dir.0, b.1 + dir.1);
    if pi.walls.contains(&new_pos) || pi.walls.contains(&(new_pos.0, new_pos.1 + 1)) {
        return false;
    }
    let blocking = pi.boxes_in_the_way(vec![(new_pos.0, new_pos.1), (new_pos.0, new_pos.1 + 1)]);

    for idx in blocking {
        if idx == b_idx {
            continue;
        }
        if !move_box(pi, idx, dir) {
            return false;
        }
    }
    pi.boxes[b_idx] = new_pos;
    true
}

fn part2(mut pi: PuzzleInput) -> i64 {
    let start = (pi.start.0, pi.start.1 * 2);
    for (_, j) in pi.boxes.iter_mut() {
        *j *= 2;
    }
    pi.walls = pi
        .walls
        .iter()
        .flat_map(|w| [(w.0, w.1 * 2), (w.0, w.1 * 2 + 1)])
        .collect();

    let mut curr_pos = start;
    for i in pi.instructions.clone() {
        // println!("{} {:?}", i, curr_pos);
        let (a, b) = to_offset(i);
        let next_pos = (curr_pos.0 + a, curr_pos.1 + b);
        if pi.is_wall(next_pos.0, next_pos.1) {
            continue;
        }
        let boxes_to_move = pi.boxes_in_the_way(vec![next_pos]);
        // dbg!(boxes_to_move.clone());
        if boxes_to_move.is_empty() {
            curr_pos = next_pos;
            continue;
        }

        // Save the values before we proceed. If we cannot move, then we reset
        let curr_boxes = pi.boxes.clone();
        let curr_grid = pi.grid.clone();
        let mut can_move = true;
        for idx in boxes_to_move {
            // println!("Trying to move box {} at {:?}", idx, pi.boxes[idx]);
            if !move_box(&mut pi, idx, (a, b)) {
                pi.grid = curr_grid;
                pi.boxes = curr_boxes;
                can_move = false;
                break;
            }
            // println!("Moved box {} -> {:?}", idx, pi.boxes[idx]);
        }
        if can_move {
            // println!("Moved player to {:?}", next_pos);
            curr_pos = next_pos;
        } else {
            // println!("Cannot move player to {:?}", next_pos);
        }
        // pi.display(curr_pos);
    }
    // pi.display(curr_pos);
    pi.score()
}

#[derive(Clone)]
struct PuzzleInput {
    grid: Vec<Vec<char>>,
    boxes: Vec<(i32, i32)>,
    walls: HashSet<(i32, i32)>,
    instructions: Vec<char>,
    start: (i32, i32), // (h, w)
}

impl PuzzleInput {
    fn boxes_in_the_way(self: &PuzzleInput, b: Vec<(i32, i32)>) -> HashSet<usize> {
        let blocking = self
            .boxes
            .iter()
            .enumerate()
            .filter(|(_, (bh, bw))| {
                // Return if blocked
                b.iter()
                    .any(|(ch, cw)| *bh == *ch && (*bw == *cw || *bw + 1 == *cw))
            })
            .map(|x| x.0)
            .collect();
        blocking
    }
    fn is_wall(self: &PuzzleInput, h: i32, w: i32) -> bool {
        self.walls.contains(&(h, w))
    }
    fn val(self: &PuzzleInput, h: i32, w: i32) -> char {
        self.grid[h as usize][w as usize]
    }
    fn score(self: &PuzzleInput) -> i64 {
        let mut score = 0;
        self.boxes.iter().for_each(|(h, w)| {
            score += 100 * *h as i64 + *w as i64;
        });
        score
    }
}
fn _display(pi: &PuzzleInput, pos: (i32, i32)) {
    let height = pi.grid.len();
    let width = pi.grid[0].len();
    for i in 0..height as i32 {
        for j in 0..width as i32 {
            if pos == (i, j) {
                print!("@");
                continue;
            }
            let val = pi.grid[i as usize][j as usize];
            if val == '@' {
                print!(".")
            } else {
                print!("{}", val)
            }
        }
        println!();
    }
}

fn _display2(pi: &PuzzleInput, pos: (i32, i32)) {
    let height = pi.grid.len();
    let width = pi.grid[0].len();
    for i in 0..height as i32 {
        for j in 0..width as i32 {
            if pos == (i, j) {
                print!("@");
                continue;
            }
            if pi.walls.contains(&(i, j)) {
                print!("#");
                continue;
            }
            if pi.boxes.contains(&(i, j)) {
                print!("[");
                continue;
            }
            if pi.boxes.contains(&(i, j - 1)) {
                print!("]");
                continue;
            }
            print!(".");
            // print!("{}", self.grid[i as usize][j as usize]);
        }
        println!();
    }
}

fn parse(s: &str) -> PuzzleInput {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let mut grid: Vec<Vec<char>> = parts[0].split("\n").map(|x| x.chars().collect()).collect();
    let instructions: Vec<char> = parts[1].replace("\n", "").trim().chars().collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut start: (i32, i32) = (0, 0);
    let mut boxes: Vec<(i32, i32)> = vec![];
    let mut walls: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..height {
        for j in 0..width {
            match grid[i as usize][j as usize] {
                '#' => {
                    walls.insert((i, j));
                    // walls.insert((i, 2 * j));
                    // walls.insert((i, 2 * j + 1));
                }
                '@' => {
                    start = (i, j);
                    grid[i as usize][j as usize] = '.';
                }
                'O' => {
                    // boxes.push((i, 2 * j));
                    boxes.push((i, j));
                }
                '.' => {}
                _ => panic!("Invalid char {}", grid[i as usize][j as usize]),
            }
        }
    }
    PuzzleInput {
        grid,
        instructions,
        start,
        boxes,
        walls,
    }
}

fn main() {
    let s: String = fs::read_to_string("./input/15.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-15.txt").unwrap();
    let pi = parse(&s);
    println!("{}", part1(pi.clone()));
    println!("{}", part2(pi));
}
