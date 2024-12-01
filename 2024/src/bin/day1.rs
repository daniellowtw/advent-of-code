use std::{collections::HashMap, fs};


fn part1(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    a.sort();
    b.sort();
    return a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();
}

fn parse_into_two_arrays(s: &Vec<Vec<i32>>) -> (Vec<i32>, Vec<i32>) {
    let (a, b )= s.iter().fold((vec![], vec![]), |acc, x| {
        let mut a = acc.0.clone();
        let mut b = acc.1.clone();
        a.push(x[0]);
        b.push(x[1]);
        return (a, b);
    });
    (a, b)
}


fn part2(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut freq : HashMap<&i32, i32> = HashMap::new();
    b.iter().for_each(|x| {
        *freq.entry(x).or_insert(0) += 1
    });
    return a.iter().map(|x| freq.get(x).unwrap_or(&0) * x).sum();
}

fn main() {
    let s: String = fs::read_to_string("./src/input1.txt").unwrap();
    let ss: Vec<Vec<i32>> = s.split("\n")
    .filter(|x| !x.is_empty())
    .map(|x| x.split_whitespace().map(|y| y.parse::<i32>().unwrap()).collect()).collect();
    let (a, b) = parse_into_two_arrays(&ss);
    println!("{}", part1(a.clone(), b.clone()));
    println!("{}", part2(a, b));
}
