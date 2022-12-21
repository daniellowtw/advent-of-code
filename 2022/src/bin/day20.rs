use std::collections::{HashMap, HashSet};
use std::{fs};

#[derive(Debug, Clone)]
struct Node {
    prev: i32,
    next: i32,
    val: i64,
    id: i32, // To handle duplicates
}

fn main() {
    let s: String = fs::read_to_string("./src/input20.txt").unwrap();
    let nums: Vec<i64> = s
        .trim_end()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    let n = nums.len();
    let nodes: Vec<Node> = nums
        .iter()
        .enumerate()
        .map(|(id, x)| Node {
            prev: ((id + n - 1) % n) as i32,
            next: ((id + 1) % n) as i32,
            val: *x,
            id: id as i32,
        })
        .collect();

    let mut lookup = nodes
        .into_iter()
        .map(|x| (x.id, x))
        .collect::<HashMap<i32, Node>>();

    let part1 = solve1(&mut lookup);
    dbg!(part1);
    lookup.iter_mut().for_each(|(_, x)| {
        x.val *= 811589153;
    });

    let part2 = solve2(&mut lookup);
    dbg!(part2);
}

fn advance_right(node_idx: i32, steps: i64, lookup: &HashMap<i32, Node>) -> i32 {
    let mut target = &lookup[&node_idx];
    for _ in 0..steps {
        target = &lookup[&target.next];
    }
    return target.id;
}

fn remove(id: i32, lookup: &mut HashMap<i32, Node>) {
    let curr = &lookup[&id];
    let (prev, next) = (curr.prev, curr.next);
    lookup.get_mut(&prev).unwrap().next = next;
    lookup.get_mut(&next).unwrap().prev = prev;
}

fn insert(node_idx: i32, target_idx: i32, lookup: &mut HashMap<i32, Node>) -> () {
    let target = &lookup[&target_idx];
    let target_next_idx = target.next;
    let mut curr = lookup.get_mut(&node_idx).unwrap();
    curr.next = target_next_idx;
    curr.prev = target_idx;

    let mut target_next = lookup.get_mut(&target_next_idx).unwrap();
    target_next.prev = node_idx;

    let mut target = lookup.get_mut(&target_idx).unwrap();
    target.next = node_idx;
}

fn solve1(lookup: &mut HashMap<i32, Node>) -> i64 {
    let n_len = lookup.len();
    for i in 0..n_len {
        // println!("MOVING {}", node_idx);
        let node_idx = i as i32;
        let nn = &lookup[&node_idx];
        let node_val = nn.val;
        let mut x = node_val % (n_len - 1) as i64;
        while x < 0 {
            x += (n_len - 1) as i64;
        }
        let target_idx = advance_right(node_idx, x, lookup);
        if target_idx == node_idx {
            continue;
        }
        remove(node_idx, lookup);
        insert(node_idx, target_idx, lookup);
    }

    // dbg!(advance_right(8727, 1000, lookup));
    let idx = lookup.iter().find(|x| x.1.val == 0).unwrap().0;

    let a = lookup[&advance_right(*idx, 1000, lookup)].val;
    let b = lookup[&advance_right(*idx, 2000, lookup)].val;
    let c = lookup[&advance_right(*idx, 3000, lookup)].val;
    dbg!(a, b, c);
    a + b + c
}

fn solve2(lookup: &mut HashMap<i32, Node>) -> i64 {
    let n_len = lookup.len();
    for _ in 0..10 {
        for i in 0..n_len {
            // println!("MOVING {}", node_idx);
            let node_idx = i as i32;
            let nn = &lookup[&node_idx];
            let node_val = nn.val;
            let mut x = node_val % (n_len - 1) as i64;
            while x < 0 {
                x += (n_len - 1) as i64;
            }
            let target_idx = advance_right(node_idx, x, lookup);
            if target_idx == node_idx {
                continue;
            }
            remove(node_idx, lookup);
            insert(node_idx, target_idx, lookup);
        }
    }

    let mut res = vec![];
    let mut idx: i32 = lookup.iter().find(|x| x.1.val == 0).unwrap().0.clone();
    for _ in 0..3 {
        idx = advance_right(idx, 1000, lookup);
        res.push(lookup[&idx].val);
    }
    dbg!(&res);
    res.into_iter().sum()
}

fn valid(lookup: &HashMap<i32, Node>, start_idx: i32) -> bool {
    let size = lookup.len();
    let mut seen = HashSet::new();
    let mut start_idx = start_idx;
    let mut path = vec![start_idx];
    for _ in 0..size {
        start_idx = advance_right(start_idx, 1, lookup);
        path.push(start_idx);
        if seen.contains(&start_idx) {
            // dbg!(seen.len());
            return false;
        }
        seen.insert(start_idx);
        let n = &lookup[&start_idx];
        assert!(n.next < size as i32);
        assert!(n.next >= 0 as i32);
        assert!(n.prev >= 0 as i32);
        assert!(n.prev < size as i32);
    }
    assert!(seen.len() == size);
    return true;
}

fn trace(lookup: &HashMap<i32, Node>) -> () {
    let mut start_idx = 0;
    let mut path = vec![0];
    for _ in 0..lookup.len() {
        start_idx = advance_right(start_idx, 1, lookup);
        path.push(start_idx);
    }
    println!("{:?}", path);
}
