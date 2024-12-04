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
    let mut count = 0;
    let offsets = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    for i in 0..ss.height {
        for j in 0..ss.width {
            if ss.grid[i][j] == 'X' {
                for (i2, j2) in offsets.iter() {
                    if ss.is_valid(i as i32 + i2, j as i32 + j2) {
                        if ss.grid[(i as i32 + i2) as usize][(j as i32 + j2) as usize] == 'M' {
                            if ss.is_valid(i as i32 + i2 + i2, j as i32 + j2 + j2) {
                                if ss.grid[(i as i32 + i2 + i2) as usize]
                                    [(j as i32 + j2 + j2) as usize]
                                    == 'A'
                                {
                                    if ss.is_valid(i as i32 + i2 + i2 + i2, j as i32 + j2 + j2 + j2)
                                    {
                                        if ss.grid[(i as i32 + i2 + i2 + i2) as usize]
                                            [(j as i32 + j2 + j2 + j2) as usize]
                                            == 'S'
                                        {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return count;
}

fn part2(ss: &PuzzleInput) -> i32 {
    // Basically just iterate through and check for the 4 possible permutations.
    let mut count = 0;
    for i in 1..ss.height - 1 {
        for j in 1..ss.width - 1 {
            if ss.grid[i][j] != 'A' {
                continue;
            } else {
                if ss.grid[i - 1][j - 1] == 'M' && ss.grid[i + 1][j + 1] == 'S' {
                    if ss.grid[i - 1][j + 1] == 'M' && ss.grid[i + 1][j - 1] == 'S'
                        || ss.grid[i + 1][j - 1] == 'M' && ss.grid[i - 1][j + 1] == 'S'
                    {
                        count += 1;
                    }
                } else if ss.grid[i - 1][j - 1] == 'S' && ss.grid[i + 1][j + 1] == 'M' {
                    if ss.grid[i - 1][j + 1] == 'M' && ss.grid[i + 1][j - 1] == 'S'
                        || ss.grid[i + 1][j - 1] == 'M' && ss.grid[i - 1][j + 1] == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    return count;
}

fn main() {
    let s: String = fs::read_to_string("./input/04.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-04.txt").unwrap();
    let pi = parse(&s);
    println!("{}", part1(&pi));
    println!("{}", part2(&pi));
}
