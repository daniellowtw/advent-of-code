use std::fs;
use std::str::FromStr;
use strum_macros::EnumString;

// I want to practise using enums tho this problem can easily have an inline solution.
// https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string
// Using macro go generate some helpers.
#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
enum Dir {
    Forward,
    Down,
    Up,
}

struct Instruction {
    direction: Dir,
    val: i64,
}

impl Instruction {
    pub fn new(direction: Dir, val: i64) -> Self {
        Instruction { direction, val }
    }
}

fn main() {
    let s: String = fs::read_to_string("./src/input2.txt").unwrap();
    let instructions = s.split("\n").filter(|x| !x.is_empty()).map(|x| {
        let xs: Vec<&str> = x.split(" ").collect();
        // println!("{:?}", xs);
        Instruction::new(Dir::from_str(xs[0]).unwrap(), xs[1].parse().unwrap())
    });

    let ans = instructions.clone().fold((0, 0), |acc, b| {
        let x = match b.direction {
            Dir::Down => (acc.0 + b.val, acc.1),
            Dir::Up => (acc.0 - b.val, acc.1),
            Dir::Forward => (acc.0, acc.1 + b.val),
        };
        x
    });
    println!("{} * {} = {}", ans.0, ans.1, ans.0 * ans.1);

    // Part 2. Time to write some better quality code...
    #[derive(Debug)]
    struct State {
        depth: i64,
        aim: i64,
        dist: i64,
    }

    let ans2: State = instructions.fold(
        State {
            aim: 0,
            depth: 0,
            dist: 0,
        },
        |acc, b| match b.direction {
            Dir::Down => State {
                aim: acc.aim + b.val,
                ..acc
            },
            Dir::Up => State {
                aim: acc.aim - b.val,
                ..acc
            },
            Dir::Forward => State {
                dist: acc.dist + b.val,
                depth: (acc.depth + acc.aim * b.val),
                ..acc
            },
        },
    );
    println!("{:?}. Ans: {}", &ans2, ans2.depth * ans2.dist)
}
