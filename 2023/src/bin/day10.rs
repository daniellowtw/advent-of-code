use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
struct PuzzleInput {
    grid: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
    start: (usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Empty,
    Start,
}

fn parse_line(s: &str) -> Vec<Tile> {
    let directions: HashMap<char, Tile> = {
        let mut m = HashMap::new();
        m.insert('|', Tile::NS);
        m.insert('-', Tile::EW);
        m.insert('L', Tile::NE);
        m.insert('F', Tile::SE);
        m.insert('J', Tile::NW);
        m.insert('7', Tile::SW);
        m.insert('.', Tile::Empty);
        m.insert('S', Tile::Start);
        m
    };
    let row: Vec<Tile> = s
        .trim()
        .chars()
        .map(|x| directions.get(&x).unwrap().clone())
        .collect();
    return row;
}

fn parse(s: String) -> PuzzleInput {
    let grid: Vec<Vec<Tile>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| parse_line(x))
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Tile::Start {
                return PuzzleInput {
                    grid,
                    height,
                    width,
                    start: (i, j),
                };
            }
        }
    }
    panic!("No start found");
}

fn neighbours(i: usize, j: usize, g: &PuzzleInput) -> Vec<(usize, usize)> {
    let mut res = vec![];
    match g.grid[i][j] {
        Tile::NS => {
            if i > 0 {
                res.push((i - 1, j));
            }
            if i < g.height - 1 {
                res.push((i + 1, j));
            }
        }
        Tile::EW => {
            if j > 0 {
                res.push((i, j - 1));
            }
            if j < g.width - 1 {
                res.push((i, j + 1));
            }
        }
        Tile::NE => {
            if i > 0 {
                res.push((i - 1, j));
            }
            if j < g.width - 1 {
                res.push((i, j + 1));
            }
        }
        Tile::NW => {
            if i > 0 {
                res.push((i - 1, j));
            }
            if j > 0 {
                res.push((i, j - 1));
            }
        }
        Tile::SE => {
            if i < g.height - 1 {
                res.push((i + 1, j));
            }
            if j < g.width - 1 {
                res.push((i, j + 1));
            }
        }
        Tile::SW => {
            if i < g.height - 1 {
                res.push((i + 1, j));
            }
            if j > 0 {
                res.push((i, j - 1));
            }
        }
        Tile::Start => {
            // Have to consider all directions. This was tricky because I did not originally consider if the pipe actually was connected to S
            if i > 0 {
                // North
                let t = g.grid[i - 1][j];
                if t == Tile::NS || t == Tile::SE || t == Tile::SW {
                    res.push((i - 1, j));
                }
            }
            if i < g.height - 1 {
                // South
                let t = g.grid[i + 1][j];
                if t == Tile::NS || t == Tile::NE || t == Tile::NW {
                    res.push((i + 1, j));
                }
            }
            if j > 0 {
                // West
                let t = g.grid[i][j - 1];
                if t == Tile::EW || t == Tile::NE || t == Tile::SE {
                    res.push((i, j - 1));
                }
            }
            if j < g.width - 1 {
                // East
                let t = g.grid[i][j + 1];
                if t == Tile::EW || t == Tile::NW || t == Tile::SW {
                    res.push((i, j + 1));
                }
            }
        }
        _ => {}
    }
    return res
        .into_iter()
        .filter(|n| g.grid[n.0][n.1] != Tile::Empty)
        .collect();
}

fn filter_to_main_loop(pi: &PuzzleInput) -> HashMap<(usize, usize), i32> {
    // Stratgy:
    // Do a BFS, and build a distance map.
    // Then find the max value in the distance map.
    let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut dist_map: HashMap<(usize, usize), i32> = HashMap::new();
    queue.push_back((pi.start, 0));
    dist_map.insert(pi.start, 0);

    loop {
        if queue.is_empty() {
            break;
        }

        let ((i, j), old_dist) = queue.pop_front().unwrap();
        seen.insert((i, j));

        let dist = old_dist + 1;
        for n in neighbours(i, j, pi) {
            if seen.contains(&n) {
                continue;
            }
            queue.push_back((n, dist));
            dist_map.insert(n, dist);
        }
    }

    return dist_map;
}
fn part1(pi: &PuzzleInput) -> i32 {
    let dist_map = filter_to_main_loop(pi);
    let mut max_dist: i32 = 0;
    // let mut furthest_point = (0, 0);
    for (_, v) in dist_map {
        if v > max_dist {
            max_dist = v;
            // furthest_point = k;
        }
    }
    // println!("{:?}", furthest_point);
    return max_dist;
}

fn part2(pi: &PuzzleInput) -> i32 {
    // Strategy:
    // Find the main loop, then go through the grid, and for points that's not in the main loop
    // determine if it's inside or outside the loop.

    // I had several bad approaches here. My first one was to do a flood fill from the outside.
    // Together with printing the map, I was hoping that I can easily offset the results from the visuals.
    // This was not great when I realized there were 200+ pools to consider.

    let main_loop = filter_to_main_loop(pi);
    let main_loop: HashSet<&(usize, usize)> = main_loop.keys().collect();
    let mut count = 0;

    let mut new_map = pi.grid.clone(); //
    for i in 0..pi.height {
        for j in 0..pi.width {
            if main_loop.contains(&(i, j)) {
                continue;
            } else {
                new_map[i][j] = Tile::Empty;
            }
        }
    }

    // println!();
    for i in 0..pi.height {
        for j in 0..pi.width {
            if main_loop.contains(&(i, j)) {
                // print!(".");
                continue;
            } else {
                // I could probably inline the is_inside function here, so for each line, it's a single pass.
                if is_inside(i, j, &new_map) {
                    // print!("I");
                    count += 1;
                } else {
                    // print!("O");
                }
            }
        }
        // println!();
    }

    return count;
}

fn is_inside(i: usize, j: usize, new_map: &Vec<Vec<Tile>>) -> bool {
    // Count number of times we cross a vertical line, we start with outside, each vertical line crossed flips the result.
    // Very tricky here. If it's a u or n shape, they don't flip the result.
    let mut res = false;
    let mut x = 0;
    loop {
        if x >= j {
            break;
        }
        // At the end of this match, x should point to the end of the vertical line
        match new_map[i][x] {
            Tile::NS => {
                res = !res;
            }
            Tile::SE => loop {
                x += 1;
                if new_map[i][x] == Tile::NW {
                    res = !res;
                    break;
                } else if new_map[i][x] == Tile::SW {
                    // n
                    break;
                }
            },
            Tile::NE => loop {
                x += 1;
                if new_map[i][x] == Tile::SW {
                    res = !res;
                    break;
                } else if new_map[i][x] == Tile::NW {
                    // u
                    break;
                }
            },
            _ => {}
        }
        x += 1;
    }
    return res;
}

fn main() {
    let s: String = fs::read_to_string("./src/input10.txt").unwrap();
    let inputs = parse(s);
    // println!("{:?}", &inputs);
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
    // let ans2: i32 = inputs.lines.iter().map(|x| part2(x)).sum();
    // println!("{}", ans2);
    // let line = vec![parse_line("FJ.")];
    // for i in 0..3 {
    //     let a = is_inside(0, i, &line);
    //     println!("{:?} -> {}", line[0][i], a);
    // }
}
