#![allow(warnings)]
use std::{
    cmp::{max, Ordering},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug, Clone)]
struct PuzzleInput {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}

impl PuzzleInput {
    fn directional_neighbour(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < self.height - 1 {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < self.width - 1 {
            neighbours.push((x, y + 1));
        }
        return neighbours;
    }
}

fn parse(s: &str) -> PuzzleInput {
    let grid: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().chars().collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    let start = (
        0,
        grid.first()
            .unwrap()
            .iter()
            .enumerate()
            .find(|x| x.1 == &'.')
            .unwrap()
            .0,
    );
    let end = (
        height - 1,
        grid.last()
            .unwrap()
            .iter()
            .enumerate()
            .find(|x| x.1 == &'.')
            .unwrap()
            .0,
    );
    return PuzzleInput {
        grid,
        start,
        end,
        width,
        height,
    };
}

#[derive(PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    visited: HashSet<(usize, usize)>,
    score: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.score.cmp(&self.score); // min heap
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn explore(
    pi: &PuzzleInput,
    start: (usize, usize),
    end: HashSet<(usize, usize)>,
) -> Vec<((usize, usize), i32)> {
    // Use BFS to find the distance to possible junctions.
    let mut res = vec![];
    let mut queue = VecDeque::new();
    queue.push_back((start, HashSet::new(), 0));
    while !queue.is_empty() {
        let (node, mut seen, score) = queue.pop_front().unwrap();
        if end.contains(&node) && node != start {
            res.push((node, score));
            continue;
        }
        if !seen.insert(node) {
            continue;
        }
        for i in pi.directional_neighbour(node.0, node.1) {
            if pi.grid[i.0][i.1] == '#' {
                continue;
            }
            queue.push_back((i, seen.clone(), score + 1));
        }
    }
    return res;
}

// I got tired of typing this out and keep changing the type.
type T = HashMap<(usize, usize), HashSet<((usize, usize), i32)>>;

fn transform(pi: &PuzzleInput) -> T {
    let mut junctions = HashSet::new();
    for i in 0..pi.height {
        for j in 0..pi.width {
            if pi.grid[i][j] == '#' {
                continue;
            }
            let mut valid_neighbours = 0;
            for k in pi.directional_neighbour(i, j) {
                if pi.grid[k.0][k.1] != '#' {
                    valid_neighbours += 1;
                }
            }
            if valid_neighbours > 2 {
                junctions.insert((i, j));
            }
        }
    }
    junctions.insert(pi.start);
    junctions.insert(pi.end);

    let mut edges_map = T::new();

    for j in junctions.iter() {
        let edges = explore(pi, *j, junctions.clone());
        for e in edges {
            edges_map
                .entry(*j)
                .or_insert(HashSet::new())
                .insert((e.0, e.1));
            edges_map
                .entry(e.0)
                .or_insert(HashSet::new())
                .insert((*j, e.1));
        }
    }

    return edges_map;
}

fn find_all_path_dijkstra(new_map: &T, start: (usize, usize), end: (usize, usize)) -> i32 {
    // Dijkstra but inverse?
    let mut queue = BinaryHeap::new();
    let mut scores = vec![];
    let mut max_score = 0;
    let mut all_visited: HashSet<(Vec<(usize, usize)>, i32)> = HashSet::new();
    queue.push(State {
        pos: start,
        score: 0,
        visited: HashSet::new(),
    });
    // let mut seen = HashSet::new();
    while !queue.is_empty() {
        let s = queue.pop().unwrap();
        let (pos, score, mut seen) = (s.pos, s.score, s.visited);
        if !seen.insert(pos) {
            continue;
        }
        if pos == end {
            scores.push(score);
            continue;
        }

        let neighbours = new_map.get(&pos).unwrap();

        for (new_pos, i) in neighbours {
            queue.push(State {
                pos: *new_pos,
                score: score - i,
                visited: seen.clone(),
            });
        }
    }
    return -*scores.iter().min().unwrap();
}

fn find_all_path_dfs(
    new_map: &T,
    start: (usize, usize),
    end: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
) -> i32 {
    if start == end {
        return 0;
    }
    let mut best_score = 0;
    let neighbours = new_map.get(&start).unwrap();
    seen.insert(start);
    for (new_pos, i) in neighbours {
        if seen.contains(new_pos) {
            continue;
        }
        let new_score = find_all_path_dfs(new_map, *new_pos, end, seen);
        if best_score < i + new_score {
            best_score = i + new_score;
        }
    }
    seen.remove(&start);
    return best_score;
}

fn find_all_path(new_map: &T, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut queue = vec![];
    let mut scores = vec![];
    queue.push((start, 0, HashSet::new()));
    while !queue.is_empty() {
        let (pos, score, mut seen) = queue.pop().unwrap();
        if !seen.insert(pos) {
            continue;
        }
        if pos == end {
            scores.push(score);
            continue;
        }
        let neighbours = new_map.get(&pos).unwrap();
        for (new_pos, i) in neighbours {
            queue.push((*new_pos, score + i, seen.clone()));
        }
    }
    return scores.into_iter().max().unwrap();
}

fn dfs(pi: &PuzzleInput, start: (usize, usize)) -> i64 {
    // Originally I tried the recursive approach but I got stack overflow.
    let mut score = 0;
    let mut queue = vec![];
    queue.push((start, HashSet::new(), 0));

    let mut scores = vec![];

    while !queue.is_empty() {
        let (start, mut seen, score) = queue.pop().unwrap();
        if start == pi.end {
            scores.push(score);
            continue;
        }
        if !seen.insert(start) {
            continue;
        }

        match pi.grid[start.0][start.1] {
            '>' => {
                queue.push(((start.0, start.1 + 1), seen.clone(), score + 1));
            }
            '<' => {
                queue.push(((start.0, start.1 - 1), seen.clone(), score + 1));
            }
            '^' => {
                queue.push(((start.0 - 1, start.1), seen.clone(), score + 1));
            }
            'v' => {
                queue.push(((start.0 + 1, start.1), seen.clone(), score + 1));
            }
            _ => {
                for i in pi.directional_neighbour(start.0, start.1) {
                    if pi.grid[i.0][i.1] == '#' {
                        continue;
                    }
                    queue.push((i, seen.clone(), score + 1));
                }
            }
        }
    }
    // dbg!(&scores);
    return *scores.iter().max().unwrap();
}

fn part1(pi: &PuzzleInput) -> i64 {
    return dfs(pi, pi.start);
}

fn part2(pi: &PuzzleInput) -> i32 {
    // This reminds me of a problem in the past where the graph should be transformed to something with fewer nodes first.

    let new_map = transform(pi);
    // Looking at the input, there's only one way to the end node.
    // Move the target dest to the penultimate node so that when the program is there,
    // it doesn't try to explore another path which is definitely not the answer.

    let end = new_map.get(&pi.end).unwrap(); // Find the penultimate node
    assert!(
        end.len() == 1,
        "There should only be one path to the end node"
    );
    let (new_end, offset) = end.iter().next().unwrap();

    // This is the slowest at about 113s
    // return offset + find_all_path_dijkstra(&new_map, pi.start, *new_end);

    // This is at about 91s
    // return offset + find_all_path(&new_map, pi.start, *new_end);

    // Without the moving of the last node, the result is about double: at 65s.
    // return offset + find_all_path_dfs(&new_map, pi.start, pi.end, &mut HashSet::new());

    // This is the fastest at about 30s
    return offset + find_all_path_dfs(&new_map, pi.start, *new_end, &mut HashSet::new());
}

fn main() {
    let s: String = fs::read_to_string("./src/input23.txt").unwrap();
    let pi: PuzzleInput = parse(s.trim());
    // let ans = part1(&pi);
    // println!("{}", ans);
    let ans = part2(&pi);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[test]
    fn test_part1_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim());
        let ans = part1(&pi);
        println!("{}", ans);
    }

    #[test]
    fn test_part2_sample_input() {
        let mut pi = parse(SAMPLE_INPUT.trim());
        let ans = part2(&mut pi);
        println!("{}", ans);
    }
}
