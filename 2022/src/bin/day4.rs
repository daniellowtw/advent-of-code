use std::{fs};

fn parse_line(s: &str) -> ((i32, i32), (i32, i32)) {
    let res = s.split_once(",").unwrap();
    let fst = res.0.split_once("-").unwrap();
    let fst = (fst.0.parse::<i32>().unwrap(), fst.1.parse::<i32>().unwrap());
    let snd = res.1.split_once("-").unwrap();
    let snd = (snd.0.parse::<i32>().unwrap(), snd.1.parse::<i32>().unwrap());
    if fst.0 > snd.0 {
        return (snd, fst);
    } else {
        return (fst, snd);
    }
}

fn find_complete_overlap(fst: &(i32, i32), snd: &(i32, i32)) -> i32 {
    // fst.0 is always <= snd.0
    if snd.1 <= fst.1 {
        return 1;
    } else if fst.0 == snd.0 && fst.1 <= snd.1{
        // first pair is shorter
        return 1;
    } else {
        return 0;
    }
}

fn find_partial_overlap(fst: &(i32, i32), snd: &(i32, i32)) -> i32 {
    // The only time it is not a partial overlap is if the first segment ends before the second.
    if fst.1 < snd.0 {
        return 0;
    } else {
        return 1;
    }
}

fn main() {
    let s: String = fs::read_to_string("./src/input4.txt").unwrap();
    let ss: Vec<&str> = s.trim_end().split("\n").collect();
    let input = ss.into_iter().map(parse_line);
    let part1: i32 = input.clone().map(|x| find_complete_overlap(&x.0, &x.1)).sum();
    println!("Part 1: {:?}", part1);
    let part2: i32 = input.clone().map(|x| find_partial_overlap(&x.0, &x.1)).sum();
    println!("Part 2: {:?}", part2);
}
