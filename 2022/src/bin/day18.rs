use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs, vec};

fn main() {
    let s: String = fs::read_to_string("./src/input18.txt").unwrap();
    let mut bound_min = (i32::MAX, i32::MAX, i32::MAX);
    let mut bound_max = (i32::MIN, i32::MIN, i32::MIN);
    let dice: Vec<(i32, i32, i32)> = s
        .trim_end()
        .split("\n")
        .map(|x| {
            let tmp: Vec<i32> = x.split(",").map(|x| x.parse().unwrap()).collect();
            let (x, y, z) = (tmp[0], tmp[1], tmp[2]);
            bound_min.0 = min(x, bound_min.0);
            bound_min.1 = min(y, bound_min.1);
            bound_min.2 = min(z, bound_min.2);
            bound_max.0 = max(x, bound_max.0);
            bound_max.1 = max(y, bound_max.1);
            bound_max.2 = max(z, bound_max.2);
            (tmp[0], tmp[1], tmp[2])
        })
        .collect();

    // dbg!(bound_min, bound_max);
    let part1 = solve1(&dice);
    dbg!(part1);
    let part2 = solve2(&dice, &bound_min, &bound_max);
    dbg!(part2);
}

type Point = (i32, i32, i32);

fn solve2(dice: &Vec<Point>, bound_min: &(i32, i32, i32), bound_max: &(i32, i32, i32)) -> i32 {
    // PS: Super sloppy code. The idea is to add a boundary of non points around the bound and
    // then do a union find on the points starting with the corner. This will find all reachable
    // points from the corner. Then, we can invert to get a list of points that form our lava and
    // reuse part 1.
    let mut space = Vec::new();
    for _ in bound_min.0..bound_max.0 + 2 {
        let mut tmp2 = Vec::new();
        for _ in bound_min.1..bound_max.1 + 2 {
            let mut tmp = Vec::new();
            for _ in bound_min.2..bound_max.2 + 2 {
                tmp.push(true);
            }
            tmp2.push(tmp);
        }
        space.push(tmp2);
    }

    // Translate points so that bound_min is (1,1,1)
    let dice: HashSet<Point> = dice
        .iter()
        .map(|x| {
            (
                x.0 - bound_min.0 + 1,
                x.1 - bound_min.1 + 1,
                x.2 - bound_min.2 + 1,
            )
        })
        .collect();

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((0, 0, 0));
    while queue.len() > 0 {
        let i = queue.pop_back().unwrap();
        let (x, y, z) = i;
        space[x as usize][y as usize][z as usize] = false;
        for j in neighbours(&(x as i32, y as i32, z as i32)) {
            if !valid(&j, &space) {
                continue;
            }
            if dice.contains(&j) {
                continue;
            }
            if !space[j.0 as usize][j.1 as usize][j.2 as usize] {
                continue;
            }
            queue.push_back((j.0 as usize, j.1 as usize, j.2 as usize));
        }
    }

    // We now have an inverted space. Reuse part 1
    let mut cubes: Vec<Point> = vec![];
    for i in 0..space.len() {
        for j in 0..space[i].len() {
            for k in 0..space[i][j].len() {
                if space[i][j][k] {
                    cubes.push((i as i32, j as i32, k as i32));
                }
            }
        }
    }

    solve1(&cubes)
}

fn valid(j: &(i32, i32, i32), space: &Vec<Vec<Vec<bool>>>) -> bool {
    // dbg!(j, space.len(), space[0].len(), space[0][0].len());
    if j.0 < 0 || j.0 >= space.len() as i32 {
        return false;
    }
    if j.1 < 0|| j.1 >= space[0].len() as i32 {
        return false;
    }
    if j.2 < 0 || j.2 >= space[0][0].len() as i32 {
        return false;
    }
    return true;
}

fn solve1(dice: &[(i32, i32, i32)]) -> i32 {
    let mut lookup: HashMap<(i32, i32, i32), i32> = dice.into_iter().map(|x| (*x, 6)).collect();
    let mut update: Vec<((i32, i32, i32), i32)> = vec![];
    for (i, _) in &lookup {
        let mut count = 0;
        for j in neighbours(i) {
            if (lookup).contains_key(&j) {
                count += 1;
            }
        }
        update.push((*i, count));
    }
    for (k, v) in update {
        let y = lookup.get_mut(&k).unwrap();
        *y -= v;
    }
    let mut score = 0;
    for (_, v) in lookup {
        score += v;
    }
    return score;
}

fn neighbours(x: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut res = vec![];
    let (a, b, c) = *x;
    res.push((a - 1, b, c));
    res.push((a + 1, b, c));
    res.push((a, b - 1, c));
    res.push((a, b + 1, c));
    res.push((a, b, c - 1));
    res.push((a, b, c + 1));
    return res;
}
