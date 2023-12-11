use std::fs;

#[derive(Debug)]
struct PuzzleInput {
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
    original_locations: Vec<(usize, usize)>,
}

fn parse(s: String) -> PuzzleInput {
    let grid: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    let empty_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_, x)| x.iter().all(|y| *y == '.'))
        .map(|(i, _)| i)
        .collect();

    let empty_columns: Vec<usize> = (0..width).fold(vec![], |mut acc, x| {
        for y in 0..height {
            if grid[y][x] != '.' {
                return acc;
            }
        }
        acc.push(x);
        return acc;
    });
    let original_locations: Vec<(usize, usize)> =
        grid.iter().enumerate().fold(vec![], |mut acc, (i, x)| {
            for j in 0..x.len() {
                if x[j] == '#' {
                    acc.push((i, j));
                }
            }
            return acc;
        });
    return PuzzleInput {
        empty_rows,
        empty_columns,
        original_locations,
    };
}

fn part1(pi: &PuzzleInput) -> i64 {
    let translated_locations: Vec<(usize, usize)> = pi
        .original_locations
        .iter()
        .map(|(i, j)| {
            let i = i + pi.empty_rows.iter().filter(|x| *x < i).count();
            let j = j + pi.empty_columns.iter().filter(|x| *x < j).count();
            return (i, j);
        })
        .collect();

    return (0..translated_locations.len()).fold(0, |mut acc, i| {
        for j in i + 1..translated_locations.len() {
            acc += distance_usize(translated_locations[i], translated_locations[j])
        }
        return acc;
    });
}

fn part2(pi: &PuzzleInput) -> i64 {
    let factor: i64 = 1000000;
    let translated_locations: Vec<(i64, i64)> = pi
        .original_locations
        .iter()
        .map(|(i, j)| {
            let i =
                *i as i64 + pi.empty_rows.iter().filter(|x| *x < i).count() as i64 * (factor - 1);
            let j = *j as i64
                + pi.empty_columns.iter().filter(|x| *x < j).count() as i64 * (factor - 1);
            return (i, j);
        })
        .collect();

    return (0..translated_locations.len()).fold(0, |mut acc, i| {
        for j in i + 1..translated_locations.len() {
            acc += distance2(translated_locations[i], translated_locations[j]);
        }
        return acc;
    });
}

fn distance_usize(a: (usize, usize), b: (usize, usize)) -> i64 {
    return distance2((a.0 as i64, a.1 as i64), (b.0 as i64, b.1 as i64));
}

fn distance2(a: (i64, i64), b: (i64, i64)) -> i64 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}

fn main() {
    // Strategy:
    // f([loc], (empty_rows, empty_columns)) -> [translated_loc]
    // g([translated_loc]) -> sum of pairwise dist
    let s: String = fs::read_to_string("./src/input11.txt").unwrap();
    let inputs = parse(s);
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}
