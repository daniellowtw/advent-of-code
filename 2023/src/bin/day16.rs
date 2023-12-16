use std::{collections::VecDeque, fs, vec};

#[derive(Debug)]
struct PuzzleInput {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

fn parse(s: String) -> PuzzleInput {
    let map: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().chars().collect())
        .collect();
    let width = map[0].len();
    let height = map.len();
    return PuzzleInput { map, width, height };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn part1(pi: &PuzzleInput, initial: (isize, isize, Direction)) -> i32 {
    // I had a bug where I mixed up the i and the j and had to redo the mapping.
    let mut visited: Vec<Vec<bool>> = vec![vec![false; pi.width]; pi.height];
    let mut visited_with_direction: Vec<Vec<Vec<Direction>>> =
        vec![vec![vec![]; pi.width]; pi.height];
    let mut queue: VecDeque<(isize, isize, Direction)> = VecDeque::new();
    queue.push_back(initial);
    while queue.is_empty() == false {
        let (i, j, dir) = queue.pop_front().unwrap();
        if i < 0 || j < 0 || i as usize >= pi.height || j as usize >= pi.width {
            continue;
        }

        let ii = i as usize;
        let jj = j as usize;
        if visited_with_direction[ii][jj].contains(&dir) {
            continue;
        }
        visited_with_direction[ii][jj].push(dir);
        visited[ii][jj] = true;

        // I do wonder if there's a more succinct way to write this.
        // I think the 5x4 blocks have to be expressed some how.
        match (pi.map[ii][jj], dir) {
            ('.', Direction::Right) => queue.push_back((i, j + 1, Direction::Right)),
            ('-', Direction::Right) => queue.push_back((i, j + 1, Direction::Right)),
            ('|', Direction::Right) => {
                queue.push_back((i + 1, j, Direction::Down));
                queue.push_back((i - 1, j, Direction::Up));
            }
            ('\\', Direction::Right) => {
                queue.push_back((i + 1, j, Direction::Down));
            }
            ('/', Direction::Right) => {
                queue.push_back((i - 1, j, Direction::Up));
            }

            ('.', Direction::Left) => queue.push_back((i, j - 1, Direction::Left)),
            ('-', Direction::Left) => queue.push_back((i, j - 1, Direction::Left)),
            ('|', Direction::Left) => {
                queue.push_back((i + 1, j, Direction::Down));
                queue.push_back((i - 1, j, Direction::Up));
            }
            ('\\', Direction::Left) => {
                queue.push_back((i - 1, j, Direction::Up));
            }
            ('/', Direction::Left) => {
                queue.push_back((i + 1, j, Direction::Down));
            }

            ('.', Direction::Up) => queue.push_back((i - 1, j, Direction::Up)),
            ('|', Direction::Up) => queue.push_back((i - 1, j, Direction::Up)),
            ('-', Direction::Up) => {
                queue.push_back((i, j + 1, Direction::Right));
                queue.push_back((i, j - 1, Direction::Left));
            }
            ('\\', Direction::Up) => {
                queue.push_back((i, j - 1, Direction::Left));
            }
            ('/', Direction::Up) => {
                queue.push_back((i, j + 1, Direction::Right));
            }

            ('.', Direction::Down) => queue.push_back((i + 1, j, Direction::Down)),
            ('|', Direction::Down) => queue.push_back((i + 1, j, Direction::Down)),
            ('-', Direction::Down) => {
                queue.push_back((i, j + 1, Direction::Right));
                queue.push_back((i, j - 1, Direction::Left));
            }
            ('\\', Direction::Down) => {
                queue.push_back((i, j + 1, Direction::Right));
            }
            ('/', Direction::Down) => {
                queue.push_back((i, j - 1, Direction::Left));
            }
            _ => {
                panic!()
            }
        }
        continue;
    }

    // for i in 0..pi.height {
    //     for j in 0..pi.width {
    //         print!("{}", if visited[i][j] { '#' } else { '.' });
    //     }
    //     println!();
    // }

    return visited
        .iter()
        .map(|x| x.iter().filter(|y| *y == &true).count() as i32)
        .sum();
}

fn part2(pi: &PuzzleInput) -> i32 {
    // There was a loop which revealed a bug where I should be tracking the set of seen directions instead of tracking just one direction per grid.
    // I also had to add some debug lines to break out of the loop when the count reached the size of the grid and then printing the map.
    let mut max_score = 0;
    for i in 0..pi.height {
        let score = part1(pi, (i as isize, 0, Direction::Right));
        if score > max_score {
            max_score = score;
        }
        let score = part1(pi, (i as isize, pi.width as isize - 1, Direction::Left));
        if score > max_score {
            max_score = score;
        }
    }
    for j in 0..pi.width {
        let score = part1(pi, (0, j as isize, Direction::Down));
        if score > max_score {
            max_score = score;
        }
        let score = part1(pi, (pi.height as isize - 1, j as isize, Direction::Up));
        if score > max_score {
            max_score = score;
        }
    }
    return max_score;
}
fn main() {
    let s: String = fs::read_to_string("./src/input16.txt").unwrap();
    let inputs = parse(s.trim().to_string());
    let ans: i32 = part1(&inputs, (0, 0, Direction::Right));
    println!("{}", ans);
    let ans: i32 = part2(&inputs);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_part1() {}
}
