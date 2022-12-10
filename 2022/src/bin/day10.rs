use std::fs;

#[derive(Debug)]
enum INST {
    NOP,
    ADDX(i32),
}

impl INST {
    fn new(s: &str) -> Self {
        match &s[..4] {
            "noop" => Self::NOP,
            "addx" => Self::ADDX(s[5..].parse::<i32>().unwrap()),
            _ => panic!("Unknown instruction"),
        }
    }
}

fn main() {
    let s: String = fs::read_to_string("./src/input10.txt").unwrap();
    let ss: Vec<INST> = s.trim_end().split("\n").map(INST::new).collect();
    let score = calculate_part1(&ss);
    println!("Part 1: {}", score);

    calculate_part2(&ss);
}

fn calculate_part1(lines: &Vec<INST>) -> i32 {
    // PS: Probably easier to take the same approach as part 2
    // and split it into one instruction per tick. That Simplifies the indexing logic.
    let mut x = 1;
    let mut step = 1;
    let mut score = 0;
    let mut next_mark = 19; // "During 20" is the same as "After 19"
    for line in lines {
        match line {
            INST::NOP => {
                step += 1;
            }
            INST::ADDX(n) => {
                x += n;
                step += 2;
            }
        }
        if step >= next_mark {
            let extra = x * (next_mark + 1);
            score += extra;
            // dbg!((step, x, next_mark + 1, extra));
            next_mark += 40;
        }
    }
    return score;
}

fn calculate_part2(lines: &Vec<INST>) {
    let mut out: Vec<bool> = Vec::new();
    let single: Vec<INST> = lines
        .iter()
        .flat_map(|x| match x {
            INST::NOP => vec![INST::NOP],
            INST::ADDX(n) => vec![INST::NOP, INST::ADDX(*n)],
        })
        .collect();
    let mut x = 1;
    let mut i = 0;
    for line in single {
        let pixel = ((x - i) as i32).abs() <= 1;
        out.push(pixel);

        i += 1;
        match line {
            INST::NOP => {}
            INST::ADDX(n) => x += n,
        }

        if i >= 40 {
            i -= 40;
        }
    }
    out.chunks(40).for_each(|x| {
        println!(
            "{}",
            x.iter()
                .map(|x| if *x { "#" } else { "." })
                .collect::<String>()
        );
    });
}
