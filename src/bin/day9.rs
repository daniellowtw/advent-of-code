use std::fs;
use std::collections::{HashMap, HashSet};
use std::detect::__is_feature_detected::xsave;
use std::hash::Hash;
use itertools::Itertools;

fn main() {
    let s: String = fs::read_to_string("./src/input9.txt").unwrap();
    let ss: Vec<Vec<i32>> = s.split("\n").
        filter(|x| { !x.is_empty() }).
        map(|l| l
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>()).
        collect(); // collect();
    let num_rows = ss.len() as i32;
    let num_cols = ss.get(0).unwrap().len() as i32;
    dbg!(num_rows, num_cols); // Just some sanity checks.

    // For part one
    let mut score = 0;

    // For part 2. Let's also keep track of the colour of the basins
    // This is a map of a point to its colour. Every low point is it's own colour.
    let mut basins: Vec<Vec<Option<i32>>> = Vec::new();
    for _ in 0..num_rows {
        let mut r = Vec::with_capacity(num_cols as usize);
        r.resize(num_cols as usize, None);
        basins.push(r);
    }
    // This keeps track of the colour and the number of points of that colour
    let mut colour_score: HashMap<i32, i32> = HashMap::new();
    // Arbitrarily choose a colour.
    let mut colour = 101;


    // Part 1: Lowest point is where the neighbours are higher.
    // Part 2: Here we initialize the lowest point with a colour.
    for i in 0..num_rows {
        for j in 0..num_cols {
            let v = ss.get(i as usize).unwrap().get(j as usize).unwrap();
            let candidates = neighbours(num_rows as usize, num_cols as usize, i as usize, j as usize);
            let lowest = candidates.iter()
                .map(|(i, j)| {
                    ss.get(*i).unwrap().get(*j).unwrap()
                })
                .all(|x1| x1 > v);
            if lowest {
                // dbg!(i, j, v);
                score += 1 + v;
                // low_points.push((i, j));
                *basins.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = Some(colour);
                colour_score.entry(colour).or_insert(1);
                colour += 1;
            }
        }
    }
    dbg!(score);

    // Part 2: After seeing that our input is quite small. I decided to use a rather inefficient algo.
    // In each iteration, we expand the neighbours of the basin. As long as we've extended once,
    // We make sure to do it again in case the new neighbours might also need to be coloured.
    let mut has_work = true;
    while has_work {
        has_work = false;
        for i in 0..num_rows {
            for j in 0..num_cols {
                let center_val = ss.get(i as usize).unwrap().get(j as usize).unwrap();
                if *center_val == 9 { continue; };
                // if it's already coloured. Skip.
                if basins.get(i as usize).unwrap().get(j as usize).unwrap().is_some() {
                    continue;
                };
                let candidates = neighbours(num_rows as usize, num_cols as usize, i as usize, j as usize);
                let pos = candidates.iter()
                    // Only consider neighbour that has a lower value.
                    .filter(|(i, j)| {
                        let neighbour_val = ss.get(*i).unwrap().get(*j).unwrap();
                        center_val > neighbour_val
                    })
                    .map(|(i, j)| {
                        let v = ss.get(*i).unwrap().get(*j).unwrap();
                        ((*i, *j), v)
                    })
                    // Here we might get tie breaker but we assume input is well defined that this
                    // doesn't happen. For example, if we have a board [[7,8],[8,7]] it
                    // becomes ambiguous which basin do the eights belong to.
                    .reduce(|x2, x3| if x2.1 < x3.1 { x2 } else { x3 });
                // If no result means this is the
                if pos.is_none() {
                    continue;
                }
                let pos = pos.unwrap().0;
                let basin_colour = basins.get(pos.0 as usize).unwrap().get(pos.1 as usize).unwrap();
                if basin_colour.is_some() {
                    let c = basin_colour.unwrap();
                    *basins.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = Some(c);
                    colour_score.entry(c).and_modify(|x| { *x += 1 });
                    has_work = true;
                }
            }
        }
    }

    let mut x: Vec<(&i32, &i32)> = colour_score.iter().collect();
    x.sort_by(|x1, x2| { x2.1.cmp(x1.1) });
    let y = &x.as_slice()[..3].iter().fold(1, |b, x4| b * x4.1); // get top 3
    dbg!(y);
}

fn neighbours(num_rows: usize, num_cols: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let vec1: Vec<i32> = vec![-1, 1];
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for k in vec1.clone() {
        let newi = i as i32 + k;
        if newi >= 0 && newi < num_rows as i32 {
            candidates.push((newi as usize, j))
        }
        let newj = j as i32 + k;
        if newj >= 0 && newj < num_cols as i32 {
            candidates.push((i, newj as usize))
        }
    }
    candidates
}
