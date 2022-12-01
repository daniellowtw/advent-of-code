use std::cmp::min;
use std::fs;

fn main() {
    let s = fs::read_to_string("./src/input7.txt").unwrap();
    let mut ns: Vec<i64> = s.trim()
        .split(",")
        .map(|x| x.parse().unwrap()).collect();
    ns.sort();

    // When the cost is linear, the answer is simply the median element. When it's even,
    // either one will be fine.
    let n = ns.len();
    let median = ns[n / 2];
    // Trying out this syntax. It's more ergonomic than println.
    dbg!(n, median);
    let s: i64 = ns.iter().fold(0, |b1, x2| { b1 + (x2 - median).abs() });
    println!("Part 1: {}", s);

    // I don't think there's any tricks to finding a custom cost function.
    // We just have to go through all possible numbers within the range
    // and find the one that minimizes the cost.
    let mut lowest = 10e9 as i64;
    let mut x: i32 = 0;
    // Since it's already sorted, we can take the first and the last to find the range to try.
    let first = *ns.first().unwrap() as i32;
    let last = *ns.last().unwrap() as i32;
    dbg!(first, last, lowest);
    for i in first..last {
        let candidate = cost(&ns, i as i64);
        if lowest > candidate {
            lowest = candidate;
            x = i;
        }
    }
    println!("Part 2: {} {} ", x, lowest);
}


fn cost(a: &Vec<i64>, target: i64) -> i64 {
    a.iter().fold(0, |b1, x2| {
        // We could just add a breakpoint here to see what's happening in the iteration.
        b1 + triangle((x2 - target).abs())
    })
}

fn triangle(n: i64) -> i64 {
    (n) * (n + 1) / 2
}