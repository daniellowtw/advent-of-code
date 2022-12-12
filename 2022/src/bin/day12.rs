use std::{collections::VecDeque, fs};

fn main() {
    let s: String = fs::read_to_string("./src/input12.txt").unwrap();
    let ss: Vec<Vec<char>> = s
        .trim_end()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();

    let mut starts = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    ss.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, val)| match val {
            &'S' => {
                starts.push((r, c));
                start = (r, c);
            }
            &'a' => starts.push((r, c)),
            &'E' => end = (r, c),
            _ => (),
        })
    });

    let part1 = solve_single_path(&ss, start, end);
    dbg!(part1);
    let part2 = starts
        .iter()
        .map(|s| solve_single_path(&ss, *s, end))
        .min()
        .unwrap();
    dbg!(part2);
}

fn solve_single_path(ss: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let (num_rows, num_cols) = (ss.len(), ss[0].len());
    let mut score: Vec<Vec<i32>> = vec![vec![i32::MAX; num_cols]; num_rows];

    score[start.0][start.1] = 0;

    queue.push_back(start);
    while queue.len() > 0 {
        let (r, c) = queue.pop_front().unwrap();
        let mut curr_val = ss[r][c] as i8;
        if curr_val == 'S' as i8 {
            curr_val = 'a' as i8;
        }
        let curr_score = score[r][c];
        let mut found = false;
        get_neighbours(r, c, num_rows, num_cols)
            .iter()
            .for_each(|(r, c)| {
                let (&r, &c) = (r, c);
                let mut candidate_val = ss[r][c] as i8;
                let special = candidate_val == 'E' as i8;
                if special {
                    candidate_val = 'z' as i8;
                }
                if candidate_val - curr_val <= 1 {
                    if score[r][c] > curr_score + 1 {
                        score[r][c] = curr_score + 1;
                        queue.push_back((r, c));
                    }
                    if special {
                        found = true
                    }
                }
            });
        if found {
            break;
        }
    }
    return score[end.0][end.1];
}

fn get_neighbours(r: usize, c: usize, num_rows: usize, num_cols: usize) -> Vec<(usize, usize)> {
    let (r, c, num_rows, num_cols) = (r as isize, c as isize, num_rows as isize, num_cols as isize);
    let res = vec![(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
    return res
        .into_iter()
        .filter(|(r, c)| r >= &0 && r < &num_rows && c >= &0 && c < &num_cols)
        .map(|(x, y)| (x as usize, y as usize))
        .collect();
}
