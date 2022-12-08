use std::{collections::HashMap, fs};

enum ACTIONS {
    RETURN,
    CONTINUE,
    STOP,
}

#[derive(Debug, Clone)]
struct Node {
    // parent: Option<&'a Node<'a>>,
    children: HashMap<String, Node>,
    total: i64,
}
impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            total: 0,
        }
    }

    fn consume(&self, iter: &mut std::slice::Iter<'_, Command>) -> (Node, ACTIONS) {
        let mut total = self.total;
        let mut children = self.children.clone();
        while let Some(cmd) = (iter).next() {
            match cmd {
                Command::Cd(dest) => {
                    let node = children.entry(dest[..].to_string()).or_insert(Node::new());
                    total -= node.total;
                    let (node, action) = node.consume(iter);
                    total += node.total;
                    children.insert(dest[..].to_string(), node);
                    match action {
                        ACTIONS::CONTINUE => continue,
                        x => {
                            return (
                                Node {
                                    total: total,
                                    children: children,
                                },
                                x,
                            )
                        }
                    }
                }
                Command::CdUp => {
                    return (
                        Node {
                            children: children,
                            total: total,
                        },
                        ACTIONS::CONTINUE,
                    )
                }
                Command::CdRoot => {
                    return (
                        Node {
                            children: children,
                            total: total,
                        },
                        ACTIONS::RETURN,
                    )
                }
                Command::Ls(i) => {
                    total += i;
                }
            }
        }
        return (
            Node {
                children: children,
                total: total,
            },
            ACTIONS::STOP,
        );
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    CdUp,
    CdRoot,
    Ls(i64),
}

fn tokenize(ss: Vec<&str>) -> Vec<Command> {
    let mut res: (Vec<Command>, Option<i64>) =
        ss.iter().fold((vec![], None), |(mut cmds, lss), s| {
            if s.starts_with("$ cd") {
                if let Some(i) = lss {
                    cmds.push(Command::Ls(i));
                }
                let dest = s.split(" ").last().unwrap();
                match dest {
                    ".." => cmds.push(Command::CdUp),
                    "/" => cmds.push(Command::CdRoot),
                    _ => cmds.push(Command::Cd(dest.to_string())),
                }
                return (cmds, None);
            } else if s == &"$ ls" {
                return (cmds, Some(0));
            } else if !s.starts_with("$") && !s.starts_with("dir") {
                let (size, _) = s.split_once(" ").unwrap();
                if let Some(x) = lss {
                    return (cmds, Some(x + size.parse::<i64>().unwrap()));
                }
                return (cmds, Some(size.parse::<i64>().unwrap()));
            } else {
                return (cmds, lss);
                // panic!("Unknown command: {}", s)
            }
        });
    res.0.push(Command::Ls(res.1.unwrap()));
    return res.0;
}

fn main() {
    let s: String = fs::read_to_string("./src/input7.txt").unwrap();
    let ss: Vec<&str> = s.trim_end().split("\n").collect();
    let cmds = tokenize(ss);
    // dbg!(&cmds);
    let mut i = cmds.iter();
    let mut root = Node {
        children: HashMap::new(),
        total: 0,
    };
    let mut action = ACTIONS::CONTINUE;
    loop {
        match action {
            ACTIONS::CONTINUE => {
                (root, action) = root.consume(&mut i);
            }
            ACTIONS::RETURN => {
                (root, action) = root.consume(&mut i);
            }
            ACTIONS::STOP => {
                break;
            }
        }
    }
    let score = calculate_part1(&root);
    // dbg!(&root);
    println!("Part 1: {}", score);
    let score = calculate_part2(&root, 30000000 - 70000000 + &root.total, None);
    println!("Part 2: {}", score.unwrap());
}

fn calculate_part1(root: &Node) -> i64 {
    let mut score = 0;
    if root.total <= 100000 {
        score += root.total;
    }
    score += root.children.values().map(calculate_part1).sum::<i64>();
    return score;
}

fn calculate_part2(root: &Node, target: i64, current: Option<i64>) -> Option<i64> {
    let smallest_from_children = root
        .children
        .values()
        .map(|x| calculate_part2(x, target, current))
        .flatten()
        .min();
    match smallest_from_children {
        Some(x) => return Some(x),
        None => {
            if let Some(current_score) = current {
                if root.total >= target && root.total < current_score {
                    return Some(root.total);
                } else {
                    return current;
                }
            } else {
                if root.total >= target {
                    return Some(root.total);
                } else {
                    return None;
                }
            }
        }
    }
}
