use std::{fs, vec};

#[derive(Debug)]
struct PuzzleInput {
    instructions: Vec<Vec<char>>,
}

fn parse(s: String) -> PuzzleInput {
    let instructions: Vec<Vec<char>> = s.split(",").map(|x| x.chars().collect()).collect();
    return PuzzleInput { instructions };
}

fn do_hash(s: &Vec<char>) -> u32 {
    let mut score = 0;
    for c in s {
        score += *c as u32;
        score *= 17;
        score %= 256;
    }
    return score;
}

fn part1(pi: &PuzzleInput) -> i32 {
    return pi.instructions.iter().map(|x| do_hash(x)).sum::<u32>() as i32;
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Sub(usize, String),
    Add(usize, String, i8),
}
fn part2(pi: &PuzzleInput) -> i32 {
    let mut boxes: Vec<Vec<(String, i8)>> = vec![vec![]; 256];
    for s in &pi.instructions {
        let pos = s.iter().enumerate().find(|x| *x.1 == '=');
        let op = match pos {
            Some(pos) => {
                let box_number = do_hash(&s[0..pos.0].to_vec()) as usize;
                Op::Add(
                    box_number,
                    s[0..pos.0].iter().collect::<String>(),
                    s[pos.0 + 1].to_digit(10).unwrap() as i8,
                )
            }
            None => {
                let box_number = do_hash(&s[0..s.len() - 1].to_vec()) as usize;
                Op::Sub(box_number, s[0..s.len() - 1].iter().collect::<String>())
            }
        };
        // println!("{:?} {:?}", s, op);
        match op {
            Op::Sub(box_number, label) => {
                boxes[box_number].retain(|x| x.0 != label);
            }
            Op::Add(box_number, label, number) => {
                let mut found = false;
                boxes[box_number].iter_mut().for_each(|v| {
                    if v.0 == label {
                        v.1 = number;
                        found = true;
                    }
                });
                if found == false {
                    boxes[box_number as usize].push((label, number));
                }
            }
        }
    }
    let mut score = 0;
    for i in 0..256 {
        for j in 0..boxes[i].len() {
            score += (1 + i as i32) * (j as i32 + 1) * boxes[i][j].1 as i32;
        }
    }
    return score;
}

fn main() {
    let s: String = fs::read_to_string("./src/input15.txt").unwrap();
    let inputs = parse(s.trim().to_string());
    let ans: i32 = part1(&inputs);
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
