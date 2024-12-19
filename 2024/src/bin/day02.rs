use std::fs;

fn is_safe(s: &Vec<i32>) -> bool {
    let mut current = s.first().unwrap();
    // Takes care of cases where two numbers are the same.
    // We cannot just compare the two. Use the inverse of the condition to short circuit the logic.
    let is_increasing = !s.windows(2).any(|x| x[1] - x[0] < 0);

    for x in s.iter().skip(1) {
        if is_increasing && (x - current < 1 || x - current > 3) {
            return false;
        }
        if !is_increasing && (current - x < 1 || current - x > 3) {
            return false;
        }
        current = x;
    }
    true
}

fn is_safe_with_removal(s: &Vec<i32>) -> bool {
    if is_safe(s) {
        return true;
    }
    for i in 0..s.len() {
        let mut s = s.clone();
        s.remove(i);
        if is_safe(&s) {
            return true;
        }
    }
    false
}

fn part1(ss: &Vec<Vec<i32>>) -> i32 {
    return ss.iter().filter(|x| is_safe(x)).count() as i32;
}

fn part2(ss: &Vec<Vec<i32>>) -> i32 {
    return ss.iter().filter(|x| is_safe_with_removal(x)).count() as i32;
}

fn main() {
    let s: String = fs::read_to_string("./input/02.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-02.txt").unwrap();
    let ss: Vec<Vec<i32>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    println!("{}", part1(&ss));
    println!("{}", part2(&ss));
}
