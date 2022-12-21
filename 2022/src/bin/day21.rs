use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
enum Monkey {
    Number(i64),
    Op(String, char, String),
}

fn main() {
    let s: String = fs::read_to_string("./src/input21.txt").unwrap();
    let nodes: HashMap<String, Monkey> = s
        .trim_end()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let (name, load) = x.split_once(":").unwrap();
            let parts: Vec<&str> = load.trim().split(" ").collect();
            if parts.len() == 1 {
                (name.to_string(), Monkey::Number(parts[0].parse().unwrap()))
            } else {
                let a = parts[0].to_string();
                let op: char = parts[1].chars().nth(0).unwrap();
                let b = parts[2].to_string();
                (name.to_string(), Monkey::Op(a, op, b))
            }
        })
        .collect();
    let part1 = solve1(nodes.clone());
    dbg!(part1);
    let part2 = solve2(nodes);
    dbg!(part2);
}

fn eval(a: i64, op: char, b: i64) -> i64 {
    match op {
        '+' => a + b,
        '*' => a * b,
        '/' => a / b,
        '-' => a - b,
        _ => todo!(),
    }
}

fn eval_node(node: &Monkey, lookup: &HashMap<String, Monkey>) -> i64 {
    match node {
        Monkey::Number(x) => *x,
        Monkey::Op(a, op, b) => eval(
            eval_node(lookup.get(a).unwrap(), lookup),
            *op,
            eval_node(lookup.get(b).unwrap(), lookup),
        ),
    }
}

fn solve1(lookup: HashMap<String, Monkey>) -> i64 {
    eval_node(lookup.get("root").unwrap(), &lookup)
}

fn solve2(mut lookup: HashMap<String, Monkey>) -> i64 {
    // Idea: Turn root node into a - b, then do an evaluation where the target is 0.
    let root = lookup.get("root").unwrap();
    let new_root: Monkey = match root {
        Monkey::Op(a, _, b) => Monkey::Op(a.to_string(), '-', b.to_string()),
        _ => panic!(),
    };
    lookup.insert(String::from("root"), new_root);

    // First step is to turn the input into a symbolic tree.
    let simplified = post_order_traversal(String::from("root"), &lookup);
    // Then evaluate value of human with the target being 0.
    solve_for_human(simplified, 0)
}

fn solve_for_human(node: SymbolicBinaryOp, target: i64) -> i64 {
    match node {
        SymbolicBinaryOp::Human => target, // effectively node is x = target
        SymbolicBinaryOp::Left(a, op, b) => {
            let new_target = match op {
                '+' => target - b,
                '*' => target / b,
                '/' => target * b,
                '-' => target + b,
                _ => panic!(),
            };
            solve_for_human(*a, new_target)
        }
        SymbolicBinaryOp::Right(a, op, b) => {
            let new_target = match op {
                '+' => target - a,
                '*' => target / a,
                '/' => a / target,  // node is a / x = target, so x = a / target
                '-' => a - target, // node is a - x = target, so x = a - target
                _ => panic!(),
            };
            solve_for_human(*b, new_target)
        }
        _ => panic!(),
    }
}

enum SymbolicBinaryOp {
    Human,
    Number(i64),
    Left(Box<SymbolicBinaryOp>, char, i64),
    Right(i64, char, Box<SymbolicBinaryOp>),
}

fn post_order_traversal(name: String, lookup: &HashMap<String, Monkey>) -> SymbolicBinaryOp {
    if name == "humn" {
        return SymbolicBinaryOp::Human;
    }
    let node = lookup.get(&name).unwrap();

    match node {
        Monkey::Number(x) => SymbolicBinaryOp::Number(*x),
        Monkey::Op(a, op, b) => {
            let left = post_order_traversal(a.to_string(), lookup);
            let right = post_order_traversal(b.to_string(), lookup);
            match (left, right) {
                (SymbolicBinaryOp::Number(x), SymbolicBinaryOp::Number(y)) => SymbolicBinaryOp::Number(eval(x, *op, y)),
                (m, SymbolicBinaryOp::Number(y)) => SymbolicBinaryOp::Left(Box::new(m), *op, y),
                (SymbolicBinaryOp::Number(y), m) => SymbolicBinaryOp::Right(y, *op, Box::new(m)),
                _ => panic!(),
            }
        }
    }
}
