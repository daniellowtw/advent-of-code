use std::fs;

fn main() {
    let s: String = fs::read_to_string("./src/input8.txt").unwrap();
    let ss: Vec<&str> = s.trim_end().split("\n").collect();
    let grid: Vec<Vec<i8>> = ss
        .into_iter()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i8).collect())
        .collect();
    let score = calculate_part1(&grid);
    println!("Part 1: {}", score);
    let score = calculate_part2(&grid);
    println!("Part 2: {}", score);
}

fn calculate_part1(grid: &Vec<Vec<i8>>) -> i32 {
    let mut score = 0;
    let num_col = grid[0].len();
    let num_row = grid.len();

    let mut seen: Vec<Vec<bool>> = Vec::new();
    for _ in 0..num_row {
        let row = vec![false; num_col];
        seen.push(row);
    }
    for i in 0..num_row {
        let mut lowest = -1;
        for j in 0..num_col {
            if grid[i][j] > lowest {
                lowest = grid[i][j];
                seen[i][j] = true;
            }
        }
    }
    // From right
    for i in 0..num_row {
        let mut lowest = -1;
        for j1 in 0..num_col {
            let j = num_col - j1 - 1;
            if grid[i][j] > lowest {
                lowest = grid[i][j];
                seen[i][j] = true;
            }
        }
    }
    // From bottom
    for j in 0..num_col {
        let mut lowest = -1;
        for i1 in 0..num_row {
            let i = num_row - i1 - 1;
            if grid[i][j] > lowest {
                lowest = grid[i][j];
                seen[i][j] = true;
            }
        }
    }
    // From top
    for j in 0..num_col {
        let mut lowest = -1;
        for i in 0..num_row {
            if grid[i][j] > lowest {
                lowest = grid[i][j];
                seen[i][j] = true;
            }
        }
    }
    for i in 0..num_row {
        for j in 0..num_col {
            if seen[i][j] {
                score += 1;
            }
        }
    }
    return score;
}

fn calculate_part2(grid: &Vec<Vec<i8>>) -> i32 {
    let mut largest_score = 0;
    let num_col = grid[0].len();
    let num_row = grid.len();

    for i1 in 1..num_row - 1 {
        for j1 in 1..num_col - 1 {
            let highest = grid[i1][j1];
            let mut scores = vec![];
            // to left

            let mut pushed = false;
            let mut score = 0;
            for a in 0..i1 {
                let i = i1 - a - 1;
                score += 1;
                if grid[i][j1] >= highest {
                    scores.push(score);
                    pushed = true;
                    break;
                }
            }
            if !pushed {
                scores.push(score);
            }

            // to right
            pushed = false;
            score = 0;
            for i in i1 + 1..num_row {
                score += 1;
                if grid[i][j1] >= highest {
                    scores.push(score);
                    pushed = true;
                    break;
                }
            }
            if !pushed {
                scores.push(score);
            }

            // to bottom
            pushed = false;
            score = 0;
            for j in j1 + 1..num_col {
                score += 1;
                if grid[i1][j] >= highest {
                    scores.push(score);
                    pushed = true;
                    break;
                }
            }
            if !pushed {
                scores.push(score);
            }
            pushed = false;
            score = 0;

            // to top
            for a in 0..j1 {
                let j = j1 - a -1;
                score += 1;
                if grid[i1][j] >= highest {
                    scores.push(score);
                    pushed = true;
                    break;
                }
            }
            if !pushed {
                scores.push(score);
            }
            assert!(scores.len() == 4);
            let final_score = scores.into_iter().reduce(|a, b| a * b).unwrap();
            if final_score > largest_score {
                largest_score = final_score;
            }
        }
    }
    return largest_score;
}
