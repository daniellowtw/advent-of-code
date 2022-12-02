use std::{fs, collections::HashMap};

fn parse_block(s: &str) -> (char, char) {
    let x: Vec<char> = s.chars().collect();
    return (x[0], x[2]);
}

fn main() {
    // PS: strategy is to hard code the mapping of each line. Much easier than coding the logic.

    let s: String = fs::read_to_string("./src/input2.txt").unwrap();
    let ss: Vec<(char, char)> = s.trim_end().split("\n").map(parse_block).collect(); 
    let mut map = HashMap::new();

    // (A, B, C) === (X, Y, Z) === (Rock, Paper, Scissors)
    // Your total score is the sum of your scores for each round. 
    // The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) 
    // plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
    map.insert(('A', 'X'), 4);
    map.insert(('B', 'X'), 1);
    map.insert(('C', 'X'), 7);
    map.insert(('A', 'Y'), 8);
    map.insert(('B', 'Y'), 5);
    map.insert(('C', 'Y'), 2);
    map.insert(('A', 'Z'), 3);
    map.insert(('B', 'Z'), 9);
    map.insert(('C', 'Z'), 6);

    let part1: i32 = ss.iter().map(|a| map.get(a).unwrap()).sum();
    println! ("Part 1: {:?}", part1);

    let mut map2 = HashMap::new();
    // The Elf finishes helping with the tent and sneaks back over to you. 
    // "Anyway, the second column says how the round needs to end: X means you need to lose, 
    // Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
    map2.insert(('A', 'X'), 3);
    map2.insert(('B', 'X'), 1);
    map2.insert(('C', 'X'), 2);
    map2.insert(('A', 'Y'), 4);
    map2.insert(('B', 'Y'), 5);
    map2.insert(('C', 'Y'), 6);
    map2.insert(('A', 'Z'), 8);
    map2.insert(('B', 'Z'), 9);
    map2.insert(('C', 'Z'), 7);
    let part2: i32 = ss.iter().map(|a| map2.get(a).unwrap()).sum();
    println! ("Part 2: {:?}", part2);
}
