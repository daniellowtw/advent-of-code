use std::{fs, collections::{HashSet}};

fn solve_line(s: &str) -> u8{
    // PS: Seems like splitting into two parts wasn't necessary. Just needed to find repeated character in the string.
    let fst = &s[0..s.len()/2];
    let snd = &s[s.len()/2..];
    let fst_set: HashSet<char> = HashSet::from_iter(fst.chars());
    let snd_set: HashSet<char> = HashSet::from_iter(snd.chars());
    let repeated = fst_set.intersection(&snd_set).collect::<Vec<&char>>()[0].clone();
    if repeated.is_ascii_uppercase() {
        return repeated as u8 - 'A' as u8 + 1 + 26;
    } else {
        return repeated as u8 - 'a' as u8 + 1;
    }
}

fn solve_3(ss: &[&str]) -> u8{
    let fst_set: HashSet<char> = HashSet::from_iter(ss[0].chars());
    let snd_set: HashSet<char> = HashSet::from_iter(ss[1].chars());
    let third_set: HashSet<char> = HashSet::from_iter(ss[2].chars());
    let repeated: HashSet<char> = fst_set.intersection(&snd_set).map(|x| x.clone()).collect::<HashSet<char>>();
    let res = repeated.intersection(&third_set).collect::<Vec<&char>>()[0].clone();
    if res.is_ascii_uppercase() {
        return res as u8 - 'A' as u8 + 1 + 26;
    } else {
        return res as u8 - 'a' as u8 + 1;
    }
}

fn main() {

    // Check logic for small input.
    // dbg!(solve_line("vJrwpWtwJgWrhcsFMMfFFhFp"));
    // return;
    let s: String = fs::read_to_string("./src/input3.txt").unwrap();
    let ss: Vec<&str> = s.trim_end().split("\n").collect(); 
    let part1: i32 = ss.iter().map(|a| solve_line(a) as i32).sum();
    println! ("Part 1: {:?}", part1);

    // Check logic for small input.
    // let xx = ["vJrwpWtwJgWrhcsFMMfFFhFp",
    // "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
    // "PmmdzqPrVvPwwTWBwg"];
    // dbg!(solve_3(&xx));
    
    let part2: i32 = ss.chunks(3).map(|a| solve_3(a)as i32).sum();
    println! ("Part 2: {:?}", part2);

}
