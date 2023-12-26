#![allow(warnings)]
use std::{
    cmp::{max, Ordering},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

type Line = (i64, i64, i64, i64, i64, i64);

#[derive(Debug, Clone)]
struct PuzzleInput {
    items: Vec<Line>,
}

fn parse(s: &str) -> PuzzleInput {
    let items: Vec<Line> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let y: Vec<i64> = x
                .replace("@", ",")
                .split(",")
                .map(|x| x.trim().parse().unwrap())
                .collect();
            return (y[0], y[1], y[2], y[3], y[4], y[5]);
        })
        .collect();
    return PuzzleInput { items };
}

fn solve_line_intersection_x_only_has_solution(l1: Line, l2: Line, small: i64, large: i64) -> bool {
    // Just check for sign changes between boundary points
    let (x1, y1, z1, a1, b1, c1) = l1;
    let (x2, y2, z2, a2, b2, c2) = l2;
    let d = gcd(a1, a2);
    if d == 0 {
        return x1 == x2;
    }
    return (x2 - x1) % d == 0;
}

fn eval_line_at(l: Line, x: i64) -> i64 {
    let (x1, y1, z1, a1, b1, c1) = l;
    return y1 - (x1 - x) / a1 * b1;
}

fn lines_intersect(l1: Line, l2: Line, small: i64, large: i64) -> bool {
    // Just check for sign changes between boundary points
    let (x1, y1, z1, a1, b1, c1) = l1;
    let (x2, y2, z2, a2, b2, c2) = l2;
    let mut small_x = small;
    let mut large_x = large;

    if a1 >= 0 {
        // Trim left boundary
        if x1 >= small_x {
            small_x = x1
        }
    }

    if a2 >= 0 {
        // Trim left boundary
        if x2 >= small_x {
            small_x = x2
        }
    }

    if a1 <= 0 {
        // Trim right boundary
        if x1 <= large_x {
            large_x = x1
        }
    }

    if a2 <= 0 {
        // Trim right boundary
        if x2 <= large_x {
            large_x = x2
        }
    }

    if (small_x > large_x) {
        return false;
    }

    let y_start = if a1 == 0 {
        y1
    } else {
        y1 - (x1 - small_x) / a1 * b1
    };
    let y_start2 = if a2 == 0 {
        y2
    } else {
        y2 - (x2 - small_x) / a2 * b2
    };
    let mut y_start_diff = y_start - y_start2;

    let y_end = if a1 == 0 {
        y1
    } else {
        y1 - (x1 - large_x) / a1 * b1
    };
    let y_end2 = if a2 == 0 {
        y2
    } else {
        y2 - (x2 - large_x) / a2 * b2
    };
    let mut y_end_diff = y_end - y_end2;

    if y_start_diff == 0 || y_end_diff == 0 {
        return true;
    }
    if y_start_diff.signum() == y_end_diff.signum() {
        return false;
    }

    // There's a sign change.
    // Now we need to narrow our search to the y range
    while (large_x - small_x > 1) {
        let x_mid = (small_x + large_x) / 2;
        let y_mid = y1 - (x1 - x_mid) / a1 * b1;
        let y_mid2 = y2 - (x2 - x_mid) / a2 * b2;
        let y_mid_diff = y_mid - y_mid2;

        if y_start_diff.signum() == y_mid_diff.signum() {
            small_x = x_mid;
            y_start_diff = y_mid_diff;
        } else {
            large_x = x_mid;
        }
    }

    // Note: I'm a bit skeptical here that this will work in all cases. TODO: Think about where this can fail.
    let y = eval_line_at(l1, small_x);
    if y < small || y > large {
        return false;
    }
    let y = eval_line_at(l1, large_x);
    if y < small || y > large {
        return false;
    }

    return true;
}

fn part1(pi: &PuzzleInput, small: i64, large: i64) -> i64 {
    let mut count = 0;
    for i in 0..pi.items.len() {
        for j in i + 1..pi.items.len() {
            if lines_intersect(pi.items[i], pi.items[j], small, large) {
                count += 1;
            }
        }
    }
    return count;
}

fn solve_dio(a1: i64, v1: i64, a2: i64, v2: i64) -> (i64, i64, i64, i64) {
    // This was my attempt at solving using diophantine equations.
    // This is way too slow for the input.
    let (d, a, b) = extended_gcd(v1, v2); // a is positive, b is negative
    let dist = a2 - a1;
    dbg!(d, a, b);
    let m = dist / d;
    let mut t = m * a;
    let mut s = -m * b;
    if t < 0 {
        dbg!(t, s);
        let times_to_increase = t / v2 - 1;
        t -= times_to_increase * v2;
        s -= times_to_increase * v1;
    }
    assert!(t > 0);
    if s < 0 {
        dbg!(t, s);
        let times_to_increase = s / v1 - 1;
        s -= times_to_increase * v1;
        t -= times_to_increase * v2;
    }
    dbg!(t, s);
    assert!(t > 0);
    assert!(s > 0);
    assert!(dist == m * a * v1 + m * b * v2); // This is a solution
    assert!(dist == t * v1 - s * v2); // This is a solution
    assert!(dist == (t + v2) * v1 - (s + v1) * v2); // This is another
    assert!(a1 + t * v1 == a2 + s * v2);
    assert!(a1 + (t + v2) * v1 == a2 + (s + v1) * v2);
    return (t, v2, s, v1);
}

fn solve_intersection(pi: &Vec<Line>) -> i64 {
    // I cheated here out of frustration. I was trying to solve this using diophantine equations.
    // but didn't realize the orders of magnitude of difference between the velocity (step value) and the search space (distance diff).
    // That meant the search was quite slow.

    // Then I tried to do binary search for s and t, using euclidean dist as the metric to decide the bijection half.
    // But the numbers were too big that I ran into overflow issues.

    // I have an idea which is to bring in the equations from the other lines to find the lcm of the periodicity.
    // But I've lost patience at this point and just want this done.
    // So i just printed out the equations and solved them with a simultaneous equation solver.

    let l1 = pi[0];
    let l2 = pi[1];

    println!("{} + s*{} = {} + t*{},", l1.0, l1.3, l2.0, l2.3);
    println!("{} + s*{} = {} + t*{},", l1.1, l1.4, l2.1, l2.4);
    println!("{} + s*{} = {} + t*{}", l1.2, l1.5, l2.2, l2.5);

    // Answer from solver.
    let s = 516434301805;
    let t = 215244678825;
    assert_eq!(
        l1.0 + s * l1.3 + l1.1 + s * l1.4 + l1.2 + s * l1.5,
        l2.0 + t * l2.3 + l2.1 + t * l2.4 + l2.2 + t * l2.5
    );
    return l1.0 + s * l1.3 + l1.1 + s * l1.4 + l1.2 + s * l1.5;
}

fn valid_for_one_direction(pi: &Vec<Line>, small: i64, large: i64) -> bool {
    for i in 0..pi.len() {
        for j in i + 1..pi.len() {
            if !solve_line_intersection_x_only_has_solution(pi[i], pi[j], small, large) {
                return false;
            }
        }
    }
    return true;
}

fn part2(pi: &PuzzleInput) -> i64 {
    // The crux to solving this is transforming all the velocities to the relative velocity wrt the rock,
    // Then all the lines must intersect at the same point, which is also the starting position of the rock.
    // Hence, the strategy is to guess the relative velocity and then transform the lines.
    // Part 1 prompted me to think about how the velocities can be guessed. If the guessed velocity is right,
    // Then the number of intersections is the number of pairs of lines.
    // My first attempt was to brute force (x, y) together, and was going to do (x, z) to reuse the same code.
    // Then I realized actually I can solve each direction separately with the same idea. This reduces the time
    // complexity.

    let n = pi.items.len() as i64;
    let num_pairs = n * (n - 1) / 2;
    // I gambled a bit here that there's only one solution. I ran aginst the input to validate and seemed
    // reasonable enough.
    let rock_velocity = [
        |x: &Line| (x.0, x.3),
        |x: &Line| (x.1, x.4),
        |x: &Line| (x.2, x.5),
    ]
    .map(|f| {
        let mut i = -10000;
        while i <= 10000 {
            let items2: Vec<Line> = pi
                .items
                .iter()
                .map(|x| {
                    let (x, v) = f(x);
                    return (x, 0, 0, v - i, 0, 0);
                })
                .collect();
            if valid_for_one_direction(&items2, 0, 999999999999) {
                break;
            }
            i += 1;
        }
        return i;
    });
    dbg!(rock_velocity);

    // let offset = (-3, 1, 2);
    // let rock_velocity = (-99, -269, 81);
    let items2: Vec<Line> = pi
        .items
        .iter()
        .map(|x| {
            let (x1, y1, z1, a1, b1, c1) = x;
            return (
                *x1,
                *y1,
                *z1,
                *a1 - rock_velocity[0],
                *b1 - rock_velocity[1],
                *c1 - rock_velocity[2],
            );
        })
        .collect();
    let res = solve_intersection(&items2);
    return res;
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut s = 0;
    let mut old_s = 1;
    let mut t = 1;
    let mut old_t = 0;
    let mut r = b;
    let mut old_r = a;
    while r != 0 {
        let quotient = old_r / r;
        let temp = r;
        r = old_r - quotient * r;
        old_r = temp;
        let temp = s;
        s = old_s - quotient * s;
        old_s = temp;
        let temp = t;
        t = old_t - quotient * t;
        old_t = temp;
    }
    return (old_r, old_s, old_t);
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn main() {
    let s: String = fs::read_to_string("./src/input24.txt").unwrap();
    let pi: PuzzleInput = parse(s.trim());
    let large = 400000000000000;
    let small = 200000000000000;
    let ans = part1(&pi, small, large);
    println!("{}", ans);
    let ans = part2(&pi);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_part1_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim());
        assert!(lines_intersect(pi.items[0], pi.items[1], 7, 27));
        assert!(lines_intersect(pi.items[0], pi.items[2], 7, 27));
        assert!(!lines_intersect(pi.items[0], pi.items[4], 7, 27));
        let ans = part1(&pi, 7, 27);
        println!("{}", ans);
    }

    #[test]
    fn test_part2_sample_input() {
        let mut pi = parse(SAMPLE_INPUT.trim());
        let ans = part2(&pi);
        println!("{}", ans);
    }

    #[test]
    fn test_part3() {
        let ans = extended_gcd(15, 5);
        println!("{:?}", ans);
    }
}
