use std::fs;
use regex::Regex;

fn part1(ss: &Vec<Vec<(i32, i32)>>) -> i32 {
    return ss.iter().map(|x| x.iter().map(|(a, b)| a * b).sum::<i32>()).sum()
}

fn part2(ss: &Vec<Vec<Inst>>) -> i32 {
    let mut include= true;
    let mut result = 0;
    for i in ss {
        for j in i {
            match j {
                Inst::Exclude => include=false,
                Inst::Include => include=true,
                Inst::Mul(a, b) => {
                    if include {
                        result += a * b;
                    }
                }
            }
        }
    }
    return result;
}

fn extract_all_mul_pairs(s: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    return re.captures_iter(s).map(|x| (x[1].parse::<i32>().unwrap(), x[2].parse::<i32>().unwrap())).collect();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Inst {
    Include,
    Exclude,
    Mul(i32, i32),
}

fn extract_all_mul_pairs_with_conditionals(s: &str) -> Vec<Inst> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(don't\(\))|(do\(\))").unwrap();
    return re.captures_iter(s).map(
        |x| 
        {
            let ss = x.get(0).unwrap().as_str();
            match ss {
                    "don't()" => {
                        return Inst::Exclude
                    },
                    "do()" => {
                        return Inst::Include
                    },
                    _ => {
                        return Inst::Mul(x[1].parse::<i32>().unwrap(), x[2].parse::<i32>().unwrap())
                    }
                }
        }
    ).collect();
}

fn main() {
    let s: String = fs::read_to_string("./input/03.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-03.txt").unwrap();
    // Note: example for part 2 is different from part 1
    let ss: Vec<Vec<(i32, i32)>> = s.split("\n")
    .filter(|x| !x.is_empty())
    .map(|x| extract_all_mul_pairs(x)).collect();
    println!("{}", part1(&ss));
    let ss2: Vec<Vec<Inst>> = s.split("\n")
    .filter(|x| !x.is_empty())
    .map(|x| extract_all_mul_pairs_with_conditionals(x)).collect();
    println!("{}", part2(&ss2));
}

