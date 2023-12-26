#![allow(warnings)]
use core::panic;
use std::{
    cmp::{max, Ordering},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs, io,
    ops::RangeBounds,
};

#[derive(Debug, Clone)]
struct PuzzleInput {
    children: HashMap<String, Vec<String>>,
    parent: HashMap<String, Vec<String>>,
    names: HashSet<String>,
    edges: HashSet<(String, String)>,
}

fn parse(s: &str) -> PuzzleInput {
    let mut names = HashSet::new();
    // Treat it as directed graph since that's what the input is.
    let children: HashMap<String, Vec<String>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let parts: Vec<_> = x.split(": ").collect();
            let name = parts[0].trim().to_string();
            names.insert(name.clone());
            let y: Vec<String> = parts[1]
                .trim()
                .split(" ")
                .map(|x| {
                    let x = x.trim();
                    names.insert(x.to_string());
                    x.to_string()
                })
                .collect();
            return (name, y);
        })
        .collect();

    let mut reverse = HashMap::new();
    for i in children.iter() {
        for j in i.1.iter() {
            reverse.entry(j.clone()).or_insert(vec![]).push(i.0.clone());
        }
    }

    let mut edges = HashSet::new();
    for i in children.iter() {
        for j in i.1.iter() {
            edges.insert((i.0.clone(), j.clone()));
        }
    }

    return PuzzleInput {
        children,
        names,
        parent: reverse,
        edges,
    };
}

fn graphviz_cheat(pi: &PuzzleInput) -> usize {
    // I cheated slightly after I realized a simple brute force doesn't work for the input. (Need to iterate > 10^9 options.)
    // Instead, I produce a graphviz input and then use graphviz to graphically solve it.

    // I have some ideas of using min-cut to solve this. But too lazy to solve this properly on Christmas day.
    println!("graph G {{");
    for i in pi.children.iter() {
        for j in i.1 {
            println!("{} -- {}", i.0, j);
        }
    }
    println!("}}");
    // Upon inspecting I can visually identify the following edges as the ones that need to be removed.
    let ans = score_components(
        &pi,
        &("xgs".to_string(), "lmj".to_string()),
        &("hgk".to_string(), "pgz".to_string()),
        &("qnz".to_string(), "gzr".to_string()),
    )
    .unwrap();
    return ans;
}

fn part1(pi: &PuzzleInput) -> i64 {
    dbg!(pi.edges.len());
    let edges: Vec<_> = pi.edges.iter().collect();
    for i in 0..edges.len() {
        for j in i + 1..edges.len() {
            for k in j + 1..edges.len() {
                let a = &edges[i];
                let b = &edges[j];
                let c = &edges[k];
                if let Some(x) = score_components(pi, &a, &b, &c) {
                    return x as i64;
                }
            }
        }
    }
    panic!();
}

fn score_components(
    pi: &PuzzleInput,
    a: &(String, String),
    b: &(String, String),
    c: &(String, String),
) -> Option<usize> {
    let mut left_to_colour: HashSet<_> = pi.names.iter().collect();
    let mut colouring: HashMap<String, usize> = HashMap::new();
    let mut colour = 0;
    while !left_to_colour.is_empty() {
        let nodes = left_to_colour.iter().next().unwrap().clone();
        colour += 1;
        if colour > 2 {
            return None;
        }
        colouring.insert(nodes.clone(), colour);

        let mut queue: Vec<String> = vec![nodes.clone()];
        while !queue.is_empty() {
            let node = queue.pop().unwrap();
            if !left_to_colour.contains(&node) {
                continue;
            }
            left_to_colour.remove(&node);

            // Ok.. too lazy to write this nicely.
            let children = match pi.children.get(&node) {
                Some(x) => {
                    let mut res = x.clone();
                    match pi.parent.get(&node) {
                        Some(x) => {
                            for i in x {
                                res.push(i.clone());
                            }
                        }
                        None => {}
                    }
                    res
                }
                None => pi.parent.get(&node).unwrap().clone(),
            };

            for child in children {
                if a.0 == *node && a.1 == *child {
                    continue;
                }
                if a.1 == *node && a.0 == *child {
                    continue;
                }
                if b.0 == *node && b.1 == *child {
                    continue;
                }
                if b.1 == *node && b.0 == *child {
                    continue;
                }
                if c.0 == *node && c.1 == *child {
                    continue;
                }
                if c.1 == *node && c.0 == *child {
                    continue;
                }
                colouring.insert(child.clone(), colour);
                queue.push(child.clone());
            }
        }
    }

    assert!(colouring.len() == pi.names.len());
    if colour != 2 {
        return None;
    }
    let mut ones = 0;
    let mut twos = 0;
    colouring.iter().for_each(|x| {
        if *x.1 == 1 {
            ones += 1;
        }
        if *x.1 == 2 {
            twos += 1;
        }
    });

    return Some(ones * twos);
}

fn main() {
    let s: String = fs::read_to_string("./src/input25.txt").unwrap();
    let pi: PuzzleInput = parse(s.trim());
    let ans = graphviz_cheat(&pi);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn test_part1_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim());
        let ans = part1(&pi);
        assert_eq!(54, ans);
    }
}
