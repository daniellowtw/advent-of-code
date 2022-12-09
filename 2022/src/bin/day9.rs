use std::{collections::HashSet, fs};

fn main() {
    let s: String = fs::read_to_string("./src/input9.txt").unwrap();
    let ss: Vec<(&str, &str)> = s
        .trim_end()
        .split("\n")
        .map(|x| x.split_once(" ").unwrap())
        .collect();
    let score = calculate_part1(&ss);
    println!("Part 1: {}", score);

    let score = calculate_part2(&ss);
    println!("Part 2: {}", score);
}

fn calc_move(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    let (mut dx, mut dy) = (h.0 - t.0, h.1 - t.1);
    if dx.abs() <= 1 && dy.abs() <= 1 {
        return (0, 0);
    }
    if dx.abs() == 2 {
        dx = 1*dx.signum();
    }
    if dy.abs() == 2 {
        dy = 1*dy.signum();
    }
    return (dx, dy);
}

fn calculate_part1(lines: &Vec<(&str, &str)>) -> usize {
    let mut seen = HashSet::new();
    let (mut x1, mut y1) = (0, 0);
    let (mut x2, mut y2) = (0, 0);
    for line in lines {
        let times = line.1.parse::<i32>().unwrap();
        for _ in 0..times {
            match line.0 {
                "L" => {
                    x1 -= 1;
                }
                "R" => {
                    x1 += 1;
                }
                "U" => {
                    y1 += 1;
                }
                "D" => {
                    y1 -= 1;
                }
                _ => panic!("Unknown direction"),
            }
            let (dx, dy) = calc_move((x1, y1), (x2, y2));
            x2 += dx;
            y2 += dy;
            seen.insert((x2, y2));
        }
    }
    return seen.len();
}

fn calculate_part2(lines: &Vec<(&str, &str)>) -> usize {
    let mut seen = HashSet::new();
    let mut knots = [(0,0);10];
    for line in lines {
        let times = line.1.parse::<i32>().unwrap();
        for _ in 0..times {
            match line.0 {
                "L" => {
                    knots[0].0 -= 1;
                }
                "R" => {
                    knots[0].0 += 1;
                }
                "U" => {
                    knots[0].1 += 1;
                }
                "D" => {
                    knots[0].1 -= 1;
                }
                _ => panic!("Unknown direction"),
            }
            for i in 1..10 {
                let (dx, dy) = calc_move(knots[i-1], knots[i]);
                knots[i].0 += dx;
                knots[i].1 += dy;
            }
            seen.insert(knots[9]);
        }
    }

    return seen.len();
}
