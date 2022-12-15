use regex::Regex;
use std::cmp::{max, min};
use std::fs;

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    size: i32,
}

fn main() {
    let s: String = fs::read_to_string("./src/input15.txt").unwrap();
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
    let ss: Vec<Sensor> = s
        .trim_end()
        .split("\n")
        .map(|x| {
            let y: Vec<(i32, i32)> = re
                .captures_iter(x)
                .map(|cap| {
                    (
                        cap[1].parse::<i32>().unwrap(),
                        cap[2].parse::<i32>().unwrap(),
                    )
                })
                .collect();
            let s = y[0];
            let b = y[1];
            Sensor {
                pos: s,
                size: (s.1 - b.1).abs() + (s.0 - b.0).abs(),
            }
        })
        .collect();
    let part1 = solve1(&ss);
    dbg!(part1);
    let part2 = solve2(&ss, 4000000);
    dbg!(part2);
}

fn solve2(ss: &[Sensor], size: i32) -> i64 {
    // PS: This is actually kinda slow. not quite sure how I can optimize this.
    for i in 0..=size {
        let r = get_merge(ss, i);
        // PS: These awkward scoring implementation can be fixed if get_merge is fixed to merge [a,b] and [b+1, c] into [a,c]
        // Then it would just be taking the intersection with [0, size] and finding one with 2 ranges.
        let score = r
            .iter()
            .filter(|(a, b)| {
                if *b < 0 {
                    return false;
                }
                if *a > size {
                    return false;
                }
                return true;
            })
            .fold(0, |a, x| {
                let (left, right) = (max(x.0, 0), min(x.1, size));
                a + (right - left + 1)
            });
        
        if score == size { // Note that our row is size + 1 in length.
            // Find the gap
            let x= r.iter().fold(r[0].0, |a, x| {
                if x.0 > a {
                    a
                } else {
                    x.1
                }
            });
            return x as i64 *4000000 + i as i64;
        }
    }
    return 0
}

fn solve1(ss: &[Sensor]) -> i32 {
    return get_merge(ss, 2000000)
        .iter()
        .fold(0, |a, x| a + (x.1 - x.0));
}

fn get_merge(ss: &[Sensor], t: i32) -> Vec<(i32, i32)> {
    let mut ranges: Vec<(i32, i32)> = ss.iter().flat_map(|s| get_range(s, t)).collect();
    ranges.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    // dbg!(ranges);
    let mut merged = Vec::new();
    let last_range = ranges
        .into_iter()
        .reduce(|a, b| {
            // disjoint
            if a.1 < b.0 {
                merged.push(a);
                return b;
            } else if a.1 > b.1 {
                // a subsumes b
                return a;
            } else {
                // a overlaps with b
                return (a.0, b.1);
            }
        })
        .unwrap();
    merged.push(last_range);
    return merged;
}

fn get_range(s: &Sensor, t: i32) -> Option<(i32, i32)> {
    let (x, y) = s.pos;
    let r = s.size - (t - y).abs();
    if r < 0 {
        return None;
    }
    return Some((x - r, x + r));
}
