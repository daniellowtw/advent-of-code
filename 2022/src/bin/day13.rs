use json::{self, JsonValue};
use std::fs;

fn main() {
    let s: String = fs::read_to_string("./src/input13.txt").unwrap();
    let tmp: Vec<&str> = s.trim_end().split("\n\n").collect();
    // too lazy to parse the lines. Use json instead.
    let mut ss: Vec<(JsonValue, JsonValue)> = tmp
        .iter()
        .map(|x| {
            let (left, right) = x.split_once("\n").unwrap();
            let left = json::parse(left).unwrap();
            let right = json::parse(right).unwrap();
            (left, right)
        })
        .collect();

    let part1: usize = ss
        .iter()
        .enumerate()
        .map(
            |(x, (a, b))| {
                if less_than(a, b).unwrap() {
                    x + 1
                } else {
                    0
                }
            },
        )
        .sum();

    dbg!(part1);

    let extra = (json::parse("[[2]]").unwrap(), json::parse("[[6]]").unwrap());
    ss.push(extra);
    let mut tmp: Vec<&JsonValue> = ss.iter().flat_map(|(a, b)| vec![a, b]).collect();
    tmp.sort_by(|a, b| {
        if less_than(a, b).unwrap() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let tmp2: Vec<String> = tmp
        .into_iter()
        .map(|x| json::stringify(x.clone()))
        .collect();
    let i = tmp2.iter().position(|x| x == "[[2]]").unwrap();
    let j = tmp2.iter().position(|x| x == "[[6]]").unwrap();
    let part2 = (i + 1) * (j + 1);
    dbg!(part2);
}

fn less_than(a: &JsonValue, b: &JsonValue) -> Option<bool> {
    match (a, b) {
        (JsonValue::Number(a), JsonValue::Number(b)) => {
            let (x, y): (i64, i64) = (
                a.as_fixed_point_i64(0).unwrap(),
                b.as_fixed_point_i64(0).unwrap(),
            );
            if x == y {
                None
            } else {
                Some(x < y)
            }
        }
        (JsonValue::Array(a), JsonValue::Array(b)) => {
            let mut i = 0;
            let mut res = None;
            loop {
                match (a.get(i), b.get(i)) {
                    (None, Some(_)) => {
                        res = Some(true);
                        break;
                    }
                    (Some(_), None) => {
                        res = Some(false);
                        break;
                    }
                    (Some(a), Some(b)) => {
                        let x = less_than(&a, &b);
                        if x.is_some() {
                            res = x;
                            break;
                        }
                    }
                    (None, None) => {
                        break;
                    }
                }
                i += 1;
            }
            return res;
        }
        (JsonValue::Number(a), JsonValue::Array(b)) => {
            return less_than(
                &JsonValue::Array(vec![JsonValue::Number(*a)]),
                &JsonValue::Array(b.clone()),
            );
        }
        (JsonValue::Array(b), JsonValue::Number(a)) => {
            return less_than(
                &JsonValue::Array(b.clone()),
                &JsonValue::Array(vec![JsonValue::Number(*a)]),
            );
        }
        _ => None,
    }
}
