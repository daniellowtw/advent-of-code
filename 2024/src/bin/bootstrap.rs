use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use aoc2024::{get_example, get_puzzle_input};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: bootstrap <year> <day>");
        std::process::exit(1);
    }

    let year: u32 = args[1].parse().expect("Year must be a number");
    let day: u32 = args[2].parse().expect("Day must be a number");

    let input_dir = Path::new("input");
    fs::create_dir_all(&input_dir)?;
    let bin_dir = Path::new("src/bin");
    fs::create_dir_all(bin_dir)?;

    match get_example(year, day) {
        Ok(example) => {
            let example_file_path = input_dir.join(format!("example-{:02}.txt", day));
            fs::write(&example_file_path, example)?;
            println!("Example file saved at {}", example_file_path.display());
        }
        Err(e) => eprintln!("Failed to fetch example: {}", e),
    }

    // Create the Rust file for the given day
    let file_name = format!("day{:02}.rs", day);
    let file_path = bin_dir.join(&file_name);

    if file_path.exists() {
        eprintln!("File {} already exists. Aborting.", file_path.display());
        std::process::exit(1);
    }

    let mut file = File::create(&file_path)?;
    let template = format!(
        r#"use std::fs;

fn part1(pi: Vec<Vec<i32>>) -> i32 {{
    return 0;
}}

fn part2(pi: Vec<Vec<i32>>) -> i32 {{
    return 0;
}}

fn main() {{
    // let s: String = fs::read_to_string("./input/{:02}.txt").unwrap();
    let s: String = fs::read_to_string("./input/example-{:02}.txt").unwrap();
    let ss: Vec<Vec<i32>> = s.split("\n")
    .filter(|x| !x.is_empty())
    .map(|x| x.split_whitespace().map(|y| y.parse::<i32>().unwrap()).collect()).collect();
    println!("{{}}", part1(ss));
    // println!("{{}}", part2(ss));
}}
"#,
        day, day
    );
    file.write_all(template.as_bytes())?;

    println!("Template created at {}", file_path.display());

    let puzzle_input = get_puzzle_input(year, day).expect("Failed to fetch puzzle input");
    let input_file_path = input_dir.join(format!("{:02}.txt", day));
    fs::write(&input_file_path, puzzle_input)?;

    println!("Input file saved at {}", input_file_path.display());

    Ok(())
}
