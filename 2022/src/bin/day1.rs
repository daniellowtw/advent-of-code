use std::fs;

fn parse_block(s: &str) -> i32 {
    let x: i32 = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .sum();
    return x;
}

fn main() {
    let s: String = fs::read_to_string("./src/input1.txt").unwrap();
    let mut ss: Vec<i32> = s.split("\n\n").map(parse_block).collect(); 
    ss.sort_by(|a, b| b.cmp(a));
    println!("{}", ss[0]);
    println!("{}", ss[0] + ss[1] + ss[2]);
}
