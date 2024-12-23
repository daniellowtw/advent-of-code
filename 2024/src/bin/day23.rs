use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part1(s: &str) -> i32 {
    let pi = parse(s);
    let mut sum = 0;
    let possible: HashSet<_> = pi.nodes.iter().filter(|x| x[0] == 't').collect();
    let mut groups: HashSet<[char; 6]> = HashSet::new();
    for n1 in possible {
        let edges = &pi.edges[n1];
        for n2 in edges.iter() {
            for n3 in pi.edges[n2].iter() {
                for n4 in pi.edges[n3].iter() {
                    if n4 == n1 {
                        let candidate1 = [n1[0], n1[1], n2[0], n2[1], n3[0], n3[1]];
                        let candidate2 = [n1[0], n1[1], n3[0], n3[1], n2[0], n2[1]];
                        let candidate3 = [n2[0], n2[1], n1[0], n1[1], n3[0], n3[1]];
                        let candidate4 = [n2[0], n2[1], n3[0], n3[1], n1[0], n1[1]];
                        let candidate5 = [n3[0], n3[1], n1[0], n1[1], n2[0], n2[1]];
                        let candidate6 = [n3[0], n3[1], n2[0], n2[1], n1[0], n1[1]];
                        if groups.contains(&candidate1)
                            || groups.contains(&candidate2)
                            || groups.contains(&candidate3)
                            || groups.contains(&candidate4)
                            || groups.contains(&candidate5)
                            || groups.contains(&candidate6)
                        {
                            continue;
                        }
                        groups.insert(candidate1);
                        groups.insert(candidate2);
                        groups.insert(candidate3);
                        groups.insert(candidate4);
                        groups.insert(candidate5);
                        groups.insert(candidate6);
                        sum += 1;
                        // println!("{:?} {:?} {:?} {:?}", n1, n2, n3, n4);
                    }
                }
            }
        }
    }
    sum
}

fn dfs(
    pi: &PuzzleInput,
    start: &[char; 2],
    curr_node: &[char; 2],
    visited: &mut HashSet<[char; 2]>,
) -> HashSet<[char; 2]> {
    visited.insert(*curr_node);
    // println!("{:?}", visited);
    let mut longest: HashSet<[char; 2]> = visited.clone();
    for n in pi.edges[curr_node].iter() {
        // ensure this candidate can reach all other existing nodes
        if !visited
            .iter()
            .filter(|n2| *n2 != n) // In case we found a cycle
            .all(|n2| pi.edges[n].contains(n2))
        {
            continue;
        }

        if n == start {
            // println!("found cycle of len {} -> {:?}", &visited.len(), visited);
            return longest;
        }

        if !visited.contains(n) {
            let candidate = dfs(pi, start, n, visited);
            if candidate.len() > longest.len() {
                longest = candidate;
            }
        }
    }
    longest
}

fn part2(s: &str) -> String {
    let pi = parse(s);

    let mut longest = HashSet::new();
    for i in &pi.nodes {
        let candidate = dfs(&pi, i, i, &mut HashSet::new());
        if candidate.len() > longest.len() {
            longest = candidate;
        }
    }
    let mut s: Vec<&[char; 2]> = longest.iter().collect();
    s.sort();
    let mut sb = format!("{}{}", s[0][0], s[0][1]);
    for i in s[1..].iter() {
        sb.push_str(&format!(",{}{}", i[0], i[1]));
    }
    sb.to_string()
}

struct PuzzleInput {
    edges: HashMap<[char; 2], HashSet<[char; 2]>>,
    nodes: HashSet<[char; 2]>,
}

fn parse(s: &str) -> PuzzleInput {
    let mut edges = HashMap::new();
    let mut nodes = HashSet::new();
    s.trim().lines().for_each(|l| {
        let (left, right) = l.split_once("-").unwrap();
        let left = [left.chars().next().unwrap(), left.chars().nth(1).unwrap()];
        let right = [right.chars().next().unwrap(), right.chars().nth(1).unwrap()];
        let le = edges.entry(left).or_insert(HashSet::new());
        le.insert(right);
        let re = edges.entry(right).or_insert(HashSet::new());
        re.insert(left);
        nodes.insert(left);
        nodes.insert(right);
    });
    PuzzleInput { edges, nodes }
}

fn main() {
    let s: String = fs::read_to_string("./input/23.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-23.txt").unwrap();
    println!("{}", part1(&s));
    println!("{}", part2(&s));
}
