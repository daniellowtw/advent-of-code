use std::fs;

#[derive(Debug, Clone)]
struct PuzzleInput {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
}

impl PuzzleInput {
    fn is_valid(&self, x: i32, y: i32) -> bool {
        return x >= 0 && y >= 0 && x < self.height as i32 && y < self.width as i32;
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
    return PuzzleInput {
        grid,
        height,
        width,
    };
}

fn part1(ss: &PuzzleInput) -> i32 {
    // Idea: pick a direction and walk 4 times.
    let offsets: Vec<(i32, i32)> = (-1..=1)
        .flat_map(|i| (-1..=1).map(move |j| (i, j)))
        .filter(|x| *x != (0, 0))
        .collect();
    let target: Vec<char> = "XMAS".chars().collect();
    let count = (0..ss.height)
        .flat_map(|i| (0..ss.width).map(move |j| (i, j)))
        .flat_map(|(i, j)| offsets.iter().map(move |offset| (i, j, offset)))
        .filter(|&(i, j, offset)| {
            target
                .iter()
                .enumerate()
                .map(|(idx, d)| {
                    let (dx, dy) = offset;
                    ((i as i32 + idx as i32 * dx, j as i32 + idx as i32 * dy), d)
                })
                .all(|(pos, c)| {
                    ss.is_valid(pos.0, pos.1) && ss.grid[pos.0 as usize][pos.1 as usize] == *c
                })
        })
        .count();
    return count as i32;
}

fn part2(ss: &PuzzleInput) -> i32 {
    // Basically just iterate through and check for the 4 possible permutations.
    let count = (1..ss.height - 1)
        .flat_map(|i| (1..ss.width - 1).map(move |j| (i, j)))
        .filter(|&(i, j)| ss.grid[i][j] == 'A')
        .filter(|&(i, j)| {
            let cond1 = ss.grid[i - 1][j - 1] == 'M' && ss.grid[i + 1][j + 1] == 'S';
            let cond2 = ss.grid[i - 1][j - 1] == 'S' && ss.grid[i + 1][j + 1] == 'M';
            cond1 || cond2
        })
        .filter(|&(i, j)| {
            let cond1 = ss.grid[i - 1][j + 1] == 'M' && ss.grid[i + 1][j - 1] == 'S';
            let cond2 = ss.grid[i - 1][j + 1] == 'S' && ss.grid[i + 1][j - 1] == 'M';
            cond1 || cond2
        })
        .count();
    return count as i32;
}

fn main() {
    let s: String = fs::read_to_string("./input/04.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-04.txt").unwrap();
    let pi = parse(&s);
    println!("{}", part1(&pi));
    println!("{}", part2(&pi));
}
