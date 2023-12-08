use std::{collections::HashMap, fs};

#[derive(Debug)]
struct PuzzleInput {
    instructions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

fn parse(s: String) -> PuzzleInput {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let instructions = parts[0].chars().collect();
    let nodes = parts[1]
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            //Example: AAA = (BBB, CCC)
            let parts: Vec<&str> = line.split(" = ").collect();
            let key = parts[0].to_string();
            let values: Vec<&str> = parts[1].split(", ").collect();
            let value = (values[0][1..].to_string(), values[1][..3].to_string());
            (key, value)
        })
        .collect();
    return PuzzleInput {
        instructions,
        nodes,
    };
}

fn part1(input: &PuzzleInput) -> i32 {
    let mut count = 0;
    let mut current = "AAA";
    for i in input.instructions.iter().cycle() {
        // println!("{}", current);
        if *i == 'L' {
            current = &input.nodes.get(current).unwrap().0;
            count += 1;
        } else {
            current = &input.nodes.get(current).unwrap().1;
            count += 1;
        }
        if current == "ZZZ" {
            return count;
        }
    }
    panic!();
}

fn find_cycle(input: &PuzzleInput, starting: &String) -> i64 {
    let mut count = 0;
    let mut current = starting;

    for i in input.instructions.iter().cycle() {
        if *i == 'L' {
            current = &input.nodes.get(current).unwrap().0;
            count += 1;
        } else {
            current = &input.nodes.get(current).unwrap().1;
            count += 1;
        }
        if current.ends_with('Z') {
            return count;
        }
    }
    panic!();
}

fn part2(input: &PuzzleInput) -> i64 {
    // Strategy: HUGE ASSUMPTION: Once you reach a Z, it becomes a cycle. This is not necessarily true in general.
    let current_nodes: Vec<&String> = input
        .nodes
        .iter()
        .map(|x| x.0)
        .filter(|x| (*x).ends_with('A'))
        .collect();
    // println!("{:?}", current_nodes);

    let cycles: Vec<i64> = current_nodes
        .iter()
        .map(|x| find_cycle(&input, x))
        .collect();

    // print!("{:?}", cycles);
    return cycles.into_iter().reduce(lcm).unwrap();
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn main() {
    let s: String = fs::read_to_string("./src/input8.txt").unwrap();
    let inputs = parse(s);
    // println!("{:?}", &inputs);
    let ans1 = part1(&inputs);
    println!("{}", ans1);
    let ans2 = part2(&inputs);
    println!("{}", ans2);
}
