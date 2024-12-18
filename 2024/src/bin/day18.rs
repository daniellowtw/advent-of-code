use std::{collections::VecDeque, fs};

fn part1(s: &str) -> i32 {
    let ss: Vec<(i32, i32)> = s
        .trim()
        .lines()
        .map(|x| {
            let xs: Vec<i32> = x.split(",").map(|y| y.parse::<i32>().unwrap()).collect();
            (xs[0], xs[1])
        })
        .collect();
    let width = 71;
    let height = 71;
    let simulation = 1024;
    let mut grid = vec![vec!["."; width + 2]; height + 2];
    for i in 0..height + 2 {
        grid[i][0] = "#";
        grid[i][width + 1] = "#";
    }
    for i in 0..width + 2 {
        grid[0][i] = "#";
        grid[height + 1][i] = "#";
    }
    for i in 0..simulation {
        grid[ss[i].1 as usize + 1][ss[i].0 as usize + 1] = "#";
    }
    let mut heap = VecDeque::new();
    // let mut heap = BinaryHeap::new();
    heap.push_back((0, 1, 1));
    let mut score = vec![vec![9999999; width + 2]; height + 2];
    // let mut seen: HashSet<(usize, usize)> = HashSet::new();
    while !heap.is_empty() {
        let (d, x, y) = heap.pop_front().unwrap();
        if score[y as usize][x as usize] <= d {
            continue;
        }
        score[y as usize][x as usize] = d;
        if x == width && y == height {
            return d;
        }
        if grid[y as usize][x as usize] == "#" {
            continue;
        }
        grid[y as usize][x as usize] = "X";
        heap.push_back((d + 1, x + 1, y));
        heap.push_back((d + 1, x - 1, y));
        heap.push_back((d + 1, x, y + 1));
        heap.push_back((d + 1, x, y - 1));
    }
    return 0;
}

fn part2(s: &str) -> String {
    let ss: Vec<(i32, i32)> = s
        .trim()
        .lines()
        .map(|x| {
            let xs: Vec<i32> = x.split(",").map(|y| y.parse::<i32>().unwrap()).collect();
            (xs[0], xs[1])
        })
        .collect();
    let width = 71;
    let height = 71;

    for simulation in 1024..ss.len() {
        let mut grid = vec![vec!["."; width + 2]; height + 2];
        for i in 0..height + 2 {
            grid[i][0] = "#";
            grid[i][width + 1] = "#";
        }
        for i in 0..width + 2 {
            grid[0][i] = "#";
            grid[height + 1][i] = "#";
        }
        for i in 0..simulation {
            grid[ss[i].1 as usize + 1][ss[i].0 as usize + 1] = "#";
        }
        // for i in 0..height+2 {
        //     for j in 0..width+2 {
        //         print!("{}", grid[i][j]);
        //     }
        //     println!();
        // }
        let mut heap = VecDeque::new();
        // let mut heap = BinaryHeap::new();
        heap.push_back((0, 1, 1));
        let mut score = vec![vec![9999999; width + 2]; height + 2];
        // let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut can_escape = false;
        while !heap.is_empty() {
            let (d, x, y) = heap.pop_front().unwrap();
            // if seen.contains(&(x, y)) {
            //     continue;
            // }
            // seen.insert((x, y));
            if score[y as usize][x as usize] <= d {
                continue;
            }
            score[y as usize][x as usize] = d;
            // println!("{}, {}, {}", d, x, y);
            if x == width && y == height {
                can_escape = true;
                break;
            }
            if grid[y as usize][x as usize] == "#" {
                continue;
            }
            heap.push_back((d + 1, x + 1, y));
            heap.push_back((d + 1, x - 1, y));
            heap.push_back((d + 1, x, y + 1));
            heap.push_back((d + 1, x, y - 1));
        }
        if !can_escape {
            return format!("{},{}", ss[simulation - 1].0, ss[simulation - 1].1);
        }
    }
    return "".to_string();
}

fn main() {
    let s: String = fs::read_to_string("./input/18.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-18.txt").unwrap();
    println!("{}", part1(&s));
    println!("{}", part2(&s));
}
