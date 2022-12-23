use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;

type Pos = (i64, i64);

fn parse_row(row: usize, line: &str) -> Vec<Pos> {
    let mut res = vec![];
    line.char_indices().for_each(|(col, c)| match c {
        '#' => res.push((col as i64, row as i64)),
        _ => (),
    });
    res
}

fn main() {
    let s: String = fs::read_to_string("./src/input23.txt").unwrap();
    let map: HashSet<Pos> = s
        .trim_end()
        .split("\n")
        .enumerate()
        .flat_map(|(x, y)| parse_row(x, y))
        .collect();

    let part1 = solve1(map.clone(), 10);
    dbg!(part1);
    let part2 = solve2(map);
    dbg!(part2);
}

fn solve1(lookup: HashSet<Pos>, rounds: i32) -> i64 {
    let mut start = lookup;
    for i in 0..rounds {
        (start, _) = next_round(&start, i);
    }
    score(&start)
}

#[allow(dead_code)]
fn solve2(curr_map: HashSet<Pos>) -> i32 {
    let mut curr_map = curr_map;
    let mut round = 0;
    loop {
        let (new_map, has_changed) = next_round(&curr_map, round);
        if !has_changed {
            return round + 1;
        }
        curr_map = new_map;
        round += 1;
    }
}

fn score(start: &HashSet<Pos>) -> i64 {
    let left_top = start
        .iter()
        .map(|a| (a.0, a.1))
        .reduce(|a, b| (min(a.0, b.0), min(a.1, b.1)))
        .unwrap();

    let right_bottom = start
        .iter()
        .map(|a| (a.0, a.1))
        .reduce(|a, b| (max(a.0, b.0), max(a.1, b.1)))
        .unwrap();
    (right_bottom.0 - left_top.0 + 1) * (right_bottom.1 - left_top.1 + 1) - start.len() as i64
}

fn next_round(curr_map: &HashSet<Pos>, i: i32) -> (HashSet<Pos>, bool) {
    // PS: added this thinking this will make comparing set for part 2 faster, but it makes no diff.
    // The bottleneck comes from simulating 1000s of rounds.
    let mut has_changed = false;
    // Map of where elves proposed to move to whether there's any confict seen and
    // the elf that made the proposal.
    let mut potential_new_cells: HashMap<(i64, i64), (bool, (i64, i64))> = HashMap::new();

    let mut new_map = HashSet::new();
    curr_map.iter().for_each(|pos| {
        // round Logic
        let n = neighbours(pos, curr_map);
        if n.len() == 0 {
            new_map.insert(*pos);
            return ();
        }
        // Propose stage and fix some non moving ones
        let new_pos = propose(*pos, i, &n);
        if potential_new_cells.contains_key(&new_pos) {
            // We know this won't move.
            new_map.insert(*pos);
            // We also know the original elf won't move.
            let (movable, first_elf) = potential_new_cells[&new_pos];
            if movable {
                new_map.insert(first_elf);
                potential_new_cells.insert(new_pos, (false, first_elf));
            }
        } else {
            potential_new_cells.insert(new_pos, (true, *pos));
        }
    });

    // Move the movable ones
    potential_new_cells.iter().for_each(|(k, (movable, _))| {
        if *movable {
            has_changed = true;
            new_map.insert((k.0, k.1));
        }
    });

    (new_map, has_changed)
}

fn neighbours(pos: &Pos, curr_map: &HashSet<Pos>) -> HashSet<Pos> {
    let (x, y) = pos;
    let mut res = HashSet::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let key = (x + i, y + j);
            if curr_map.contains(&key) {
                res.insert((x + i, y + j));
            }
        }
    }
    res
}

fn propose(pos: Pos, round: i32, neighbours: &HashSet<Pos>) -> Pos {
    for i in 0..4 {
        match (round + i) % 4 {
            0 => {
                // Check north
                if (-1..=1)
                    .map(|i| (pos.0 + i, pos.1 - 1))
                    .all(|key| !neighbours.contains(&key))
                {
                    return (pos.0, pos.1 - 1);
                }
            }

            1 => {
                // Check south
                if (-1..=1)
                    .map(|i| (pos.0 + i, pos.1 + 1))
                    .all(|key| !neighbours.contains(&key))
                {
                    return (pos.0, pos.1 + 1);
                }
            }
            2 => {
                // Check West
                if (-1..=1)
                    .map(|i| (pos.0 - 1, pos.1 + i))
                    .all(|key| !neighbours.contains(&key))
                {
                    return (pos.0 - 1, pos.1);
                }
            }
            3 => {
                // Check East
                if (-1..=1)
                    .map(|i| (pos.0 + 1, pos.1 + i))
                    .all(|key| !neighbours.contains(&key))
                {
                    return (pos.0 + 1, pos.1);
                }
            }
            _ => panic!(),
        }
    }
    return pos;
}

#[allow(dead_code)]
fn print_map(start: &HashSet<Pos>) -> () {
    println!("-----");
    let left_top = start
        .iter()
        .map(|a| (a.0, a.1))
        .reduce(|a, b| (min(a.0, b.0), min(a.1, b.1)))
        .unwrap();

    let right_bottom = start
        .iter()
        .map(|a| (a.0, a.1))
        .reduce(|a, b| (max(a.0, b.0), max(a.1, b.1)))
        .unwrap();

    for y in left_top.1..=right_bottom.1 {
        let mut row = String::new();
        for x in left_top.0..=right_bottom.0 {
            if start.contains(&(x, y)) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        println!("{}", row);
    }
    println!("-----");
}
