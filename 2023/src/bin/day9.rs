use std::fs;

#[derive(Debug)]
struct PuzzleInput {
    lines: Vec<Vec<i32>>,
}

fn parse(s: String) -> PuzzleInput {
    let lines = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split(" ").map(|y| y.parse::<i32>().unwrap()).collect())
        .collect();
    return PuzzleInput { lines };
}

fn part2(xs: &Vec<i32>) -> i32 {
    let mut next_line = vec![];
    for i in 1..xs.len() {
        next_line.push(xs[i] - xs[i - 1]);
    }
    let first_val = xs[0];
    if next_line.iter().all(|x| *x == 0) {
        return first_val;
    } else {
        let prev_val = part2(&next_line);
        return first_val - prev_val;
    }
}

fn part1(xs: &Vec<i32>) -> i32 {
    let mut next_line = vec![];
    for i in 1..xs.len() {
        next_line.push(xs[i] - xs[i - 1]);
    }
    let last_val = xs[xs.len() - 1];
    if next_line.iter().all(|x| *x == 0) {
        return last_val;
    } else {
        let next_val = part1(&next_line);
        return last_val + next_val;
    }
}

fn main() {
    let s: String = fs::read_to_string("./src/input9.txt").unwrap();
    let inputs = parse(s);
    // println!("{:?}", &inputs);
    let ans1: i32 = inputs.lines.iter().map(|x| part1(x)).sum();
    println!("{}", ans1);
    let ans2: i32 = inputs.lines.iter().map(|x| part2(x)).sum();
    println!("{}", ans2);
}
