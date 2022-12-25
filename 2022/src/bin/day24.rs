use std::{cmp::Ordering};
use std::collections::{BinaryHeap, HashSet};
use std::fs;

fn parse(s: &str) -> Vec<Vec<HashSet<char>>> {
    s.trim_end()
        .split("\n")
        .map(|r| {
            r.chars()
                .map(|c| {
                    let mut res = HashSet::new();
                    match c {
                        '#' => res.insert('#'),
                        '.' => false,
                        o => res.insert(o),
                    };
                    res
                })
                .collect()
        })
        .collect()
}

fn main() {
    let s: String = fs::read_to_string("./src/input24.txt").unwrap();
    let map = parse(&s);

    let part1 = solve1(&map);
    dbg!(part1);
    let part2 = solve2(&map);
    dbg!(part2);
}
fn solve1(map: &Vec<Vec<HashSet<char>>>) -> i32 {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let m = (height - 2) * (width - 2);
    let mut maps = vec![map.clone()];
    for i in 1..m {
        maps.push(next_map(&maps[i as usize - 1]));
    }

    let x1 = bfs(&maps, 0, (1, 0), (width-2, height-2)).unwrap();
    x1
}

fn solve2(map: &Vec<Vec<HashSet<char>>>) -> i32 {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let m = (height - 2) * (width - 2);
    let mut maps = vec![map.clone()];
    for i in 1..m {
        maps.push(next_map(&maps[i as usize - 1]));
    }

    let x1 = bfs(&maps, 0, (1, 0), (width-2, height-2)).unwrap();
    let x2 = bfs(&maps, x1 % m, (width-2, height-1), (1, 1)).unwrap();
    let x3 = bfs(&maps, (x2 + x1) % m, (1, 0), (width-2, height-2)).unwrap();
    dbg!(x1, x2, x3);
    x1 + x2 + x3
}

type Grid = Vec<Vec<HashSet<char>>>;
type Pos = (i32, i32);

#[derive(Eq, PartialEq)]
struct Item {
    priority: i32,
    task: (i32, Pos),
}

// Manually implpement Ord so we have a min heap
impl<'a> Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

// PartialOrd is required by Ord
impl<'a> PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(maps: &Vec<Grid>, idx: i32, pos: (i32, i32), target: Pos) -> Option<i32> {
    // PS: Such an epic fail not using a set to begin with.
    // Using a vec here is too slow.
    let mut history = HashSet::new();


    // PS: actually did a* here instead because I went to use vec which was too slow
    // and I tried to optimize it with a*. But actually it makes little difference here.
    let mut queue = BinaryHeap::with_capacity(5);
    queue.push(Item {
        priority: manhatten(pos, target),
        task: (idx, pos),
    });
    // let mut queue = VecDeque::new();
    // queue.push_back((idx, pos));


    // PS: could use i32::MAX here, but my wrapping function requires target to not
    // be on the edge, so I manually add 1 to the steps when I reach bottom right.
    // Because of +1, I can't use i32::MAX here.
    let mut min_score = 1000;

    let m = maps.len() as i32;

    // let mut start_time = time::Instant::now();
    // let mut count = 0;
    while !queue.is_empty() {
        // let (steps, curr_pos) = queue.pop_back().unwrap();
        let (steps, curr_pos) = queue.pop().unwrap().task;
        history.insert((steps % m, curr_pos));
        // count += 1;
        // if count % 1000000 == 0 {
            // let elapsed = start_time.elapsed();
            // start_time = time::Instant::now();
            // println!("{}: {}. Queue: {}, m:{}, step:{}, {}", count, min_score, queue.len(), m, steps, elapsed.as_secs_f64());
        // }

        if steps +1 > min_score {
            continue;
        }
        if curr_pos == target {
            if steps +1 < min_score {
                min_score = steps +1 ;
            }
            continue;
        }

        let next_map = &maps[((steps+1) % m) as usize];
        let next_poss = possible_moves(&next_map, curr_pos);
        for next_pos in next_poss {
            if history.contains(&((steps + 1) % m, next_pos)) {
                continue;
            }
            if steps + 1 >= min_score {
                continue;
            }
            let p = manhatten(next_pos, target); 
            queue.push(Item {
                priority: p,
                task: (steps + 1, next_pos),
            });
            // queue.push_back((steps + 1, next_pos));
        }
    }
    return Some(min_score - idx);
}

fn manhatten(pos: Pos, target: Pos) -> i32 {
    let (x, y) = pos;
    let (x2, y2) = target;
    return (x - x2).abs() + (y - y2).abs();
}

fn next_map(map: &Vec<Vec<HashSet<char>>>) -> Vec<Vec<HashSet<char>>> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut next_map = vec![vec![HashSet::new(); width as usize]; height as usize];

    // Build new map
    for r in 0..height {
        for c in 0..width {
            map[r as usize][c as usize]
                .iter()
                .for_each(|char| match char {
                    '#' => {
                        next_map[r as usize][c as usize].insert('#');
                    }
                    '>' => {
                        let (r1, c1) = wrap(height, width, r, c + 1);
                        next_map[r1 as usize][c1 as usize].insert('>');
                    }
                    '<' => {
                        let (r1, c1) = wrap(height, width, r, c - 1);
                        next_map[r1 as usize][c1 as usize].insert('<');
                    }
                    '^' => {
                        let (r1, c1) = wrap(height, width, r - 1, c);
                        next_map[r1 as usize][c1 as usize].insert('^');
                    }
                    'v' => {
                        let (r1, c1) = wrap(height, width, r + 1, c);
                        next_map[r1 as usize][c1 as usize].insert('v');
                    }
                    _ => (),
                });
        }
    }
    next_map
}

fn possible_moves(
    map: &Vec<Vec<HashSet<char>>>,
    pos: (i32, i32),
) -> Vec<(i32, i32)> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut next_pos = vec![];
    for n in neighbours(pos, height, width) {
        if map[n.1 as usize][n.0 as usize].is_empty() {
            next_pos.push(n);
        }
    }

    next_pos
}

#[allow(dead_code)]
fn print_map(res: &[Vec<HashSet<char>>], pos: (i32, i32)) -> () {
    let (x, y) = pos;
    for (row, r) in res.iter().enumerate() {
        for (col, c) in r.iter().enumerate() {
            if c.is_empty() {
                if x == col as i32 && y == row as i32 {
                    print!("X");
                } else {
                    print!(".");
                }
            } else {
                if c.len() > 1 {
                    print!("{}", c.len());
                } else {
                    print!("{}", c.iter().next().unwrap());
                }
            }
        }
        println!();
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn wrap(height: i32, width: i32, r: i32, c: i32) -> (usize, usize) {
    let r1 = if r == 0 {
        height - 2
    } else if r == height - 1 {
        1
    } else {
        r
    };
    let c1 = if c == 0 {
        width - 2
    } else if c == width - 1 {
        1
    } else {
        c
    };
    (r1 as usize, c1 as usize)
}

fn neighbours(arg: (i32, i32), height: i32, width: i32) -> Vec<(i32, i32)> {
    let (x, y) = arg;
    let mut res = vec![(x, y)];
    if x > 1 {
        res.push((x - 1, y));
    }
    if x < width - 2 {
        res.push((x + 1, y));
    }
    if y > 1 {
        res.push((x, y - 1));
    }
    if y < height - 2 {
        res.push((x, y + 1));
    }
    res
}
