use std::fs;

fn part1(pi: &Vec<i32>, target: i32) -> i32 {
    return two_sum(pi, target);
}

fn two_sum(pi: &Vec<i32>, target: i32) -> i32 {
    // 2 sum problem
    let mut left = 0;
    let mut right = pi.len() - 1;
    while left < right {
        if pi[left] + pi[right] == target {
            return pi[left] * pi[right];
        }
        if pi[left] + pi[right] < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    return 0;
}

fn part2(pi: &Vec<i32>, target: i32) -> i32 {
    for i in 0..pi.len() {
        let complement = target - pi[i];
        let sub_vec = pi[1..].to_vec();
        let product = two_sum(&sub_vec, complement);
        if product != 0 {
            return product * pi[i];
        }
    }
    return 0;
}

fn main() {
    let s: String = fs::read_to_string("./input/01.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-01.txt").unwrap();
    let mut ss: Vec<i32> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    ss.sort();
    println!("{}", part1(&ss, 2020));
    println!("{}", part2(&ss, 2020));
}
