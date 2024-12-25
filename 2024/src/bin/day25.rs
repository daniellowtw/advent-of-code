use std::fs;

fn part1(pi: &str) -> i32 {
    let pi = parse(pi);
    let mut count = 0;
    for i in 0..pi.bottom.len() {
        for j in 0..pi.top.len() {
            if fits(&pi.top[j], &pi.bottom[i]) {
                count += 1;
            }
        }
    }
    return count;
}

fn fits(top: &Vec<usize>, bottom: &Vec<usize>) -> bool {
    for k in 0..bottom.len() {
        if bottom[k] + top[k] > 5 {
            return false;
        }
    }
    return true;
}

struct PuzzleInput {
    top: Vec<Vec<usize>>,
    bottom: Vec<Vec<usize>>,
}

fn parse_chunk(s: &str) -> (Vec<usize>, bool) {
    let lines: Vec<Vec<char>> = s.lines().map(|x| x.chars().collect()).collect();
    let is_top = lines[0][0] == '#';
    let mut res = Vec::new();
    for i in 0..lines[0].len() {
        res.push((1..lines.len() - 1).filter(|j| lines[*j][i] == '#').count());
    }
    return (res, is_top);
}

fn parse(s: &str) -> PuzzleInput {
    let mut tops: Vec<Vec<usize>> = Vec::new();
    let mut bottom: Vec<Vec<usize>> = Vec::new();
    s.split("\n\n").for_each(|chunk| {
        let (x, is_top) = parse_chunk(chunk);
        if is_top {
            tops.push(x);
        } else {
            bottom.push(x);
        }
    });
    return PuzzleInput { top: tops, bottom };
}

fn main() {
    let s: String = fs::read_to_string("./input/25.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-25.txt").unwrap();
    println!("{}", part1(&s));
}
