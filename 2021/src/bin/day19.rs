use std::{fs};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::time::Instant;
use itertools::Itertools;

#[derive(Eq, PartialEq, Hash)]
struct Point(i32, i32, i32);

const ZERO: Point = Point(0, 0, 0);

impl Point {
    pub(crate) fn minus(&self, p0: &Point) -> Point {
        Point(self.0 - p0.0, self.1 - p0.1, self.2 - p0.2)
    }
    pub(crate) fn add(&self, p0: &Point) -> Point {
        Point(self.0 + p0.0, self.1 + p0.1, self.2 + p0.2)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point {} {} {}", self.0, self.1, self.2)
    }
}

fn generate_rotation_for_points(p: &Vec<Point>) -> Vec<Vec<Point>> {
    // Keyed by rotation id, which there should be 24
    let mut res: Vec<Vec<Point>> = Vec::new();
    for _i in 0..24 {
        res.push(Vec::new())
    }
    for x in p {
        let new_points = generate_rotation(x);
        let mut id = 0;
        for pp in new_points {
            res.get_mut(id).unwrap().push(pp);
            id += 1;
        }
    }
    res
}

fn generate_rotation(p: &Point) -> [Point; 24] {
    // This function fk'ed my brain. I ended up just searching for the matrices of the 24 rotations.
    // https://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
    let Point(a, b, c) = *p;
    [
        // Rotating around z axis => z is always the same
        Point(a, b, c),
        Point(a, -c, b),
        Point(a, -b, -c),
        Point(a, c, -b),
        Point(-b, a, c),
        Point(c, a, b),
        Point(b, a, -c),
        Point(-c, a, -b),
        Point(-a, -b, c),
        Point(-a, -c, -b),
        Point(-a, b, -c),
        Point(-a, c, b),
        Point(b, -a, c),
        Point(c, -a, -b),
        Point(-b, -a, -c),
        Point(-c, -a, b),
        Point(-c, b, a),
        Point(b, c, a),
        Point(c, -b, a),
        Point(-b, -c, a),
        Point(-c, -b, -a),
        Point(-b, c, -a),
        Point(c, b, -a),
        Point(b, -c, -a),
    ]
}

struct Scanner {
    // To align with reference scanner
    rotation_idx: Option<usize>,
    // Translation after rotation to get it to be in same reference frame.
    translation: Option<Point>,
    rotated_relative_diff: Vec<HashMap<Point, HashSet<Point>>>,
    // original_relative_diff: HashMap<Point, HashSet<Point>>,
    rotated_initial_points: Vec<Vec<Point>>,
}

impl Scanner {
    pub(crate) fn mark_as_solved(&mut self, rotation_idx: usize, translation: Point) {
        self.rotation_idx = Some(rotation_idx);
        self.translation = Some(translation);
    }
    pub fn original_relative_diff(&self) -> &HashMap<Point, HashSet<Point>> {
        &self.rotated_relative_diff[self.rotation_idx.unwrap()]
    }
}

fn relative_diff(ps: &Vec<Point>) -> HashMap<Point, HashSet<Point>> {
    let l = ps.len();
    let mut res: HashMap<Point, HashSet<Point>> = HashMap::with_capacity(l);
    for i in 0..l {
        let a = &ps[i];
        let mut row: HashSet<Point> = HashSet::with_capacity(l);
        for j in 0..l {
            let b = &ps[j];
            row.insert(Point(b.0 - a.0, b.1 - a.1, b.2 - a.2));
        }
        res.insert(a.add(&Point(0, 0, 0)), row);
    }
    res
}

impl Scanner {
    fn new(s: &str) -> Scanner {
        let initial_points_for_scanner = s.split("\n")
            .filter(|x| { !x.is_empty() && !x.starts_with("---") })
            .map(|x1| {
                let vals = x1
                    .split(",")
                    .filter(|x| !x.is_empty())
                    .map(|x2| x2.parse::<i32>().unwrap())
                    .collect_vec();
                let i = vals.get(0).unwrap();
                let j = vals.get(1).unwrap();
                let k = vals.get(2).unwrap();
                Point(*i, *j, *k)
            })
            .collect_vec();
        let mut rotated_relative_differences: Vec<HashMap<Point, HashSet<Point>>> = Vec::new();
        let rotated_initial_points = generate_rotation_for_points(&initial_points_for_scanner);
        for initial_points in rotated_initial_points.iter() {
            let rd = relative_diff(initial_points);
            rotated_relative_differences.push(rd);
        }
        assert_eq!(rotated_relative_differences.len(), 24);
        Scanner {
            rotated_relative_diff: rotated_relative_differences,
            rotation_idx: None,
            translation: None,
            // original_relative_diff: relative_diff(&initial_points_for_scanner),
            rotated_initial_points,
        }
    }
}

fn main() {
    let start = Instant::now();
    let s: String = fs::read_to_string("./src/input19.txt").unwrap();
    let mut scanners: VecDeque<Scanner> = s.split("\n\n").map(|x| { Scanner::new(x) }).collect();

    let mut solved_scanners: Vec<Scanner> = Vec::new();

    let mut ref_scanner = scanners.pop_front().unwrap();
    ref_scanner.mark_as_solved(0, Point(0,0,0));
    solved_scanners.push(ref_scanner);

    while !scanners.is_empty() {
        dbg!(&solved_scanners.len());
        let mut candidate = scanners.pop_front().unwrap();
        let mut changed = false;
        for ref_scanner in solved_scanners.iter() {
            let (overlap, rotation_idx, point_idx) =
                calculate_overlap(&candidate.rotated_relative_diff, &ref_scanner.original_relative_diff());
            if overlap >= 12 {
                let (a, b) = point_idx.unwrap();
                let translation = b.add(&ref_scanner.translation.as_ref().unwrap()).minus(&a);
                dbg!(&translation);
                candidate.mark_as_solved(rotation_idx, translation);
                changed = true;

                // Following to debug the final beacon locations.
                // let offset = &candidate.translation.as_ref().unwrap();
                // let i = (&candidate).rotation_idx.unwrap();
                // for pt in &candidate.rotated_initial_points[i] {
                //     dbg!(&pt.add(offset));
                // }
                break;
            }
        }
        if changed {
            solved_scanners.push(candidate)
        } else {
            scanners.push_back(candidate)
        }
    }

    // Current solution is really really inefficient. It takes about 4m on my i7-7700HQ machine.
    // Would be interesting to see how I can reduce this.

    // Collect all the points we have.
    let mut final_points: HashSet<Point> = HashSet::new();
    for x in solved_scanners.iter() {
        let offset = x.translation.as_ref().unwrap();
        for pt in &x.rotated_initial_points[x.rotation_idx.unwrap()] {
            final_points.insert(offset.add(&pt));
        }
    }
    dbg!(final_points.len());

    let mut largest_dist = 0;
    for x in solved_scanners.iter().combinations(2) {
        let p1 = x.first().unwrap().translation.as_ref().unwrap();
        let p2 = x.last().unwrap().translation.as_ref().unwrap();
        let dist = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs();
        if dist > largest_dist {
            largest_dist = dist
        }
    }
    dbg!(largest_dist);
    dbg!(start.elapsed());

    // for x in final_points {
    //     dbg!(x);
    // }
}

fn calculate_overlap(
    candidate_rotated_relative_diffs: &Vec<HashMap<Point, HashSet<Point>>>,
    ref_scanner_rd: &HashMap<Point, HashSet<Point>>,
) -> (usize, usize, Option<(Point, Point)>) {
    let mut max_overlap = 0;
    // Try all rotations
    let mut max_rotated_id = 0;
    let mut max_location = None;
    for (rotated_id, candidate_rd) in candidate_rotated_relative_diffs.iter().enumerate() {
        let (overlap, location) = calculate_overlap_once(candidate_rd, ref_scanner_rd);
        if overlap >= max_overlap {
            max_overlap = overlap;
            max_rotated_id = rotated_id;
            max_location = location;
        }
    }
    return (max_overlap, max_rotated_id, max_location);
}

fn calculate_overlap_once(candidate: &HashMap<Point, HashSet<Point>>, ref_obj: &HashMap<Point, HashSet<Point>>) -> (usize, Option<(Point, Point)>) {
    let mut local_max = 0;
    let mut local_translation = None;
    for (i, rd_from_single_point) in candidate.iter() {
        for (j, candidate_rd_from_single_point) in ref_obj.iter() {
            let a = rd_from_single_point.intersection(&candidate_rd_from_single_point).collect_vec();
            let s = (&a).len();
            if s > local_max {
                // dbg!(&a);
                local_max = s;
                local_translation = Some((i.add(&ZERO), j.add(&ZERO)))
            }
        }
    }
    (local_max, local_translation)
}
