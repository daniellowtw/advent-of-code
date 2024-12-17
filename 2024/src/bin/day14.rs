use std::{collections::HashMap, fs};

fn part1(pi: &Vec<(i32, i32, i32, i32)>, width: i32, height: i32) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    for (x, y, z, w) in pi {
        let nx = ((x + z * 100) % width + width) % width;
        let ny = ((y + w * 100) % height + height) % height;
        *map.entry((nx, ny)).or_insert(0) += 1;
    }
    let mut counts = [0; 4];
    for (k, v) in map {
        let (x, y) = k;
        if x == width / 2 || y == height / 2 {
            continue;
        }
        if x < width / 2 && y < height / 2 {
            counts[0] += v;
        } else if x > width / 2 && y < height / 2 {
            counts[1] += v;
        } else if x < width / 2 && y > height / 2 {
            counts[2] += v;
        } else {
            counts[3] += v;
        }
        // println!("{:?} {:?}", k, v);
    }
    return counts.iter().fold(1, |acc, x| acc * x);
}

fn calculate_dist_sq(points: &Vec<(i32, i32)>) -> i64 {
    let mut dist = 0;
    for (px, py) in points.iter() {
        for (qx, qy) in points.iter() {
            dist += ((*px as i64 - *qx as i64).abs() + (*py as i64 - *qy as i64).abs()).pow(2);
        }
    }
    return dist;
}

fn part2(pi: &Vec<(i32, i32, i32, i32)>, width: i32, height: i32) -> i32 {
    // The following algo works after solving it the first time and understanding what the tree looks like.
    // The assumption here is that the correct answer has the distance minimized.

    // The idea here is we find a time where the distance between all the points is minimized.
    // From 1 to 103, we know we would find x such that x == ans % 101, or x == ans % 103.
    // We won't know which state we are in, but we find the minimum in both states.
    let mut min_dist = 9e9 as i64;
    let mut ans = 0;
    for i in 1..=101 {
        let mut points: Vec<(i32, i32)> = vec![];
        for (x, y, z, w) in pi {
            let nx = ((x + z * i) % width + width) % width;
            let ny = ((y + w * i) % height + height) % height;
            points.push((nx, ny));
        }

        let dist = calculate_dist_sq(&points);
        if dist < min_dist {
            min_dist = dist;
            ans = i;
        }
    }
    // So now we need to determine if we are in the 101 case or 103 case. So just try both.
    for step in [101, 103].iter() {
        for i in (ans..=101 * 103).step_by(*step) {
            let mut points: Vec<(i32, i32)> = vec![];
            for (x, y, z, w) in pi {
                let nx = ((x + z * i) % width + width) % width;
                let ny = ((y + w * i) % height + height) % height;
                points.push((nx, ny));
            }

            let dist = calculate_dist_sq(&points);
            if dist < min_dist {
                min_dist = dist;
                ans = i;
            }
        }
    }
    return ans;
}

fn _part2_original(pi: &Vec<(i32, i32, i32, i32)>, width: i32, height: i32) -> i32 {
    // for i in (1..=101*103).step_by(101) {
    // This is my first attempt. It is imperative to note that 101 and 103 are prime. So we know the x positions will repeat every 101 and y positions will repeat every 103. So the grid will repeat every 101*103.
    // I ran this and saved the output in a txt file. And noticed that in certain iterations, the entrophy is very low. There are some grids where the x axis distribution is concentrated, and some where y is concentrated.
    // Noticing this, I saw that they repeat every (101 and 103).
    // So this is essentially solving the chinese remainder theorem.  I need to find x where x % 101 = a and x % 103 = b.
    // Where a and b can be read out from the initial output.
    // At this point I could probably solve it with pen and paper since the numbers are small. But I decided to keep the spirit of my algorithm
    // And just print out the grid for times a + 101 * k, then read out from the output, which there will only be 102 of them.

    for i in (72..=101 * 103).step_by(101) {
        let mut map = vec![vec![false; width as usize]; height as usize];

        for (x, y, z, w) in pi {
            let nx = ((x + z * i) % width + width) % width;
            let ny = ((y + w * i) % height + height) % height;
            map[ny as usize][nx as usize] = true;
        }

        // for j in 0..height {
        //     for k in 0..width {
        //         print!(
        //             "{}",
        //             if map[j as usize][k as usize] {
        //                 '#'
        //             } else {
        //                 '.'
        //             }
        //         );
        //     }
        //     println!();
        // }
        // println!("-----------------{}-----------------", i);
    }
    return 0;
}

fn main() {
    let s: String = fs::read_to_string("./input/14.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-14.txt").unwrap();
    let ss: Vec<(i32, i32, i32, i32)> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let row: Vec<Vec<i32>> = x
                .split(" ")
                .map(|y| {
                    y[2..]
                        .trim()
                        .split(",")
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect()
                })
                .collect();

            (row[0][0], row[0][1], row[1][0], row[1][1])
        })
        .collect();
    // Example code requires a different grid input!
    // println!("{}", part1(ss, 11, 7));
    println!("{}", part1(&ss, 101, 103));
    println!("{}", part2(&ss, 101, 103));
    // println!("{}", part2(ss));
}
