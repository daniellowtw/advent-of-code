use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn cost(start: usize, end: usize, invalid: usize) -> Vec<String> {
    // Invalid to capture the empty positions so we don't even bother traversing.
    let mut cost_map = [999; 12];
    let mut queue: VecDeque<(usize, usize, Vec<char>)> = VecDeque::new();
    let mut all_possible_paths: Vec<Vec<char>> = Vec::new();
    queue.push_back((start, 0, Vec::new()));
    while let Some((r, score, mut path)) = queue.pop_front() {
        if r == invalid {
            continue;
        }
        if cost_map[r] < score {
            continue;
        } else {
            cost_map[r] = score;
        }
        if r == end {
            all_possible_paths.push(path);
            continue;
        }

        if r > 3 {
            path.push('v');
            queue.push_back((r - 3, score + 1, path.clone()));
            path.pop();
        }
        if r < 9 {
            path.push('^');
            queue.push_back((r + 3, score + 1, path.clone()));
            path.pop();
        }
        if r % 3 != 0 {
            path.push('<');
            queue.push_back((r - 1, score + 1, path.clone()));
            path.pop();
        }
        if r % 3 != 2 {
            path.push('>');
            queue.push_back((r + 1, score + 1, path.clone()));
            path.pop();
        }
    }
    // Observation: Zig zag is always worse. So filter them out to reduce search space.
    let filtered_to_only_2_dir: Vec<String> = all_possible_paths
        .iter()
        .filter(|&y| {
            y.iter()
                .fold((0, 'x'), |(score, curr), &x| {
                    if curr != x {
                        (score + 1, x)
                    } else {
                        (score, x)
                    }
                })
                .0
                <= 2
        })
        .map(|x| x.iter().collect())
        .collect();
    filtered_to_only_2_dir
}

fn min_seq_for_dkey_ending_with_a(
    curr_seq: String, // Must end with A
    depth: i32,
    cache: &mut HashMap<(String, i32), i64>,
) -> i64 {

    if depth == 0 {
        return curr_seq.len() as i64;
    }
    if let Some(v) = cache.get(&(curr_seq.clone(), depth)) {
        return *v;
    }

    let keypad = ['<', 'v', '>', 'X', '^', 'A'];
    let possible = find_ways_for_keypad(&keypad, &curr_seq, 5, 3);
    let min_sum = possible
        .iter()
        .map(|p| {
            let mut sum = 0;
            for part in break_up_string(&p) {
                let ans =
                    min_seq_for_dkey_ending_with_a(p[part.0..part.1].to_string(), depth - 1, cache);
                sum += ans;
            }
            sum
        })
        .min()
        .unwrap();
    cache.insert((curr_seq, depth), min_sum);
    min_sum
}

fn find_ways_for_keypad(
    keypad: &[char],
    curr_seq: &str,
    starting_pos: usize,
    invalid_pos: usize,
) -> Vec<String> {
    let mut all_possible_paths: Vec<String> = vec![String::new()];
    let mut curr_pos = starting_pos;
    let idx_map = keypad
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, &x)| {
            acc.insert(x, i);
            acc
        });
    for c in curr_seq.chars() {
        let next_dest = idx_map[&c];
        let paths = cost(curr_pos, next_dest, invalid_pos);
        // dbg!(&paths);
        let mut next_all_possible_paths = Vec::new();

        for path in all_possible_paths {
            for p in paths.iter() {
                let mut new_path = path.clone();
                new_path.push_str(&p);
                new_path.push_str("A");
                next_all_possible_paths.push(new_path);
            }
        }
        all_possible_paths = next_all_possible_paths;
        curr_pos = next_dest;
    }
    all_possible_paths
}

fn part1(pi: &Vec<&str>) -> i32 {
    let keypad = ['X', '0', 'A', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let keypad2 = ['<', 'v', '>', 'X', '^', 'A'];

    let mut score = 0;
    for cs in pi {
        let mut shortest_len = 99999;
        let mut door = find_ways_for_keypad(&keypad, cs, 2, 0);
        for _ in 0..2 {
            let mut next_door = Vec::new();
            for d in door.iter() {
                let door2 = find_ways_for_keypad(&keypad2, &d, 5, 3);
                next_door.extend(door2);
            }
            door = next_door;
        }
        for d3 in door.iter() {
            if d3.len() < shortest_len {
                shortest_len = d3.len();
            }
        }

        let x = cs[..cs.len() - 1].parse::<i32>().unwrap();
        // println!("{} -> {} * {}", cs, x, shortest_len);
        score += x * shortest_len as i32;
    }
    score
}

fn break_up_string(s: &str) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    let mut last_idx = 0;
    for (idx, c) in s.char_indices() {
        if c == 'A' {
            v.push((last_idx, idx + 1));
            last_idx = idx + 1;
        }
    }
    v
}

fn part2(pi: &Vec<&str>) -> i64 {
    let keypad = ['X', '0', 'A', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let depth = 25;

    let mut cache = HashMap::new();
    let mut score = 0;
    for cs in pi {
        let ways_for_door = find_ways_for_keypad(&keypad, cs, 2, 0);
        let shortest_len = ways_for_door
            .iter()
            .map(|x| {
                let parts = break_up_string(&x);
                let mut sum = 0;
                for (start, end) in parts {
                    let ans = min_seq_for_dkey_ending_with_a(
                        x[start..end].to_string(),
                        depth,
                        &mut cache,
                    );
                    sum += ans;
                }
                // println!("{} -> {}", &x, sum);
                sum
            })
            .min()
            .unwrap();
        let x = cs[..cs.len() - 1].parse::<i64>().unwrap();
        // println!("{} -> {} * {}", cs, x, shortest_len);
        score += x * shortest_len;
    }
    score
}

fn main() {
    let s: String = fs::read_to_string("./input/21.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-21.txt").unwrap();
    let ss: Vec<&str> = s.lines().collect();
    // let keypad = ['X', '0', 'A', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    // dbg!(f(&keypad, "029A", 2, 0));
    // dbg!(cost(1, 8,0));
    println!("{}", part1(&ss));
    println!("{}", part2(&ss));
}
