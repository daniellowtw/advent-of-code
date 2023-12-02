use std::cmp::max;
use std::fs;

struct Game {
    id: i32,
    cubes: Vec<(i32, i32, i32)>,
}

fn parse_cubes(s: &str) -> (i32, i32, i32) {
    // Sample input: 3 blue, 4 red
    return s.split(",").map(|x| x.trim()).fold((0, 0, 0), |acc, x| {
        let mut iter = x.split(" ");
        let num = iter.next().unwrap().parse::<i32>().unwrap();
        let color = iter.next().unwrap();
        match color {
            "red" => (acc.0 + num, acc.1, acc.2),
            "green" => (acc.0, acc.1 + num, acc.2),
            "blue" => (acc.0, acc.1, acc.2 + num),
            _ => panic!("Invalid color"),
        }
    });
}

fn parse(s: &str) -> Game {
    // Sample input: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let id = s.split(":").nth(0).unwrap()[5..].parse::<i32>().unwrap();
    let games = s
        .split(":")
        .nth(1)
        .unwrap()
        .split(";")
        .map(|x| parse_cubes(x.trim()));
    return Game {
        id,
        cubes: games.collect(),
    };
}

fn part1(s: Game) -> i32 {
    for (r, g, b) in s.cubes {
        if !(r <= 12 && g <= 13 && b <= 14) {
            return 0;
        }
    }
    return s.id;
}

fn part2(s: Game) -> i32 {
    let min_cubes = s.cubes.into_iter().fold((0, 0, 0), |acc, (r, g, b)| {
        (max(acc.0, r), max(acc.1, g), max(acc.2, b))
    });

    return min_cubes.0 * min_cubes.1 * min_cubes.2;
}

fn main() {
    let s: String = fs::read_to_string("./src/input2.txt").unwrap();
    let ss: i32 = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| part1(parse(x)))
        .sum();
    println!("{}", ss);
    let ss: i32 = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| part2(parse(x)))
        .sum();
    println!("{}", ss);
}
