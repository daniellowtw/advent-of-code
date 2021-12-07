use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Board {
    rows: [[i32; 5]; 5],
    row_sum: [i32; 5],
    col_sum: [i32; 5],
}

impl Board {
    fn new(five_lines: &[String]) -> Board {
        let mut rows: [[i32; 5]; 5] = [[0; 5]; 5];
        let mut row_sum: [i32; 5] = [0; 5];
        let mut col_sum: [i32; 5] = [0; 5];
        five_lines.iter().enumerate()
            .for_each(|(row, v)| {
                // Parse line
                Vec::from(&v[..])
                    .chunks(3)
                    .enumerate()
                    .for_each(|(col, v)| {
                        // This gave me a huge headache trying to get &[u8] for
                        //individual digits to number.
                        let v = String::from_utf8(v.to_vec()).unwrap()
                            .trim()
                            .parse().unwrap();
                        rows[row][col] = v;
                        row_sum[row] += v;
                        col_sum[col] += v;
                    })
            });
        Board { rows, row_sum, col_sum }
    }

    fn reveal(&mut self, x: i32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.rows[row][col] == x {
                    self.rows[row][col] = -1;
                    self.row_sum[row] -= x + 1;
                    self.col_sum[col] -= x + 1;
                }
            }
        }
    }

    fn check(&self) -> bool {
        [self.row_sum, self.col_sum].iter()
            .map(|x| {
                x.iter().any(|x1| *x1 == -5)
            })
            .any(|x| x)
    }

    fn score(&self) -> i32 {
        let s = self.rows.iter()
            .map(|x| {
                x.iter().filter(|&&y| y > 0).sum::<i32>()
            })
            .sum::<i32>();
        s
    }

    fn simulate(&mut self, x: &Vec<i32>) -> (usize, i32) {
        for (i, x) in x.iter().enumerate() {
            self.reveal(*x);
            if self.check() {
                return (i, self.score() * x);
            }
        }
        return (0, 0);
    }
}

fn main() {
    // Note: I actually solved this first in golang. This is just me redoing the puzzle with rust.
    let s = File::open("./src/input4.txt").unwrap();
    let r = BufReader::new(s);
    let mut ss: Vec<String> = r.lines()
        .filter_map(|x| x.ok())
        .filter(|x| !x.is_empty())
        .collect();

    let numbers = ss.remove(0);
    let numbers: Vec<i32> = numbers
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    // Pretty cool API!
    let mut boards: Vec<Board> = ss.chunks(5)
        .map(move |x| Board::new(x))
        .collect();

    // The crux here is to use iter_mut because the function mutates the map!
    let full_play = boards.iter_mut().map(|x| x.simulate(&numbers));

    // I cannot fold this twice :( So I have to do them in the same pass.
    // I'm really fighting the compiler for this problem.
    // let part1 = f1.fold(((9999, 0),(0,0)) |old @ ((earliest, earliest_score), (latest, latest_score)), (win, score)| {
    //     if win < earliest { (win, score) } else { old }
    // });
    // dbg!(part1);
    let ans = full_play
        .fold(
            ((9999, 0), (0, 0)),
            |old, (win, score)| {
                let fst = if win < old.0.0 { (win, score) } else { old.0 };
                let snd = if win > old.1.0 { (win, score) } else { old.1 };
                (fst, snd)
            });
    dbg!(ans);
}