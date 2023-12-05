use std::cmp::min;
use std::fs;

type SingleMapStage = Vec<((i64, i64), i64)>;
// This encodes the mapping that needs to happen for a stage.

#[derive(Debug)]
struct PuzzleInput {
    seeds: Vec<i64>,
    // Needed for part 2
    seed_ranges: Vec<(i64, i64)>,
    maps: Vec<SingleMapStage>,
}

fn parse_single_map(s: &str) -> SingleMapStage {
    let mut tmp = s.trim().split("\n");
    tmp.next(); // Skip first line
    let res = tmp
        .map(|x| {
            let parts: Vec<i64> = x.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            ((parts[1], parts[1] + parts[2]), parts[0])
        })
        .collect();
    return res;
}

fn parse(s: String) -> PuzzleInput {
    let mut parts = s.split("\n\n");
    let mut iter = parts.nth(0).unwrap().trim().split(" ");
    iter.next(); // First one is "seeds: "
    let seeds: Vec<i64> = iter.map(|x| x.parse::<i64>().unwrap()).collect();
    let maps = parts.map(parse_single_map).collect();

    let seed_ranges: Vec<(i64, i64)> = (0..seeds.len())
        .step_by(2)
        .map(|i| (seeds[i], seeds[i] + seeds[i + 1]))
        .collect();

    return PuzzleInput {
        seeds,
        seed_ranges,
        maps,
    };
}

fn score1(input: &PuzzleInput) -> i64 {
    let seeds = input.seeds.clone();
    let locations = seeds.into_iter().map(|x| {
        input.maps.iter().fold(x, |acc, map| {
            for ((a, b), c) in map {
                if *a <= acc && acc < *b {
                    // println!("{}<= {} < {}. Offset {}. Ans -> {}", *a, acc, *b, c, c+ (acc - a));
                    return c + (acc - a);
                }
            }
            // println!("{} not found", acc);
            return acc;
        })
    });
    return locations.reduce(|a, b| min(a, b)).unwrap();
}

fn intersect_range(
    a: (i64, i64),
    b: (i64, i64),
    new_start: i64,
) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    // Return mapped, unmapped
    let (a1, a2) = a;
    let (b1, b2) = b;
    if a2 < b1 || b2 < a1 {
        return (vec![], vec![a]);
    }
    if a1 < b1 {
        if a2 < b2 {
            return (vec![(new_start, new_start + a2 - b1)], vec![(a1, b1)]);
        } else {
            return (
                vec![(new_start, new_start + (b2 - b1))],
                vec![(a1, b1), (b2, a2)],
            );
        }
    } else {
        // a1 >=b1
        if a2 <= b2 {
            return (vec![(a1 - b1 + new_start, a2 - b1 + new_start)], vec![]);
        } else {
            return (
                vec![(a1 - b1 + new_start, b2 - b1 + new_start)],
                vec![(b2, a2)],
            );
        }
    }
}

fn map_single_stage(m: SingleMapStage, ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // I tried to do this in multiple folds, but it was too complex for me to follow along.
    let (mapped, unmapped) = m.iter().fold((vec![], ranges.clone()), |acc, map| {
        let (mapped, unmapped) = acc;
        let mut new_mapped = mapped;
        let mut new_unmapped = vec![];
        for r in unmapped {
            let (c, d) = intersect_range(r, map.0, map.1);
            new_mapped = [new_mapped, c].concat();
            new_unmapped = [new_unmapped, d].concat();
        }
        return (new_mapped, new_unmapped);
    });
    return [mapped, unmapped].concat();
}

fn score2(input: &PuzzleInput) -> i64 {
    let starting = input.seed_ranges.clone();
    let mut idx = 0;
    let res = input.maps.iter().fold(starting, |current_range, map| {
        let next_ranges = map_single_stage(map.clone(), &current_range);
        // println!("Stage {}: {:?} -> {:?}", idx, &current_range, next_ranges);
        idx += 1;
        return next_ranges;
    });
    return res.iter().map(|x| x.0).min().unwrap();
}

fn main() {
    let s: String = fs::read_to_string("./src/input5.txt").unwrap();
    let input = parse(s);

    // println!("{:?}", input);
    let score1: i64 = score1(&input);
    println!("{}", score1);
    let score2: i64 = score2(&input);
    println!("{}", score2);
}
