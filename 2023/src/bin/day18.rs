use std::fs;

#[derive(Debug)]
struct PuzzleInput {
    lines: Vec<(Direction, i32)>,
    lines2: Vec<(Direction, i32)>,
}

const DEBUG: bool = false;

fn parse(s: String) -> PuzzleInput {
    let map: Vec<(Direction, i32)> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let parts: Vec<&str> = x.trim().split(" ").collect();

            match parts[0] {
                "L" => return (Direction::Left, parts[1].parse::<i32>().unwrap()),
                "R" => return (Direction::Right, parts[1].parse::<i32>().unwrap()),
                "U" => return (Direction::Up, parts[1].parse::<i32>().unwrap()),
                "D" => return (Direction::Down, parts[1].parse::<i32>().unwrap()),
                _ => panic!("Unknown direction"),
            }
        })
        .collect();
    let map2: Vec<(Direction, i32)> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let parts: Vec<&str> = x.trim().split(" ").collect();
            let hex = &parts[2][2..7];
            let dist = i32::from_str_radix(hex, 16).unwrap();
            match &parts[2].chars().nth(7).unwrap() {
                '0' => return (Direction::Right, dist),
                '1' => return (Direction::Down, dist),
                '2' => return (Direction::Left, dist),
                '3' => return (Direction::Up, dist),
                _ => panic!("Unknown direction"),
            }
        })
        .collect();
    return PuzzleInput {
        lines: map,
        lines2: map2,
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn is_inside(i: usize, end: usize, grid: &Vec<Vec<bool>>) -> bool {
    let mut inside = false;
    let mut j: usize = 0;
    while j < end {
        if grid[i][j] == true {
            if grid[i - 1][j] && grid[i + 1][j] {
                inside = !inside;
            } else if grid[i - 1][j] {
                // Have an up wall
                loop {
                    j += 1;
                    // Try match up
                    if grid[i][j] == true && grid[i - 1][j] == true {
                        break;
                    } else if {
                        // Try match down
                        grid[i][j] == true && grid[i + 1][j] == true
                    } {
                        inside = !inside;
                        break;
                    }
                }
            } else if grid[i + 1][j] {
                loop {
                    j += 1;
                    // Try match up
                    if grid[i][j] == true && grid[i - 1][j] == true {
                        inside = !inside;
                        break;
                    } else if grid[i][j] == true && grid[i + 1][j] == true {
                        break;
                        // Try match down
                    }
                }
            } else {
                // Nothing up and down
            }
        } else {
        }
        j += 1;
    }
    return inside;
}

fn part1(lines: &Vec<(Direction, i32)>, size: (usize, usize), start: (usize, usize)) -> i32 {
    // I got really lazy here and saw that it can use the same idea as day 10, even with the inefficient is_inside function.
    // So I tested a few sizes and saw that a 700x700 grid contains the input, so a brute force solution reusing the function works.
    let mut grid: Vec<Vec<bool>> = vec![vec![false; size.0]; size.1];
    let mut curr = start;
    grid[curr.0][curr.1] = true;
    for i in lines {
        match i.0 {
            Direction::Left => {
                for _ in 0..i.1 {
                    curr.1 -= 1;
                    grid[curr.0][curr.1] = true;
                }
            }
            Direction::Right => {
                for _ in 0..i.1 {
                    curr.1 += 1;
                    grid[curr.0][curr.1] = true;
                }
            }
            Direction::Up => {
                for _ in 0..i.1 {
                    curr.0 -= 1;
                    grid[curr.0][curr.1] = true;
                }
            }
            Direction::Down => {
                for _ in 0..i.1 {
                    curr.0 += 1;
                    grid[curr.0][curr.1] = true;
                }
            }
        }
    }

    if DEBUG {
        for i in grid.iter() {
            for j in i.iter() {
                if *j == true {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == true {
                count += 1;
            } else {
                if is_inside(i, j, &grid) {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn combine(xs: Vec<(i32, i32)>, x: (i32, i32)) -> (Vec<(i32, i32)>, i64) {
    // PS: Definitely not by best work.

    // This is the trickiest bit. The following should cover all cases.
    //                  OUTSIDE
    //        xxxx                 ( disjoint )
    //        .  .    xxxxxxxxxx
    //        .  .    . INSIDE .
    //        .  xxxxxx        .
    //   xxxxxx           xxx  xxx  ( partially extend, intersect internally)
    //   .                .O.    .
    //   xxxx             . .    .  ( reduce partially)
    //      .             . . xxxx  ( reduce partially)
    //      .             . . .
    //      .             . xxx
    //      .             .
    //      xxxxxxxxxxxxxxx         ( cancel out )
    // assume sorted
    let mut idx = 0;
    let mut score_for_row = 0;
    let mut x = Some(x);
    let mut result: Vec<(i32, i32)> = vec![];
    while idx < xs.len() {
        if x.is_none() {
            result.push(xs[idx]);
            idx += 1;
            continue;
        }
        let segment = x.unwrap();
        let existing = xs[idx];
        if existing.1 < segment.0 {
            // Case don't intersect
            result.push(existing);
        } else if existing.0 > segment.1 {
            // Case don't intersect
            result.push(existing);
        } else if existing.0 == segment.0 && existing.1 == segment.1 {
            // Case cancel out
            x = None;
            score_for_row = (existing.1 - existing.0) as i64 + 1;
        } else if existing.0 < segment.0 && existing.1 > segment.1 {
            // Case intersect partially internally
            result.push((existing.0, segment.0));
            result.push((segment.1, existing.1));
            x = None;
            score_for_row += (segment.1 - segment.0 - 1) as i64;
        } else if segment.1 == existing.0 {
            // Case new segment right match -> extend
            x = Some((segment.0, existing.1));
        } else if segment.0 == existing.1 {
            // Case new segment left match -> extend
            x = Some((existing.0, segment.1));
        } else if segment.1 == existing.1 {
            // Case new segment right match -> reduce
            result.push((existing.0, segment.0));
            score_for_row += (segment.1 - segment.0) as i64;
            x = None
        } else if segment.0 == existing.0 {
            // Case new segment left match -> reduce
            result.push((segment.1, existing.1));
            score_for_row += (segment.1 - segment.0) as i64;
            x = None
        } else {
            println!("{:?} {:?}", segment, existing);
            panic!("Unknown case");
        }
        idx += 1;
    }
    match x {
        Some(x) => {
            result.push(x);
        }
        None => (),
    }

    for i in &result {
        score_for_row += (i.1 - i.0) as i64 + 1;
    }

    return (result, score_for_row);
}

fn part2(lines: &Vec<(Direction, i32)>) -> i64 {
    // Strategy: we go from top to bottom and only keep track of the horizontal segments,
    // we only need to iterate over the number of rows with horizontal segments, which is at most half of the total number of rows.
    // A segment here records the "covered" segment.
    // Then we only need to keep track of how the nubmer of horizontal segment changes over time.
    // We will note that the new edges have only a finite number of ways to combine with the existing segments.
    // So then the score is simply Sum(RowScore + Sum(Active segment * Height)).
    let mut curr = (0, 0);
    let mut segments: Vec<(i32, (i32, i32))> = vec![];
    for (dir, length) in lines {
        match dir {
            Direction::Left => {
                curr.1 -= length;
                segments.push((curr.0, (curr.1, curr.1 + length)))
            }
            Direction::Right => {
                curr.1 += length;
                segments.push((curr.0, (curr.1 - length, curr.1)))
            }
            Direction::Up => {
                curr.0 -= length;
            }
            Direction::Down => {
                curr.0 += length;
            }
        }
    }
    assert!(curr == (0, 0), "The edges should result in a loop");

    // Sort by y value, because then we can be sure that the initial segment tracks when we are inside the polygon.
    segments.sort_by(|a, b| a.0.cmp(&b.0));
    let mut area: i64 = 0;
    let mut y = segments[0].0;
    let mut current_segments: Vec<(i32, i32)> = vec![];
    for candidate in segments {
        // emit volumes
        let height = candidate.0 - y;
        y = candidate.0;
        for i in &current_segments {
            area += (height as i64 - 1) * (i.1 - i.0 + 1) as i64;
        }
        let (new_segments, score_for_row) = combine(current_segments, candidate.1);
        current_segments = new_segments;
        area += score_for_row;
        // Probably not needed, but I want to make sure we always combine from smallest x to largest x.
        current_segments.sort_by(|a, b| a.0.cmp(&b.0));
    }
    assert!(
        current_segments.len() == 0,
        "The last segment should result in an empty active segment list."
    );

    return area;
}

fn main() {
    let s: String = fs::read_to_string("./src/input18.txt").unwrap();
    let pi = parse(s.trim().to_string());
    let ans: i32 = part1(&pi.lines, (700, 700), (300, 300));
    println!("{}", ans);
    let ans = part2(&pi.lines2);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
    R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_part1_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim().to_string());
        let ans = part1(&pi.lines, (20, 20), (2, 2));
        println!("{}", ans);
        assert!(ans == 62, "got {}", ans);
    }

    #[test]
    fn test_part2_sample_input_produces_same_result() {
        let pi = parse(SAMPLE_INPUT.trim().to_string());
        let ans = part2(&pi.lines);
        assert!(ans == 62, "got {}", ans);
    }

    #[test]
    fn test_part2_sample_input_large_output() {
        let pi = parse(SAMPLE_INPUT.trim().to_string());
        let ans = part2(&pi.lines2);
        assert!(ans == 952408144115, "got {}", ans);
    }
}
