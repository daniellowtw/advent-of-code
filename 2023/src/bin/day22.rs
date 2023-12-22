#![allow(warnings)]
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs,
};

type Pos = (usize, usize, usize);

#[derive(Debug, Clone)]
struct PuzzleInput {
    pub grid: Vec<(Pos, Pos)>,
}

impl PuzzleInput {}

fn parse(s: &str) -> PuzzleInput {
    let mut grid: Vec<(Pos, Pos)> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut parts: Vec<_> = x
                .split("~")
                .map(|y| {
                    let parts = y
                        .split(",")
                        .map(|z| z.trim().parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    (parts[0], parts[1], parts[2])
                })
                .collect();
            parts.sort_by(|a, b| a.2.cmp(&b.2));
            return (parts[0], parts[1]);
        })
        .collect();
    // Sort by z value
    grid.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));

    // Check assumption that we only have bricks of a single direction.
    // Don't think it affects the algorithm, but it simplifies boudnary checks (iterate in 1 direction instead of 2).
    for i in &grid {
        let mut num_diff = 0;
        if i.0 .0 != i.1 .0 {
            num_diff += 1;
        }
        if i.0 .1 != i.1 .1 {
            num_diff += 1;
        }
        if i.0 .2 != i.1 .2 {
            num_diff += 1;
        }
        assert!(num_diff <= 1);
    }
    return PuzzleInput { grid };
}

fn simulate(pi: &PuzzleInput) -> HashMap<usize, Vec<usize>> {
    // Simulate the falling bricks, and return whats supported by what.
    let max_pos = pi.grid.iter().fold((0, 0, 0), |acc, x| {
        let new_x = max(acc.0, max(x.0 .0, x.1 .0));
        let new_y = max(acc.1, max(x.0 .1, x.1 .1));
        let new_z = max(acc.2, max(x.0 .2, x.1 .2));
        (new_x, new_y, new_z)
    });

    // Keep track of what bricks are supported by what. key is current brick, value is the bricks supporting it.
    let mut supported: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut space: Vec<Vec<Vec<usize>>> =
        vec![vec![vec![0; max_pos.2 + 1]; max_pos.1 + 1]; max_pos.0 + 1];
    for (id, brick) in pi.grid.iter().enumerate() {
        if brick.0 .1 != brick.1 .1 {
            // Same x, z
            let mut min_z = 0;
            for i in (0..=brick.0 .2).rev() {
                if (brick.0 .1..=brick.1 .1).all(|y| space[brick.0 .0][y][i] == 0) {
                    min_z = i;
                } else {
                    let mut supported_types: HashSet<usize> = (brick.0 .1..=brick.1 .1)
                        .map(|y| space[brick.0 .0][y][i])
                        .collect();
                    supported_types.remove(&0);
                    supported.insert(id + 1, supported_types.into_iter().collect());
                    break;
                }
            }
            // Place brick
            for i in brick.0 .1..=brick.1 .1 {
                space[brick.0 .0][i][min_z] = id + 1;
            }
        } else if brick.0 .0 != brick.1 .0 {
            // Same y, z
            let mut min_z = 0;
            for i in (0..=brick.0 .2).rev() {
                if (brick.0 .0..=brick.1 .0).all(|x| space[x][brick.0 .1][i] == 0) {
                    min_z = i;
                } else {
                    let mut supported_types: HashSet<usize> = (brick.0 .0..=brick.1 .0)
                        .map(|x| space[x][brick.0 .1][i])
                        .collect();
                    supported_types.remove(&0);
                    supported.insert(id + 1, supported_types.into_iter().collect());
                    break;
                }
            }
            // Place brick
            for i in brick.0 .0..=brick.1 .0 {
                space[i][brick.0 .1][min_z] = id + 1;
            }
        } else {
            // vertical, same x, y or Single brick
            let mut min_z = 0;
            for i in (0..=brick.0 .2).rev() {
                if space[brick.0 .0][brick.0 .1][i] == 0 {
                    min_z = i;
                } else {
                    supported.insert(id + 1, vec![space[brick.0 .0][brick.0 .1][i]]);
                    break;
                }
            }
            // Place brick
            for i in 0..(brick.1 .2 - brick.0 .2 + 1) {
                space[brick.0 .0][brick.0 .1][i + min_z] = id + 1;
            }
        }
    }
    return supported;
}

fn _display_grid(space: &Vec<Vec<Vec<usize>>>) {
    for z in (0..space[0][0].len()).rev() {
        for x in 0..space.len() {
            let mut seen = HashSet::new();
            for y in 0..space[0].len() {
                if space[x][y][z] != 0 {
                    seen.insert(space[x][y][z]);
                }
            }

            if seen.len() == 0 {
                print!(" ");
            } else if seen.len() == 1 {
                print!("{}", seen.iter().next().unwrap());
            } else {
                print!("?");
            }
        }
        for y in 0..space[0].len() {
            let mut seen = HashSet::new();
            for x in 0..space.len() {
                if space[x][y][z] != 0 {
                    seen.insert(space[x][y][z]);
                }
            }

            if seen.len() == 0 {
                print!(" ");
            } else if seen.len() == 1 {
                print!("{}", seen.iter().next().unwrap());
            } else {
                print!("?");
            }
        }
        println!();
    }
    // Ask for input to see the bricks layed out layer by layer.
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn part1(pi: &PuzzleInput) -> i32 {
    let supported = simulate(pi);
    let mut ans = 0;
    for b in 1..=pi.grid.len() {
        // can remove only if not supporting anything that is not supported otherwise
        if supported.iter().all(|s| !(s.1.len() == 1 && s.1[0] == b)) {
            ans += 1;
        }
    }
    return ans;
}

fn part2(pi: &PuzzleInput) -> i32 {
    let supported = simulate(pi);
    let mut ans = 0;
    for b in 1..=pi.grid.len() {
        let mut state = supported.clone();
        let mut to_remove = vec![b];
        let mut affected = HashSet::new();
        while !to_remove.is_empty() {
            let c = to_remove.pop().unwrap();
            for (id, supported_by) in state.iter_mut() {
                if supported_by.contains(&c) {
                    supported_by.retain(|x| x != &c);
                    if supported_by.len() == 0 {
                        to_remove.push(*id);
                        affected.insert(*id);
                    }
                }
            }
        }
        ans += affected.len();
    }
    return ans as i32;
}

fn main() {
    let s: String = fs::read_to_string("./src/input22.txt").unwrap();
    let pi: PuzzleInput = parse(s.trim());
    let ans = part1(&pi);
    println!("{}", ans);
    let ans = part2(&pi);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn test_part1_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim());
        let ans = part1(&pi);
        println!("{}", ans);
    }

    #[test]
    fn test_part2_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim());
        let ans = part2(&pi);
        println!("{}", ans);
    }
}
