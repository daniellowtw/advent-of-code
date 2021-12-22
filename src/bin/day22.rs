use std::fs;
use std::time::Instant;
use itertools::Itertools;
use crate::RangeType::*;

fn split_range(x: &str) -> Range {
    let (a, b) = x[2..].split_once("..").unwrap();
    Range(a.parse().unwrap(), b.parse().unwrap())
}

#[derive(Debug, Eq, PartialEq)]
enum RangeType {
    // This means it's included in the original range, but not the one compared against.
    Included(i64, i64),
    // This means it is overlapping with the one compared against.
    Overlapped(i64, i64),
}

// Ranges are inclusive.
#[derive(Debug, Clone)]
struct Range(i64, i64);

impl Range {
    // The strategy for this problem involves breaking the range into critical segments and then
    // asking what type is it. We are only concerned with whether it's in the original range,
    // and whether it overlaps.
    fn intersect(&self, other: &Range) -> (Vec<RangeType>, bool) {
        // 6 cases to consider
        // Disjoint cases
        if self.1 < other.0 || self.0 > other.1 {
            (vec![Included(self.0, self.1)], true)
        } else if other.0 <= self.0 && other.1 >= self.1 {
            // Nothing case
            (vec![Overlapped(self.0, self.1)], false)
        } else if other.0 > self.0 && other.1 < self.1 {
            // Split into 2 case
            (vec![Included(self.0, other.0 - 1), Overlapped(other.0, other.1), Included(other.1 + 1, self.1)], false)
        } else if other.1 < self.1 {
            // Excluded, Overlapped, included
            (vec![Overlapped(self.0, other.1), Included(other.1 + 1, self.1)], false)
        } else if other.0 > self.0 {
            // Included, Overlapped, Excluded
            // Right split
            (vec![Included(self.0, other.0 - 1), Overlapped(other.0, self.1)], false)
        } else {
            todo!()
        }
    }
}

#[derive(Debug, Clone)]
struct Cubiod(Range, Range, Range, bool);

impl Cubiod {
    fn area(&self) -> i64 {
        (self.0.1 - self.0.0 + 1) * (self.1.1 - self.1.0 + 1) * (self.2.1 - self.2.0 + 1)
    }

    // Subtrack is self - other. E.g. if a 3x3x3 cube - 1x1x1 cube in the core, then we
    // should have a vector of cuboids representing the shell.
    // This algo is dumb - I repeat, dumb - and can return up to 27 elements!
    fn subtract(&self, other: &Cubiod) -> Vec<Cubiod> {
        // A crux to solving this problem without running forever is to exit early when it's
        // disjointed! Any axes that are disjoint can be short circuited.
        let mut res = Vec::new();
        let (xs, short) = &self.0.intersect(&other.0);
        if *short {
            return vec![self.clone()];
        }
        let (ys, short) = &self.1.intersect(&other.1);
        if *short {
            return vec![self.clone()];
        }
        let (zs, short) = &self.2.intersect(&other.2);
        if *short {
            return vec![self.clone()];
        }
        for x in xs {
            for y in ys {
                for z in zs {
                    match (x, y, z) {
                        (&Included(x1, x2), &Included(y1, y2), &Included(z1, z2)) =>
                            res.push(Cubiod(Range(x1, x2), Range(y1, y2), Range(z1, z2), true)),
                        (&Overlapped(x1, x2), &Included(y1, y2), &Included(z1, z2)) =>
                            res.push(Cubiod(Range(x1, x2), Range(y1, y2), Range(z1, z2), true)),
                        (&Included(x1, x2), &Overlapped(y1, y2), &Included(z1, z2)) =>
                            res.push(Cubiod(Range(x1, x2), Range(y1, y2), Range(z1, z2), true)),
                        (&Included(x1, x2), &Included(y1, y2), &Overlapped(z1, z2)) =>
                            res.push(Cubiod(Range(x1, x2), Range(y1, y2), Range(z1, z2), true)),
                        (&Overlapped(x1, x2), &Overlapped(y1, y2), &Included(z1, z2)) =>
                            res.push(Cubiod(Range(x1, x2), Range(y1, y2), Range(z1, z2), true)),
                        (&Included(x1, x2), &Overlapped(y1, y2), &Overlapped(z1, z2)) =>
                            res.push(Cubiod(Range(x1, x2), Range(y1, y2), Range(z1, z2), true)),
                        (&Overlapped(x1, x2), &Included(y1, y2), &Overlapped(z1, z2)) =>
                            res.push(Cubiod(Range(x1, x2), Range(y1, y2), Range(z1, z2), true)),
                        (&Overlapped(_x1, _x2), &Overlapped(_y1, _y2), &Overlapped(_z1, _z2)) => (),
                    }
                }
            }
        }
        res
    }
}

fn main() {
    let s: String = fs::read_to_string("./src/input22.txt").unwrap();
    let entries = s.split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (state, rest) = line.split_once(" ").unwrap();
            // let line = &line[3..];
            let (x, rest) = rest.split_once(",").unwrap();
            let (y, z) = rest.split_once(",").unwrap();
            let a: Cubiod = Cubiod(split_range(x), split_range(y), split_range(z), state == "on");
            // dbg!(&a);
            a
        }).collect_vec();

    // Part one is naive and we can just brute force.
    naive_part1(entries.clone());

    let start = Instant::now();
    // For part 2, we should split cuboids into distinct non overlapping cuboids where we can
    // count the region easily.
    let count = simluate_and_count(&entries);
    dbg!(start.elapsed());
    dbg!(count);


    let start = Instant::now();
    let count = reverse_and_count_smarter(entries);
    dbg!(start.elapsed());
    dbg!(count);
}

fn reverse_and_count_smarter(entries: Vec<Cubiod>) -> i64 {
    // While trying to optimize this, I was considering alternative ways to iterate the list.
    // One insight is if we process backwards, then those that have already considered can
    // simply be stored as one single cuboid, instead of being stored as fragments like in the
    // first case.
    let mut count = 0;
    let mut blackhole: Vec<Cubiod> = Vec::new();
    // Reversing the list should greatly reduces the number of cuboids to consider.
    for c in entries.into_iter().rev() {
        if (&c).3 {
            let mut remainder = vec![c.clone()];
            for existing in &blackhole {
                let mut new_remainder: Vec<Cubiod> = vec![];
                for r in remainder {
                    for new_el in r.subtract(&existing) {
                        new_remainder.push(new_el);
                    }
                }
                remainder = new_remainder;
            }
            count += remainder.iter().fold(0, |b, x1| { x1.area() + b });
        }
        blackhole.push(Cubiod(c.0, c.1, c.2, true));
    }
    count
}

fn simluate_and_count(entries: &Vec<Cubiod>) -> i64 {
    let mut remainder: Vec<Cubiod> = vec![];
    for  c in entries.into_iter() {
        let mut new_remainder: Vec<Cubiod> = vec![];
        for existing in &remainder {
            for r in existing.subtract(&c) {
                new_remainder.push(r);
            }
        }
        remainder = new_remainder;
        if c.3 {
            remainder.push(c.clone())
        }
    }
    let mut count = 0;
    for x in remainder {
        count += x.area()
    }
    count
}

fn naive_part1(entries: Vec<Cubiod>) {
    // Clearly part 1 is small enough to be brute forced... So lets try that.
    let mut state = Vec::new();
    let size = 101;
    for _i in 0..size {
        let mut y = Vec::new();
        for _i in 0..size {
            let mut z = Vec::new();
            z.resize(size, false);
            y.push(z)
        }
        state.push(y)
    }

    entries.iter()
        .filter(|&x1| {
            let Cubiod(x, y, z, _) = x1;
            !(x.0 < -50 || x.1 > 50 || y.0 < -50 || y.1 > 50 || z.0 < -50 || z.1 > 50)
        }).for_each(
        |c| {
            for x in c.0.0..=c.0.1 {
                for y in c.1.0..=c.1.1 {
                    for z in c.2.0..=c.2.1 {
                        if c.3 {
                            state[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] |= c.3
                        } else {
                            state[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] &= c.3
                        }
                    }
                }
            }
        }
    );
    let mut count = 0;
    for x in state {
        for y in x {
            for x in y {
                if x {
                    count += 1;
                }
            }
        }
    }
    dbg!(count);
}

// Wrote some tests to make sure my intersection logic is sound.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_overlap_partially() {
        let (c, _) = Range(-1, 1).intersect(&Range(0, 1));
        assert_eq!(c, vec![Included(-1, -1), Overlapped(0, 1)])
    }

    #[test]
    fn overlap_with_subset() {
        let range = || Range(-1, 1); // 3x3x3
        let c = Cubiod(range(), range(), range(), true);
        let range2 = || Range(0, 1); //2x2x2
        let c2 = Cubiod(range2(), range2(), range2(), true);
        let vec = &c.subtract(&c2);
        let area = vec.iter().fold(0, |b, x| { b + x.area() });
        assert_eq!(area, 19);
    }

    #[test]
    fn overlap_with_larger() {
        let range = || Range(-1, 1); // 3x3x3
        let c = Cubiod(range(), range(), range(), true);
        let range2 = || Range(0, 3); //2x2x2
        let c2 = Cubiod(range2(), range2(), range2(), true);
        let vec = c.subtract(&c2);
        let area = vec.iter().fold(0, |b, x| { b + x.area() });
        assert_eq!(area, 19);
    }

    #[test]
    fn overlap_inside() {
        let range = || Range(-1, 1); // 3x3x3
        let c = Cubiod(range(), range(), range(), true);
        let range2 = || Range(0, 0); //2x2x2
        let c2 = Cubiod(range2(), range2(), range2(), true);
        let remaining = c.subtract(&c2);
        let area = remaining.iter().fold(0, |b, x| { b + x.area() });
        assert_eq!(area, 26);
    }

    #[test]
    fn overlap_outside() {
        let range = || Range(-1, 1); // 3x3x3
        let c = Cubiod(range(), range(), range(), true);
        let range2 = || Range(0, 0); //2x2x2
        let c2 = Cubiod(range2(), range2(), range2(), true);
        let remaining = c2.subtract(&c);
        assert_eq!(remaining.len(), 0);
        let area = remaining.iter().fold(0, |b, x| { b + x.area() });
        assert_eq!(area, 0);
    }

    #[test]
    fn overlap_exact() {
        let range = || Range(-1, 1); // 3x3x3
        let c = Cubiod(range(), range(), range(), true);
        let c2 = Cubiod(range(), range(), range(), true);
        let remaining = c2.subtract(&c);
        assert_eq!(remaining.len(), 0);
        let area = remaining.iter().fold(0, |b, x| { b + x.area() });
        assert_eq!(area, 0);
    }

    #[test]
    fn overlap_2_axes() {
        let c = Cubiod(Range(0, 2), Range(0, 2), Range(0, 4), true);
        let c2 = Cubiod(Range(0, 2), Range(0, 2), Range(0, 2), true); // 3x3x3
        let remaining = c.subtract(&c2);
        let area = remaining.iter().fold(0, |b, x| { b + x.area() });
        assert_eq!(area, 18); //3x3x2
    }
}
