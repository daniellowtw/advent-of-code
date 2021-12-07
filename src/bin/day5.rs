use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

struct Line {
    a: Point,
    b: Point,
}

impl Line {
    // Return change in x and change in y
    fn slope(&self) -> (i32, i32) {
        let cy = self.b.y - self.a.y;
        let cx = self.b.x - self.a.x;

        if cy == 0 {
            return (cx / cx.abs(), 0);
        }
        if cx == 0 {
            return (0, cy / cy.abs());
        }

        let d = gcd(cx, cy);
        return (cx / d, cy / d);
    }
}

fn main() {
    // I solved this in golang first and am rewriting in rust.
    let s = fs::read_to_string("./src/input5.txt").unwrap();
    // The following parse logic splits by -> and then splits by "," to parse out the x and y.
    let line_segments: Vec<Line> = s.split("\n").filter(|x| !x.is_empty()).map(|line| {
        let t: Vec<Point> = line
            .split(" -> ")
            .map(|p| {
                let t: Vec<i32> = p.split(",").map(|x| x.parse().unwrap()).collect();
                Point { x: t[0], y: t[1] }
            })
            .collect();

        Line {
            a: t[0],
            b: t[1],
        }
    }).collect();

    let mut map: HashMap<Point, i32> = HashMap::new();

    // The main logic here is to walk along the line and increase the counter for each point.
    for x in line_segments {
        let (cx, cy) = x.slope();
        // Part 1. Uncomment for part one.
        //     if cx != 0 && cy != 0 {
        //         continue
        //     }

        // Part 2
        let mut candidate = x.a;
        loop {
            // copied from day6
            *map.entry(candidate).or_insert(0) += 1;
            candidate.x += cx;
            candidate.y += cy;
            if candidate == x.b {
                *map.entry(candidate).or_insert(0) += 1;
                break;
            }
        }
    }

    let ans = map.iter().filter(|&x| *(x.1) >= 2).count();

    dbg!(ans);
}

fn gcd(x: i32, y: i32) -> i32 {
    assert_ne!(x, 0);
    assert_ne!(y, 0);
    if x < 0 || y < 0 {
        return gcd(x.abs(), y.abs());
    }

    if x > y {
        return gcd(y, x);
    }

    if y % x == 0 { x } else { gcd(y % x, x) }
}

