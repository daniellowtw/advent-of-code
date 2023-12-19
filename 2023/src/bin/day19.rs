use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Rule {
    Goto(String),
    Less(char, i32, String),
    More(char, i32, String),
}

#[derive(Debug)]
struct PuzzleInput {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<(i32, i32, i32, i32)>,
}

fn parse(s: String) -> PuzzleInput {
    let ss: Vec<&str> = s.split("\n\n").collect();
    let workflows = ss[0]
        .split("\n")
        .map(|l| {
            let parts: Vec<&str> = l.split("{").collect();
            let name = parts[0].trim().to_string();
            let rules = parts[1][..parts[1].len() - 1]
                .split(",")
                .map(|r| {
                    if r.contains(':') {
                        let x: Vec<&str> = r.split(":").collect();
                        let rule: Vec<char> = x[0].trim().chars().collect();
                        let val = x[0][2..].parse::<i32>().unwrap();
                        match rule[1] {
                            '<' => return Rule::Less(rule[0], val, x[1].trim().to_string()),
                            '>' => return Rule::More(rule[0], val, x[1].trim().to_string()),
                            _ => {
                                panic!("Unknown rule {}", x[0])
                            }
                        }
                    } else {
                        Rule::Goto(r.trim().to_string())
                    }
                })
                .collect();
            return (name, rules);
        })
        .collect();
    let parts = ss[1]
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let parts: Vec<i32> = x[1..x.len() - 1]
                .split(",")
                .map(|a| a[2..].parse::<i32>().unwrap())
                .collect();
            (parts[0], parts[1], parts[2], parts[3])
        })
        .collect();
    return PuzzleInput { workflows, parts };
}

fn part2(pi: &PuzzleInput) -> i64 {
    let candidates = vec![vec![(1, 4000)]; 4];
    let ans = run_workflow_2(&candidates, "in", pi).unwrap();
    return ans;
}

fn score(candidates: &Vec<Vec<(i32, i32)>>) -> i64 {
    let mut ans = 1;
    for c in candidates {
        for r in c {
            ans *= r.1 as i64 - r.0 as i64 + 1;
        }
    }
    return ans;
}

fn run_workflow_2(candidates: &Vec<Vec<(i32, i32)>>, node: &str, pi: &PuzzleInput) -> Option<i64> {
    // Do a postorder tree traversal
    if node == "R" {
        return None;
    }
    if node == "A" {
        return Some(score(candidates));
    }

    let mut candidates = candidates.clone();
    let rules = &pi.workflows[node];
    let mut res = Vec::new();
    for r in rules {
        match r {
            Rule::Goto(s) => {
                if let Some(ans) = run_workflow_2(&candidates, s, pi) {
                    res.push(ans);
                }
            }
            Rule::Less(c, v, s) => {
                let candidate = match c {
                    'x' => &candidates[0],
                    'm' => &candidates[1],
                    'a' => &candidates[2],
                    's' => &candidates[3],
                    _ => {
                        panic!("Unknown rule {}", c)
                    }
                };
                // Split the candidate into two ranges
                let less_group = candidate
                    .into_iter()
                    .filter_map(|range| {
                        if range.1 < *v {
                            return Some(*range);
                        }
                        if range.0 >= *v {
                            return None;
                        }
                        return Some((range.0, *v - 1));
                    })
                    .collect();
                let more_group = candidate
                    .into_iter()
                    .filter_map(|range| {
                        if range.0 >= *v {
                            return Some(*range);
                        }
                        if range.1 < *v {
                            return None;
                        }
                        return Some((*v, range.1));
                    })
                    .collect();

                let mut new_candidates = candidates.clone();
                match c {
                    'x' => new_candidates[0] = less_group,
                    'm' => new_candidates[1] = less_group,
                    'a' => new_candidates[2] = less_group,
                    's' => new_candidates[3] = less_group,
                    _ => {
                        panic!("Unknown rule {}", c)
                    }
                }
                if let Some(ans) = run_workflow_2(&new_candidates, s, pi) {
                    res.push(ans);
                }

                match c {
                    'x' => candidates[0] = more_group,
                    'm' => candidates[1] = more_group,
                    'a' => candidates[2] = more_group,
                    's' => candidates[3] = more_group,
                    _ => {
                        panic!("Unknown rule {}", c)
                    }
                }
            }
            Rule::More(c, v, s) => {
                let candidate = match c {
                    'x' => &candidates[0],
                    'm' => &candidates[1],
                    'a' => &candidates[2],
                    's' => &candidates[3],
                    _ => {
                        panic!("Unknown rule {}", c)
                    }
                };
                // Split the candidate into two ranges
                let less_group = candidate
                    .into_iter()
                    .filter_map(|range| {
                        if range.1 <= *v {
                            return Some(*range);
                        }
                        if range.0 > *v {
                            return None;
                        }
                        return Some((range.0, *v));
                    })
                    .collect();
                let more_group = candidate
                    .into_iter()
                    .filter_map(|range| {
                        if range.0 > *v {
                            return Some(*range);
                        }
                        if range.1 <= *v {
                            return None;
                        }
                        return Some((*v + 1, range.1));
                    })
                    .collect();

                let mut new_candidates = candidates.clone();
                match c {
                    'x' => new_candidates[0] = more_group,
                    'm' => new_candidates[1] = more_group,
                    'a' => new_candidates[2] = more_group,
                    's' => new_candidates[3] = more_group,
                    _ => {
                        panic!("Unknown rule {}", c)
                    }
                }
                if let Some(ans) = run_workflow_2(&new_candidates, s, pi) {
                    res.push(ans);
                }

                match c {
                    'x' => candidates[0] = less_group,
                    'm' => candidates[1] = less_group,
                    'a' => candidates[2] = less_group,
                    's' => candidates[3] = less_group,
                    _ => {
                        panic!("Unknown rule {}", c)
                    }
                }
            }
        }
    }

    let ans = res.iter().fold(0, |acc, b| acc + b);
    return Some(ans);
}

fn run_workflow(p: &(i32, i32, i32, i32), pi: &PuzzleInput) -> bool {
    let mut rules = &pi.workflows["in"];
    loop {
        for r in rules {
            match r {
                Rule::Goto(s) => {
                    if s == "R" {
                        return false;
                    }
                    if s == "A" {
                        return true;
                    }

                    rules = &pi.workflows[s];
                    break;
                }
                Rule::Less(c, v, s) => {
                    let candidate = match c {
                        'x' => p.0,
                        'm' => p.1,
                        'a' => p.2,
                        's' => p.3,
                        _ => {
                            panic!("Unknown rule {}", c)
                        }
                    };
                    if candidate < *v {
                        if s == "R" {
                            return false;
                        }
                        if s == "A" {
                            return true;
                        }
                        rules = &pi.workflows[s];
                        break;
                    }
                }
                Rule::More(c, v, s) => {
                    let candidate = match c {
                        'x' => p.0,
                        'm' => p.1,
                        'a' => p.2,
                        's' => p.3,
                        _ => {
                            panic!("Unknown rule {}", c)
                        }
                    };
                    if candidate > *v {
                        if s == "R" {
                            return false;
                        }
                        if s == "A" {
                            return true;
                        }
                        rules = &pi.workflows[s];
                        break;
                    }
                }
            }
        }
    }
}

fn part1(pi: &PuzzleInput) -> i32 {
    let mut ans = 0;

    for p in &pi.parts {
        if run_workflow(p, pi) {
            ans += p.0 + p.1 + p.2 + p.3;
        }
    }
    return ans;
}

fn main() {
    let s: String = fs::read_to_string("./src/input19.txt").unwrap();
    let pi = parse(s.trim().to_string());
    let ans: i32 = part1(&pi);
    println!("{}", ans);
    let ans = part2(&pi);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_part1_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim().to_string());
        // println!("{:?}", pi);
        let ans = part1(&pi);
        println!("{}", ans);
    }

    #[test]
    fn test_part3_sample_input() {
        const s: &str = "
in{s<1001:A}

{x=787,m=2655,a=1222,s=2876}
";
        let pi = parse(s.trim().to_string());
        // println!("{:?}", pi);
        let ans = part2(&pi);
        println!("{}", ans);
    }
    #[test]
    fn test_part2_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim().to_string());
        let ans = part2(&pi);
        println!("{}", ans);
    }
}
