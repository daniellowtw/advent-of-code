use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn dfs(curr: usize, order: &mut Vec<usize>, seen: &mut HashSet<usize>, pi: &PuzzleInput) {
    if seen.contains(&curr) {
        return;
    }
    seen.insert(curr);
    let i = pi.instructions.get(curr).unwrap();
    pi.instructions
        .iter()
        .enumerate()
        .filter(|(_, v)| v.0 == i.3 || v.2 == i.3)
        .map(|x| x.0)
        .for_each(|x| {
            dfs(x, order, seen, pi);
        });
    order.push(curr);
}

fn run_circuit(pi: &PuzzleInput, order: &[usize]) -> HashMap<String, bool> {
    let mut data = pi.data.clone();
    for idx in order.iter() {
        let i = &pi.instructions[*idx];
        // dbg!(&i);
        match i.1.as_str() {
            "AND" => {
                let a = *data.get(&i.0).unwrap();
                let b = *data.get(&i.2).unwrap();
                data.insert(i.3.to_string(), a & b);
            }
            "OR" => {
                let a = *data.get(&i.0).unwrap();
                let b = *data.get(&i.2).unwrap();
                data.insert(i.3.to_string(), a | b);
            }
            "XOR" => {
                let a = *data.get(&i.0).unwrap();
                let b = *data.get(&i.2).unwrap();
                data.insert(i.3.to_string(), a ^ b);
            }
            _ => panic!("Unknown instruction"),
        }
    }
    data
}

fn read_register(data: &HashMap<String, bool>, s: &str) -> i64 {
    data.iter()
        .filter(|x| x.0.to_string().starts_with(s))
        .sorted()
        .rev()
        .fold(0, |i, x| {
            let mut i = i << 1;
            if *x.1 {
                i += 1;
            }
            i
        })
}

fn part1(pi: &str) -> i64 {
    // The trick here is figuring out the order of the instructions to execute.
    // This can be solved by topological sorting.
    let pi = parse(pi);
    let mut order = Vec::new();
    let mut seen = HashSet::new();
    for i in 0..pi.instructions.len() {
        dfs(i, &mut order, &mut seen, &pi);
    }
    order.reverse();
    let data = run_circuit(&pi, &order);
    read_register(&data, "z")
}

fn _check(i: i32, pi: &PuzzleInput) {
    // My attempt is to detect which bits are wrong in the adder.
    // My hypothesis is that if we want to test whether bit i is correct, we just need to set the bits i-1 and i to 1.
    // This is sufficient to test bit i independent of the other bits.
    // Coupled with a visuzl map of the circuit, the answer should be obvious.
    let mut pi = pi.clone();
    for j in pi.data.iter_mut() {
        *j.1 = false;
    }
    let mut order = Vec::new();
    let mut seen = HashSet::new();
    for i in 0..pi.instructions.len() {
        dfs(i, &mut order, &mut seen, &pi);
    }
    order.reverse();
    let j = i - 1;
    pi.data.insert(format!("x{:02}", j), true);
    pi.data.insert(format!("y{:02}", j), true);

    let j = i;
    pi.data.insert(format!("x{:02}", j), true);
    pi.data.insert(format!("y{:02}", j), true);
    let x = read_register(&pi.data, "y");
    let data = run_circuit(&pi, &order);
    let res = read_register(&data, "z");
    let expected_ans = if i > 0 { 0b11_i64 << i } else { 0b10_i64 };
    if res != expected_ans {
        println!("FAILED at {}: expected {} actual {}", i, expected_ans, res);
        println!("x        {:b}", x);
        println!("expected {:b}", expected_ans);
        println!("actual   {:b}", res);
    }
}

fn _part2_mermaid(s: &str) -> i32 {
    let pi = parse(s);
    for i in pi.instructions.iter() {
        println!("{} --> |{}| {}", i.0, i.1, i.3);
        println!("{} --> |{}| {}", i.2, i.1, i.3);
    }
    0
    // return 0;
}

fn part2(s: &str) -> i32 {
    // I did this by hand. The idea was
    // 1. Use _check to identify wrong bits
    // 2. Use a visual map of the circuit to identify the mistakes.

    // let pi = parse(s.as_str());
    // for i in 0..43 {
    //     check(i, &pi, &[]);
    // }
    0
}

#[derive(Debug, Clone)]
struct PuzzleInput {
    data: HashMap<String, bool>,
    instructions: Vec<(String, String, String, String)>,
}

fn parse(s: &str) -> PuzzleInput {
    let (a, b) = s.split_once("\n\n").unwrap();
    let mut data: HashMap<String, bool> = a
        .lines()
        .map(|x| {
            let (a, b) = x.split_once(": ").unwrap();
            (a.to_string(), b.parse::<i8>().unwrap() == 1)
        })
        .collect();
    let instructions: Vec<(String, String, String, String)> = b
        .lines()
        .map(|x| {
            let (a, b) = x.split_once(" -> ").unwrap();
            let c: Vec<&str> = a.split(" ").collect();
            data.entry(c[0].to_string()).or_insert(false);
            data.entry(c[2].to_string()).or_insert(false);
            data.entry(b.to_string()).or_insert(false);
            (
                c[0].to_string(),
                c[1].to_string(),
                c[2].to_string(),
                b.to_string(),
            )
        })
        .collect();
    return PuzzleInput { data, instructions };
}

fn main() {
    let s: String = fs::read_to_string("./input/24.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-24.txt").unwrap();

    println!("{}", part1(&s));
    println!("{}", part2(&s));
    // println!("{:b}", 0b1111111111111111);
}
