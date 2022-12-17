use std::collections::HashMap;
use std::iter::Cycle;
use std::slice::Iter;
use std::{fs, vec};

fn main() {
    let s: String = fs::read_to_string("./src/input17.txt").unwrap();
    // let s: String = String::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
    let jet: Vec<char> = s.trim_end().chars().collect();

    let part1 = solve1(&jet, 2022);
    dbg!(part1);
    let part2 = solve2(&jet, 1_000_000_000_000);
    dbg!(part2);
}

fn solve2(jet: &[char], cycle: i64) -> i64 {
    let (_, seen) = helper(jet, 7000); // Choose a number high enough to get a periodic pattern.
    // dbg!(&seen);
    let mut cycle_length = 0;
    let mut cycle_heigh_incr = 0;
    seen.values().for_each(|evidences| {
        if evidences.len() >= 4 {
            evidences.windows(2).for_each(|w| {
                if cycle_length == 0 {
                    cycle_length = w[1].0 - w[0].0;
                    cycle_heigh_incr = w[1].1 - w[0].1;
                } else {
                    // Assert the pattern is consistent
                    assert_eq!(cycle_length, w[1].0 - w[0].0);
                    assert_eq!(cycle_heigh_incr, w[1].1 - w[0].1);
                }
            });
        }
    });

    if cycle_length == 0 {
        panic!("No periodic pattern found, try increasing length");
    }

    // Make sure to subtract 1 cycle so that we are sure we're in the cycle pattern.
    let largest_cycle = cycle / cycle_length as i64 - 1;
    let remainder = cycle - largest_cycle*cycle_length as i64;
    dbg!(largest_cycle, remainder);
    let (leftover_height, _) = helper(jet, remainder as i32);
    return largest_cycle * cycle_heigh_incr as i64 + leftover_height as i64 ;
}

fn solve1(jet: &[char], num_rocks: i32) -> i32 {
    let (ans, _) = helper(jet, num_rocks);
    return ans;
}

// First return is height after simulating num_rocks,
//
// Second return is a map from (key -> (num rocks, height)). This is for part2.
// Key here is the modulo of the number of rocks and the length of the jet stream.
// The suspicion here is that the height increase will be periodic.
// not sure how to prove this though.
fn helper(jet: &[char], num_rocks: i32) -> (i32, HashMap<(i32, i32), Vec<(i32, i32)>>) {
    let patterns: Vec<Blocks2> = patterns2();
    let mut grid: Vec<Vec<bool>> = vec![vec![true; 7]; 1];
    // Convenient to do intersection.
    // Reminder: up is increasing y direction.
    for _ in 0..num_rocks {
        // On average we need 3 more rows, but 2 seems to be sufficient.
        grid.extend(vec![vec![false; 7]; 2]);
    }
    let mut jet_stream: Cycle<Iter<char>> = jet.iter().cycle();
    let mut height: i32 = 1;
    let mut num_chars = 0;

    // For part 2
    let mut seen: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    for i in 0..num_rocks {
        let mut rock: Blocks2 = patterns[(i % 5) as usize].clone();
        rock.move_up(height as isize + 3);
        num_chars += simulate(&mut grid, &mut jet_stream, rock);
        height = calc_height(&grid);

        // For part 2, just store 0 mod 5 is sufficient.
        if i % 5 == 0 {
            let e = seen
                .entry((i % 5, num_chars % jet.len() as i32))
                .or_insert(vec![]);
            e.push((i, height - 1));
        }
    }

    return (height - 1, seen);
}

fn calc_height(grid: &[Vec<bool>]) -> i32 {
    let mut height = 0;
    for row in grid.iter() {
        if row.iter().all(|x| !*x) {
            return height;
        }
        height += 1;
    }
    return height;
}

fn simulate(grid: &mut [Vec<bool>], jet_stream: &mut Cycle<Iter<char>>, rock: Blocks2) -> i32 {
    let candidate = &mut rock.clone();
    let mut num_chars = 0;
    loop {
        let dir = jet_stream.next().unwrap();
        num_chars += 1;
        match dir {
            '>' => {
                if candidate.can_move_right() {
                    candidate.move_right()
                }
            }
            '<' => {
                if candidate.can_move_left() {
                    candidate.move_left()
                }
            }
            _ => panic!(),
        }
        if intersect2(&candidate, grid) {
            match dir {
                '<' => {
                    if candidate.can_move_right() {
                        (candidate).move_right()
                    }
                }
                '>' => {
                    if candidate.can_move_left() {
                        (candidate).move_left()
                    }
                }
                _ => panic!(),
            }
        }

        (candidate).move_down();
        candidate.coord.iter().for_each(|(_, y)| assert!(*y >= 0));
        if !intersect2(&candidate, grid) {
            continue;
        } else {
            candidate.move_up(1);
            // print_map(grid, &candidate);
            candidate
                .coord
                .iter()
                .for_each(|(x, y)| grid[*y as usize][*x as usize] = true);
            return num_chars;
        };
    }
}

fn intersect2(rock: &Blocks2, grid: &[Vec<bool>]) -> bool {
    for (x, y) in rock.coord.iter() {
        if grid[*y as usize][*x as usize] {
            return true;
        }
    }
    return false;
}

#[derive(Debug, Clone)]

struct Blocks2 {
    coord: Vec<(isize, isize)>,
}

impl Blocks2 {
    fn move_left(&mut self) -> () {
        self.coord.iter_mut().for_each(|(x, _)| *x -= 1);
    }
    fn move_right(&mut self) -> () {
        self.coord.iter_mut().for_each(|(x, _)| *x += 1);
    }
    fn move_down(&mut self) -> () {
        self.coord.iter_mut().for_each(|(_, y)| *y -= 1);
    }
    fn move_up(&mut self, val: isize) -> () {
        self.coord.iter_mut().for_each(|(_, y)| *y += val);
    }
    fn can_move_left(&mut self) -> bool {
        self.coord.iter().all(|(x, _)| x - 1 >= 0)
    }
    fn can_move_right(&mut self) -> bool {
        self.coord.iter().all(|(x, _)| x + 1 < 7)
    }
}

fn patterns2() -> Vec<Blocks2> {
    let mut res: Vec<Blocks2> = vec![];
    res.push(Blocks2 {
        coord: vec![(2, 0), (3, 0), (4, 0), (5, 0)],
    });
    res.push(Blocks2 {
        coord: vec![(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)],
    });
    res.push(Blocks2 {
        coord: vec![(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)],
    });
    res.push(Blocks2 {
        coord: vec![(2, 0), (2, 1), (2, 2), (2, 3)],
    });
    res.push(Blocks2 {
        coord: vec![(2, 0), (3, 0), (2, 1), (3, 1)],
    });
    return res;
}
