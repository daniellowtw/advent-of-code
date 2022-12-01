use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Node<'a> {
    adjacent: Vec<&'a str>,
}

// The idea for this problem is a straight forward DFS.
fn main() {
    let s: String = fs::read_to_string("./src/input12.txt").unwrap();
    let ss: Vec<(&str, &str)> = s.split("\n").
        filter(|x| { !x.is_empty() }).
        map(|x| {
            let tmp = x.split("-").collect::<Vec<&str>>();
            (tmp[0], tmp[1])
        }).
        collect();
    let mut nodes: HashMap<&str, Node> = HashMap::new();
    for (a, b) in &ss {
        for x in vec![a, b] {
            nodes.entry(x).or_insert(Node {
                adjacent: vec![],
            });
        }
    }
    for (a, b) in &ss {
        // Originally I wanted the adjacent var to point to a Vec of &Nodes. But that meant I needed
        // to hold two mutable ref of nodes, and the compiler wasn't happy.
        // Hence, I decided for the nodes to hold just the string representing the node ID.
        let aa = &mut nodes.get_mut(a).unwrap();
        aa.adjacent.push(b);
        let bb = &mut nodes.get_mut(b).unwrap();
        bb.adjacent.push(a);
    }

    // Now run DFS
    let stack: Vec<&str> = Vec::new();
    // Part one. I had to modified this signature for part 2 to include a boolean to indicate
    // whether it should allow for a second occurrences of small caves.
    let score = dfs_helper("start", &nodes, stack, false);
    // Part two.
    // let score = dfs_helper("start", &nodes, stack, true);

    dbg!(score);
}

// Originally I wanted to inline this as a closure function in the main func. But I can't do
// recursive calls in a let binding.
fn dfs_helper<'a>(key: &'a str, nodes: &HashMap<&'a str, Node<'a>>, mut stack: Vec<&'a str>, accept_twos: bool) -> i32 {
    stack.push(key);
    if key == "end" {
        // Uncomment this to debug the paths taken.
        // dbg!(stack);
        return 1;
    }
    let mut local_score = 0;
    let node = &nodes.get(key).unwrap();
    node.adjacent.iter().for_each(|x| {
        if *x == "start" {
            return;
        }
        if is_large_cave(x) {
            local_score += dfs_helper(x, nodes, stack.clone(), accept_twos);
            return;
        }
        // We are in small cave.

        // Count number of times this value appears
        let freq_of_id = stack.iter().fold(0, |acc, y| {
            if x == y { acc + 1 } else { acc }
        });

        // We should stop the search in this branch
        if !accept_twos && freq_of_id > 0 {
            return;
        }

        let propagate = if freq_of_id == 1 { false } else { accept_twos };

        // Uncomment to debug steps.
        // dbg!(&stack);
        local_score += dfs_helper(x, nodes, stack.clone(), propagate);
    });
    return local_score;
}

fn is_large_cave(id: &str) -> bool {
    id.to_uppercase() == id
}

