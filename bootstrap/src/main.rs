use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use aoc_bootstrap::{get_example, get_puzzle_input};
use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum, Debug)]
enum TemplateKind {
    /// Integer parsing template (Vec<Vec<i32>>)
    GridInts,
    /// Integer parsing template (Vec<i32>)
    Ints,
    /// String parsing template (Vec<Vec<&str>>)
    GridStrs,
    /// String parsing template (Vec<&str>)
    Strs,
    /// String parsing template (Vec<Vec<char>>)
    GridChars,
}

#[derive(Parser, Debug)]
#[command(name = "bootstrap")]
#[command(about = "Bootstrap Advent of Code solution files")]
struct Cli {
    year: u32,
    day: u32,
    kind: TemplateKind,
    #[arg(long)]
    force: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let year = cli.year;
    let day = cli.day;
    let kind = cli.kind;
    let force = cli.force;


    let input_dir = Path::new("input");
    fs::create_dir_all(input_dir)?;
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

    if file_path.exists() && !force {
        eprintln!("File {} already exists. Use --force to overwrite.", file_path.display());
        std::process::exit(1);
    }

    let mut file = File::create(&file_path)?;
    let map_fn = match kind {
        TemplateKind::GridInts => "|x| x.split_whitespace().map(|y| y.parse::<i32>().unwrap()).collect()",
        TemplateKind::Ints => "|y| y.parse::<i32>().unwrap()",
        TemplateKind::Strs => "|x| x.to_string()",
        TemplateKind::GridStrs => "|x| x.split_whitespace().collect()",
        TemplateKind::GridChars => "|x| x.chars().collect()",
    };
    let fn_type = match kind {
        TemplateKind::GridInts => "Vec<Vec<i32>>",
        TemplateKind::Ints => "Vec<i32>",
        TemplateKind::Strs => "Vec<&str>",
        TemplateKind::GridStrs => "Vec<Vec<&str>>",
        TemplateKind::GridChars => "Vec<Vec<char>>",
    };
    let template = format!(
            r#"use std::{{env, fs}};

fn part1(pi: {}) -> i32 {{
    return 0;
}}

fn part2(pi: {}) -> i32 {{
    return 0;
}}

fn main() {{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {{
        eprintln!("Usage: [1e, 1, 2e, 2]");
        std::process::exit(1);
    }}

    let sel = args[1].as_str();

    let s: String = match sel {{
        "1e" | "2e" => fs::read_to_string("./input/example-{:02}.txt").unwrap(),
        "1" | "2" => fs::read_to_string("./input/{:02}.txt").unwrap(),
        _ => {{
            eprintln!("Invalid argument. Use 1e, 1, 2e, or 2.");
            std::process::exit(1);
        }}
    }};
    let ss: {} = s.split("\n")
    .filter(|x| !x.is_empty())
    .map({})
    .collect();

    
    if sel == "1e" || sel == "1" {{
        println!("{{}}", part1(ss));
    }} else {{
        println!("{{}}", part2(ss));
    }}
}}
"#,
            fn_type, fn_type, day, day, fn_type, map_fn
        );
    file.write_all(template.as_bytes())?;

    println!("Template created at {}", file_path.display());

    let puzzle_input = get_puzzle_input(year, day).expect("Failed to fetch puzzle input");
    let input_file_path = input_dir.join(format!("{:02}.txt", day));
    fs::write(&input_file_path, puzzle_input)?;

    println!("Input file saved at {}", input_file_path.display());

    Ok(())
}
