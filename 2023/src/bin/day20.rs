use std::{
    collections::{HashMap, VecDeque},
    fs, vec,
};

#[derive(Debug, Clone)]
struct PuzzleInput {
    rows: HashMap<String, (String, Vec<String>)>,
    hm: HashMap<String, Node>,
}

fn parse(s: &str) -> PuzzleInput {
    let rows: HashMap<String, (String, Vec<String>)> = s
        .trim()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            // %jx -> rt, rs
            let parts: Vec<&str> = x.split(" -> ").collect();
            let name = parts[0].trim().to_string();
            let module_type = if name == "broadcaster" {
                ("b", "broadcaster")
            } else {
                name.split_at(1)
            };
            let dst: Vec<String> = parts[1].trim().split(", ").map(|x| x.to_string()).collect();
            (module_type.1.to_string(), (module_type.0.to_string(), dst))
        })
        .into_iter()
        .collect();

    let mut hm: HashMap<String, Node> = HashMap::new();
    for i in rows.iter() {
        let node = Node {
            children: i.1 .1.clone(),
            node_type: i.1 .0.chars().next().unwrap(),
            incoming: HashMap::new(),
            state: false,
        };
        hm.insert(i.0.clone(), node);
    }

    for i in rows.iter() {
        for dst in i.1 .1.iter() {
            match hm.get_mut(dst) {
                Some(node) => {
                    node.incoming.insert(i.0.clone(), false);
                }
                None => {}
            }
        }
    }
    return PuzzleInput { rows, hm };
}

#[derive(Debug, Clone)]
struct Node {
    children: Vec<String>,
    node_type: char,
    incoming: HashMap<String, bool>, // For conjunctions
    state: bool,                     // for latches
}

fn observe(hm: &HashMap<String, Node>, name: Vec<(&str, bool)>, i: i32) -> Option<i64> {
    for (n, cond) in name {
        let node = hm.get(n).unwrap();
        if node.incoming.iter().all(|x| x.1 == &cond) {
            // println!(
            //     "{}: {} {} - {:?}",
            //     i,
            //     n,
            //     node.node_type,
            //     node.incoming.clone()
            // );
            return Some(i as i64); // We found the cycle.
        }
    }
    return None;
}

fn part1(pi: PuzzleInput, times: i32) -> i64 {
    let mut res = (0, 0); // low, high or false, true
    let mut queue = VecDeque::new();
    let mut hm = pi.hm;
    for _ in 0..times {
        queue.push_back(("broadcaster".to_string(), false));
        res.0 += 1;
        while !queue.is_empty() {
            let (name, signal) = queue.pop_front().unwrap();
            let node = hm.get_mut(&name).unwrap();
            // Mutate node state and figure out the signal and children to send to.
            let (new_signal, children) = match node.node_type {
                'b' => (signal, node.children.clone()),
                '%' => {
                    if !signal {
                        // Only activate when low signal is received
                        node.state = !node.state;
                        (node.state, node.children.clone())
                    } else {
                        (signal, vec![])
                    }
                }
                '&' => {
                    let all_high = node.incoming.iter().all(|x| x.1 == &true);
                    (!all_high, node.children.clone())
                }
                _ => {
                    panic!("Unknown node type")
                }
            };

            for child in children {
                let node = hm.get_mut(&child);
                match node {
                    Some(node) => {
                        node.incoming.insert(name.clone(), new_signal);
                        if new_signal {
                            res.1 += 1
                        } else {
                            res.0 += 1
                        }
                        queue.push_back((child.clone(), new_signal));
                    }
                    None => {
                        if new_signal {
                            res.1 += 1;
                        } else {
                            res.0 += 1;
                        }
                    }
                }
            }
        }
    }

    return res.0 * res.1;
}

fn part2(pi: PuzzleInput) -> i64 {
    // In reality I found the penultimate node by looking at the input. But here's a programmatic version.
    // When I manually inspected the final node and see what nodes led to it, I realized that the nodes
    // leading to it all had to be low. Then I recursively inspected those penultimate nodes, and also noticed
    // they were all conjunctions. My next intuition was to figure out the pattern for when those penultimate
    // nodes emiited high, so that the final node would emit low. I wrote "observe" to look at the inputs and
    // noticed a pattern that they only became the correct value curing certain cycles. Given it was a small number
    // of inputs to the final node, I just manually inspected the cycles (4 of them) and found the LCM.
    let final_node = pi
        .rows
        .iter()
        .find(|i| i.1 .1.contains(&String::from("rx")))
        .unwrap();
    let penultimate_nodes: Vec<_> = pi
        .rows
        .iter()
        .filter(|i| i.1 .1.contains(final_node.0))
        .map(|x| x.0)
        .collect();

    assert!(
        final_node.1 .0 == "&",
        "Algo assumes final node is a conjunction"
    );

    // dbg!(&penultimate_nodes);

    // Find the cycles and take LCM
    let cycles: Vec<_> = penultimate_nodes
        .iter()
        .map(|x| {
            let mut queue = VecDeque::new();
            let mut hm = pi.hm.clone();
            for i in 0..10000 {
                queue.push_back(("broadcaster".to_string(), false));
                while !queue.is_empty() {
                    if i > 10 {
                        if let Some(c) = observe(&hm, vec![(x, false)], i + 1) {
                            return (x, c);
                        }
                    }
                    let (name, signal) = queue.pop_front().unwrap();
                    let node = hm.get_mut(&name).unwrap();
                    let (new_signal, children) = match node.node_type {
                        'b' => (signal, node.children.clone()),
                        '%' => {
                            if !signal {
                                // Only activate when low signal is received
                                node.state = !node.state;
                                (node.state, node.children.clone())
                            } else {
                                (signal, vec![])
                            }
                        }
                        '&' => {
                            let all_high = node.incoming.iter().all(|x| x.1 == &true);
                            (!all_high, node.children.clone())
                        }
                        _ => {
                            panic!("Unknown node type")
                        }
                    };
                    for child in children {
                        let node = hm.get_mut(&child);
                        match node {
                            Some(node) => {
                                node.incoming.insert(name.clone(), new_signal);
                                queue.push_back((child.clone(), new_signal));
                            }
                            None => {}
                        }
                    }
                }
            }
            panic!();
        })
        .collect();

    // dbg!(&cycles);

    let ans = cycles
        .iter()
        .map(|x| x.1)
        .reduce(|a, b| lcm(a as i64, b as i64));
    return ans.unwrap();
}
fn main() {
    let s: String = fs::read_to_string("./src/input20.txt").unwrap();
    let pi: PuzzleInput = parse(s.trim());
    let ans = part1(pi.clone(), 1000);
    println!("{}", ans);
    let ans = part2(pi);
    println!("{}", ans);
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

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    #[test]
    fn test_part1_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim());
        let ans = part1(pi, 1000);
        println!("{}", ans);
    }
    #[test]

    fn test_part1_sample_input_2() {
        const SAMPLE_INPUT: &str = "
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
        let pi = parse(SAMPLE_INPUT.trim());
        // println!("{:?}", pi);
        let ans = part1(pi, 1000);
        println!("{}", ans);
    }
}
