use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs, vec,
};

#[derive(Debug)]
struct PuzzleInput {
    map: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

const DEBUG: bool = false;

fn parse(s: String) -> PuzzleInput {
    let map: Vec<Vec<i32>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let width = map[0].len();
    let height = map.len();
    return PuzzleInput { map, width, height };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    i: isize,
    j: isize,
    dir: Direction,
    count: i8,
    history: HashSet<(usize, usize)>,
    score: i32,
}

impl Ord for State {
    // Get a min heap
    fn cmp(&self, other: &Self) -> Ordering {
        return other.score.cmp(&self.score);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl State {
    fn _display_grid(&self, pi: &PuzzleInput) {
        for i in 0..pi.height {
            for j in 0..pi.width {
                if self.history.contains(&(i, j)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn part1(pi: &PuzzleInput, initial: Vec<State>) -> i32 {
    // This took me a long time. I clearly did not remember my lesson from 2 years ago where I also struggled with dijkstra.
    // I tried adding a cost table, but that pruned the correct branches.
    // I then tried to add a buffer to the cost table to avoid pruning too early, but that had a huge search space as well.
    // The crux here is to prune by looking at the node, not the cost.

    let mut queue = BinaryHeap::new();
    let mut lowest = 9999999;
    let mut lowest_state = initial[0].clone();
    let mut seen = HashSet::new();

    for s in initial {
        queue.push(s);
    }
    while queue.is_empty() == false {
        let mut s = queue.pop().unwrap();
        let key = (s.i, s.j, s.dir, s.count);
        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);

        if s.score >= lowest {
            continue;
        }

        let i = s.i;
        let j = s.j;
        if i < 0 || j < 0 || i as usize >= pi.height || j as usize >= pi.width {
            continue;
        }

        let ii = i as usize;
        let jj = j as usize;

        if s.history.contains(&(ii, jj)) {
            continue;
        }
        s.history.insert((ii, jj));

        // Reached end state.
        if ii == pi.height - 1 && jj == pi.width - 1 {
            if lowest > s.score {
                lowest_state = s.clone();
                lowest = s.score;
            }
            continue;
        }

        let dir = s.dir;

        // Try go up
        if dir != Direction::Down && i > 0 {
            let mut count = 1;
            if dir == Direction::Up {
                count += s.count;
            }
            if count != 4 {
                let score = s.score + pi.map[ii - 1][jj] as i32;
                queue.push(State {
                    i: i - 1,
                    j,
                    dir: Direction::Up,
                    count,
                    history: s.history.clone(),
                    score,
                });
            }
        }

        // Try go left
        if dir != Direction::Right && j > 0 {
            let mut count = 1;
            if dir == Direction::Left {
                count += s.count;
            }
            if count != 4 {
                let score = s.score + pi.map[ii][jj - 1] as i32;
                queue.push(State {
                    i,
                    j: j - 1,
                    dir: Direction::Left,
                    count,
                    history: s.history.clone(),
                    score,
                });
            }
        }

        // Try go right
        if dir != Direction::Left && jj + 1 < pi.width {
            let mut count = 1;
            if dir == Direction::Right {
                count += s.count;
            }
            if count != 4 {
                let score = s.score + pi.map[ii][jj + 1] as i32;
                queue.push(State {
                    i,
                    j: j + 1,
                    dir: Direction::Right,
                    count,
                    history: s.history.clone(),
                    score,
                });
            }
        }

        // Try go down
        if dir != Direction::Up && ii + 1 < pi.height {
            let mut count = 1;
            if dir == Direction::Down {
                count += s.count;
            }
            if count != 4 {
                let score = s.score + pi.map[ii + 1][jj] as i32;

                queue.push(State {
                    i: i + 1,
                    j,
                    dir: Direction::Down,
                    count,
                    history: s.history.clone(),
                    score,
                });
            }
        }
    }
    if DEBUG {
        lowest_state._display_grid(pi);
    }
    return lowest_state.score;
}

fn part2(pi: &PuzzleInput, initial: Vec<State>) -> i32 {
    // A lot of repetition here because of the way I hard coded the map.
    // There's probably a much nicer way to keep the code DRY.
    let mut queue = BinaryHeap::new();
    let mut lowest = 9999999;
    let mut lowest_state = initial[0].clone();
    let mut seen = HashSet::new();
    for s in initial {
        queue.push(s);
    }
    while queue.is_empty() == false {
        let mut s = queue.pop().unwrap();
        let key = (s.i, s.j, s.dir, s.count);
        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);

        if s.score >= lowest {
            continue;
        }

        let i = s.i;
        let j = s.j;
        if i < 0 || j < 0 || i as usize >= pi.height || j as usize >= pi.width {
            continue;
        }

        let ii = i as usize;
        let jj = j as usize;

        if s.history.contains(&(ii, jj)) {
            continue;
        }
        s.history.insert((ii, jj));

        // Reached end state.
        if ii == pi.height - 1 && jj == pi.width - 1 {
            // Need this otherwise sample input breaks
            if s.count < 4 || s.count > 10 {
                continue;
            }
            if lowest > s.score {
                lowest_state = s.clone();
                lowest = s.score;
            }
            continue;
        }

        let dir = s.dir;

        if s.count < 4 {
            match dir {
                Direction::Up => {
                    if i > 0 {
                        let score = s.score + pi.map[ii - 1][jj] as i32;
                        queue.push(State {
                            i: i - 1,
                            j,
                            dir: s.dir,
                            count: s.count + 1,
                            history: s.history.clone(),
                            score,
                        });
                    }
                }
                Direction::Left => {
                    if j > 0 {
                        let score = s.score + pi.map[ii][jj - 1] as i32;
                        queue.push(State {
                            i,
                            j: j - 1,
                            history: s.history.clone(),
                            dir: s.dir,
                            count: s.count + 1,
                            score,
                        });
                    }
                }
                Direction::Right => {
                    if jj + 1 < pi.width {
                        let score = s.score + pi.map[ii][jj + 1] as i32;
                        queue.push(State {
                            i,
                            j: j + 1,
                            dir: s.dir,
                            history: s.history.clone(),
                            count: s.count + 1,
                            score,
                        });
                    }
                }
                Direction::Down => {
                    if ii + 1 < pi.height {
                        let score = s.score + pi.map[ii + 1][jj] as i32;
                        queue.push(State {
                            i: i + 1,
                            j,
                            dir: s.dir,
                            history: s.history.clone(),
                            count: s.count + 1,
                            score,
                        });
                    }
                }
            }
            continue;
        }

        // Try go up
        if i > 0 {
            let mut count = 1;
            let score = s.score + pi.map[ii - 1][jj] as i32;
            if dir == Direction::Up {
                count += s.count;
            }
            if count != 11 {
                queue.push(State {
                    i: i - 1,
                    j,
                    dir: Direction::Up,
                    history: s.history.clone(),
                    count,
                    score,
                });
            }
        }

        // Try go left
        if j > 0 {
            let mut count = 1;
            let score = s.score + pi.map[ii][jj - 1] as i32;
            if dir == Direction::Left {
                count += s.count;
            }
            if count != 11 {
                queue.push(State {
                    i,
                    j: j - 1,
                    dir: Direction::Left,
                    history: s.history.clone(),
                    count,
                    score,
                });
            }
        }

        // Try go right
        if dir != Direction::Left && jj + 1 < pi.width {
            let mut count = 1;
            let score = s.score + pi.map[ii][jj + 1] as i32;
            if dir == Direction::Right {
                count += s.count;
            }
            if count != 11 {
                queue.push(State {
                    i,
                    j: j + 1,
                    dir: Direction::Right,
                    history: s.history.clone(),
                    count,
                    score,
                });
            }
        }

        // Try go down
        if dir != Direction::Up && ii + 1 < pi.height {
            let mut count = 1;
            if dir == Direction::Down {
                count += s.count;
            }
            if count != 11 {
                let score = s.score + pi.map[ii + 1][jj] as i32;
                queue.push(State {
                    i: i + 1,
                    j,
                    dir: Direction::Down,
                    history: s.history.clone(),
                    count,
                    score,
                });
            }
        }
    }

    if DEBUG {
        lowest_state._display_grid(pi);
    }
    return lowest_state.score;
}

fn main() {
    let s: String = fs::read_to_string("./src/input17.txt").unwrap();
    let pi = parse(s.trim().to_string());
    let ans: i32 = part1(
        &pi,
        vec![State {
            i: 0,
            j: 0,
            dir: Direction::Right,
            count: 0,
            history: HashSet::new(),
            score: 0,
        }],
    );
    println!("{}", ans);
    let ans: i32 = part2(
        &pi,
        vec![
            State {
                i: 0,
                j: 1,
                dir: Direction::Right,
                count: 1,
                history: HashSet::new(),
                score: pi.map[0][1] as i32,
            },
            State {
                i: 1,
                j: 0,
                dir: Direction::Down,
                count: 1,
                history: HashSet::new(),
                score: pi.map[1][0] as i32,
            },
        ],
    );
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1_sample_input() {
        // Actually better to go down first, even though it's not optimal with a cost grid.
        // The algo needs to not prune DR too early.
        let pi = parse(SAMPLE_INPUT.trim().to_string());
        let ans = part1(
            &pi,
            vec![State {
                i: 0,
                j: 0,
                dir: Direction::Right,
                count: 0,
                history: HashSet::new(),
                score: 0,
            }],
        );
        assert!(ans == 102, "got {}", ans);
    }

    #[test]
    fn test_part1_counter_example_for_vanilla_dijkstra() {
        // Actually better to go down first, even though it's not optimal with a cost grid.
        // The algo needs to not prune DR too early.
        let input = "
041119
611911
";
        let pi = parse(input.trim().to_string());
        let ans = part1(
            &pi,
            vec![State {
                i: 0,
                j: 0,
                dir: Direction::Right,
                count: 0,
                history: HashSet::new(),
                score: 0,
            }],
        );
        println!("{}", ans);
        assert!(ans == 11);
    }
    #[test]
    fn test_part1_larger_example() {
        let input = "
015999
991999
991999
991999
991111
";
        let pi = parse(input.trim().to_string());
        let ans = part1(
            &pi,
            vec![
                State {
                    i: 0,
                    j: 1,
                    dir: Direction::Right,
                    count: 1,
                    history: HashSet::new(),
                    score: pi.map[0][1] as i32,
                },
                State {
                    i: 1,
                    j: 0,
                    dir: Direction::Down,
                    count: 1,
                    history: HashSet::new(),
                    score: pi.map[1][0] as i32,
                },
            ],
        );
        println!("{}", ans);
        assert!(ans == 17);
    }

    #[test]
    fn test_part2_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim().to_string());
        let ans = part2(
            &pi,
            vec![
                State {
                    i: 0,
                    j: 1,
                    dir: Direction::Right,
                    history: HashSet::new(),
                    count: 1,
                    score: pi.map[0][1] as i32,
                },
                State {
                    i: 1,
                    j: 0,
                    dir: Direction::Down,
                    history: HashSet::new(),
                    count: 1,
                    score: pi.map[1][0] as i32,
                },
            ],
        );
        assert!(ans == 94, "got {}", ans);
    }
    #[test]
    fn test_part2_sample_input_2() {
        let input = "
111111111111
999999999991
999999999991
999999999991
999999999991
";
        let pi = parse(input.trim().to_string());
        let ans = part2(
            &pi,
            vec![
                State {
                    i: 0,
                    j: 1,
                    dir: Direction::Right,
                    history: HashSet::new(),
                    count: 1,
                    score: pi.map[0][1] as i32,
                },
                State {
                    i: 1,
                    j: 0,
                    dir: Direction::Down,
                    history: HashSet::new(),
                    count: 1,
                    score: pi.map[1][0] as i32,
                },
            ],
        );
        println!("{}", ans);
        assert!(ans == 71);
    }
}
