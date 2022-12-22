use regex::{CaptureMatches, Regex};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    flow: i32,
    neighbours: Vec<String>,
}

fn main() {
    let s: String = fs::read_to_string("./src/input16.txt").unwrap();
    let re = Regex::new(r"Valve ([A-Z]+).*?rate=(\d+);.*?valve[s]? (.*?)$").unwrap();
    let original_nodes: HashMap<String, Node> = s
        .trim_end()
        .split("\n")
        .map(|x| {
            let mut y: CaptureMatches = re.captures_iter(x);
            let cap = y.nth(0);
            let cap = cap.unwrap();
            let name: String = (&cap)[1].to_string();
            (
                name.clone(),
                Node {
                    name: name,
                    flow: cap[2].parse::<i32>().unwrap(),
                    neighbours: cap[3].split(",").map(|x| x.trim().to_string()).collect(),
                },
            )
        })
        .collect();
    let reduced_nodes: HashMap<String, Node> = (original_nodes.clone())
        .into_iter()
        .filter(|(name, node)| node.flow > 0 || *name == "AA")
        .collect();
    let node_keys: HashSet<String> = reduced_nodes.iter().map(|(x, _)| x.to_string()).collect();
    let nodes_to_others: HashMap<String, HashMap<String, i32>> = (&reduced_nodes)
        .into_iter()
        .map(|(name, node)| (name.clone(), shortest_path(&original_nodes, &node)))
        .collect();

    assert!(valid(&nodes_to_others, &node_keys.iter().collect()));

    // let part1 = solve_part_1(&nodes, &node_keys, &nodes_to_others);
    // dbg!(part1);
    let state = State {
        nodes: reduced_nodes,
        node_keys,
        nodes_to_others,
    };
    let part2 = solve_part_2(&state);
    dbg!(part2);
    // let part2 = solve2(&ss);
    // dbg!(part2);
}

fn valid(
    nodes_to_others: &HashMap<String, HashMap<String, i32>>,
    node_keys: &Vec<&String>,
) -> bool {
    for i in 0..node_keys.len() {
        for j in 0..node_keys.len() {
            let name = node_keys[i];
            let name2 = node_keys[j];
            assert!(nodes_to_others[name][name2] == nodes_to_others[name2][name]);
        }
    }
    return true;
}

struct State {
    nodes: HashMap<String, Node>,
    node_keys: HashSet<String>,
    nodes_to_others: HashMap<String, HashMap<String, i32>>,
}

// fn solve_part_1(
//     nodes: &HashMap<&String, &Node>,
//     node_keys: &HashSet<&String>,
//     nodes_to_others: &HashMap<&String, HashMap<String, i32>>,
// ) -> i32 {
//     // Idea: create a reduced graph with only the nodes that have flow > 0
//     // Then, do a DFS from AA, and keep track of the score
//     // The score is the sum of the flow * the depth of the node

//     let mut history: Vec<String> = vec![String::from("AA")];
//     let (score, _) = dfs(
//         &nodes,
//         node_keys,
//         &nodes_to_others,
//         String::from("AA"),
//         &mut history,
//         30,
//     );
//     return score;
// }

fn solve_part_2(state: &State) -> i32 {
    // The idea is to do part1, and then when we reach the end, we go back to AA, and start another DFS
    // PS: totally rubbish performance. 7m30s to run.
    let mut history: Vec<String> = vec![];
    // Checking scoring function works.
    // let t = score2(state, &"DD,HH,EE,AA,JJ,BB,CC".split(",").map(|x| x.to_string()).collect(), 26);
    dfs2(state, String::from("AA"), &mut history, 26)
}

// fn dfs(
//     state: &State,
//     start: String,
//     history: &mut Vec<String>,
//     time_left: i32,
// ) -> (i32, Vec<String>) {
//     if time_left <= 0 {
//         return (score2(state, history, 30), history.clone());
//     }
//     let mut score = 0;
//     let mut flag = false;
//     let mut best: Vec<String> = vec![];
//     state.nodes.into_iter().for_each(|&n| {
//         if !history.contains(n) {
//             flag = true;
//             history.push(n.clone());
//             let cost = nodes_to_others[&start][n];
//             let (candidate_score, candidate_history) = dfs(
//                 ss,
//                 nodes,
//                 nodes_to_others,
//                 (*n).clone(),
//                 history,
//                 time_left - cost - 1,
//             );
//             if candidate_score > score {
//                 score = candidate_score;
//                 best = candidate_history.clone();
//             }
//             history.pop();
//         }
//     });
//     if !flag {
//         return (score2(ss, history, nodes_to_others, 30), history.clone());
//     }
//     return (score, best);
// }

fn score2(state: &State, history: &Vec<String>, max_time: i32) -> i32 {
    let mut curr = &String::from("AA");
    let mut score = 0;
    let mut time = 0;

    for i in 0..history.len() {
        let next = &history[i];

        if next == "AA" {
            score += score2(state, &history[i + 1..].to_vec(), max_time);
            break;
        }

        let cost = state.nodes_to_others[curr][next];
        // hist.push((next, cost));
        time += cost + 1;
        if time >= max_time {
            curr = next;
            continue;
        }
        let f = &state.nodes[next].flow;
        score += f * (max_time - time);
        curr = next;
    }
    return score;
}

fn dfs2(state: &State, start: String, history: &mut Vec<String>, time_left: i32) -> i32 {
    // Base case
    let aa: String = String::from("AA");
    if time_left <= 1 {
        if history.contains(&aa) {
            return score2(state, &history, 26);
        } else {
            let mut hist2 = history.clone();
            hist2.push(aa.to_string());
            return dfs2(state, aa.to_string(), &mut hist2, 26);
        }
    }

    // Recursive case
    let mut score = 0;
    let mut flag = false;
    // let mut best: Vec<String> = vec![];
    state.node_keys.iter().for_each(|n| {
        // Handle "AA" separately since it means to "restart and go back to AA"
        if n == &aa {
            ()
        } else if !history.contains(n) {
            flag = true;
            history.push(n.to_string());
            if history.len() == 1 {
                dbg!(&history[0]);
            }
            let cost = state.nodes_to_others[&start][n];
            if time_left - cost - 1 >= 1 {
                let candidate_score = dfs2(state, n.to_string(), history, time_left - cost - 1);
                if candidate_score > score {
                    score = candidate_score;
                }
            }
            history.pop();
        }
    }); // Adding the length condition shaves off 5 minutes. But how do I know this will not miss out optimal solution?
    if !history.contains(&aa) && history.len() > 5 {
        flag = true;
        // Also consider stopping here.
        let mut hist2 = history.clone();
        hist2.push(aa.to_string());
        let candidate_score = dfs2(state, aa.to_string(), &mut hist2, 26);
        if candidate_score > score {
            score = candidate_score;
        }
    }
    if !flag {
        return score2(state, &history, 26);
    }
    return score;
}

fn shortest_path(nodes: &HashMap<String, Node>, start: &Node) -> HashMap<String, i32> {
    let mut distances: HashMap<String, i32> = HashMap::new();
    let mut queue: Vec<String> = vec![];
    queue.push(start.name.clone());
    distances.insert(start.name.clone(), 0);
    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        let curr_node = &nodes[&curr];
        for n in curr_node.neighbours.iter() {
            if !distances.contains_key(n) {
                distances.insert(n.clone(), distances[&curr] + 1);
                queue.push(n.clone());
            } else {
                if distances[n] > distances[&curr] + 1 {
                    distances.insert(n.clone(), distances[&curr] + 1);
                    queue.push(n.clone());
                }
            }
        }
    }
    return distances;
}
