use std::{collections::HashMap, fs, vec};

#[derive(Debug)]
struct PuzzleInput {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl PuzzleInput {
    fn spin(&mut self) {
        // Move north
        for j in 0..self.width {
            let mut free = 0;
            for i in 0..self.height {
                if self.map[i][j] == 'O' {
                    self.map[i][j] = '.';
                    self.map[free][j] = 'O';
                    free += 1;
                } else if self.map[i][j] == '#' {
                    free = i + 1;
                }
            }
        }
        // Move west
        for i in 0..self.height {
            let mut free = 0;
            for j in 0..self.width {
                if self.map[i][j] == 'O' {
                    self.map[i][j] = '.';
                    self.map[i][free] = 'O';
                    free += 1;
                } else if self.map[i][j] == '#' {
                    free = j + 1;
                }
            }
        }
        // Move south
        for j in 0..self.width {
            let mut free = self.height - 1;
            for i in (0..self.height).rev() {
                if self.map[i][j] == 'O' {
                    self.map[i][j] = '.';
                    self.map[free][j] = 'O';
                    if free > 0 {
                        free -= 1;
                    }
                } else if self.map[i][j] == '#' {
                    if i > 0 {
                        free = i - 1;
                    }
                }
            }
        }
        // Move east
        for i in 0..self.height {
            let mut free = self.width - 1;
            for j in (0..self.width).rev() {
                if self.map[i][j] == 'O' {
                    self.map[i][j] = '.';
                    self.map[i][free] = 'O';
                    if free > 0 {
                        free -= 1;
                    }
                } else if self.map[i][j] == '#' {
                    if j > 0 {
                        free = j - 1;
                    }
                }
            }
        }
    }

    fn _display_grid(&self) {
        // For debugging
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", self.map[i][j]);
            }
            println!();
        }
        println!();
    }

    fn score(&self) -> i32 {
        let mut scores = vec![];
        for j in 0..self.width {
            for i in 0..self.height {
                if self.map[i][j] == 'O' {
                    let weight = self.height - i;
                    scores.push(weight as i32);
                }
            }
        }
        return scores.iter().sum();
    }
}

fn parse(s: String) -> PuzzleInput {
    let map: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    let height = map.len();
    let width = map[0].len();
    return PuzzleInput { map, height, width };
}

fn part1(pi: &PuzzleInput) -> i32 {
    let mut scores = vec![];
    for j in 0..pi.width {
        let mut weight = pi.height as i32;
        for i in 0..pi.height {
            if pi.map[i][j] == '#' {
                weight = (pi.height - i - 1) as i32;
            } else if pi.map[i][j] == 'O' {
                scores.push(weight as i32);
                weight -= 1;
            }
        }
    }
    return scores.iter().sum();
}

fn part2(pi: &mut PuzzleInput) -> i32 {
    // The number of times to spin is too high, so some caching is needed.
    // The hope here is there are cycles.
    let mut cache: HashMap<Vec<Vec<char>>, i32> = HashMap::new();
    let mut rem = 1000000000;
    let mut i = 0;
    while rem > 0 {
        pi.spin();
        i += 1;
        rem -= 1;
        if cache.contains_key(&pi.map) {
            let last_seen = cache.get(&pi.map).unwrap();
            let cycle_length = i - last_seen;
            rem = rem % cycle_length;
        }
        // println!("{} {}", i, rem);
        // pi._display_grid();
        cache.insert(pi.map.clone(), i);
    }
    return pi.score();
}

fn main() {
    let s: String = fs::read_to_string("./src/input14.txt").unwrap();
    let mut inputs = parse(s);
    let ans: i32 = part1(&inputs);
    println!("{}", ans);
    let ans: i32 = part2(&mut inputs);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let mut inputs = parse(input_str.to_string());
        let ans: i32 = part1(&mut inputs);
        assert_eq!(ans, 136);
    }
    #[test]
    fn test_spin() {
        let input_str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let mut inputs = parse(input_str.to_string());
        inputs.spin();
        inputs.spin();
        inputs.spin();
        let result: String = inputs
            .map
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(
            result,
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
        );
    }
}
