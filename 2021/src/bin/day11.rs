use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::fs;

struct Node {
    val: i32,
    // We need something to keep track of whether a node has flashed in the given step/round
    // We could use a boolean and then reset it after each round. but simpler is to keep track
    // of the round so we don't need another pass to reset the flag after each round.
    // We could also just in-line this whole node as a pair of i32 instead.
    last_flashed: i32,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

// Note: I'm still wrapping my head around ownership. Should the Board struct that owns Nodes
// manage the state of Node? Or should it be done by the Node struct?
impl Node {
    fn incr(&mut self) -> i32 {
        self.val += 1;
        self.val
    }

    fn update_flash_and_reset(&mut self, round: i32) {
        self.last_flashed = round;
        self.val = 0;
    }
}

// Abstract over the grid of node and expose simpler functions
struct Board {
    board: Vec<Vec<Node>>,
    num_rows: i32,
    num_cols: i32,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.board.iter()
            .map(|x| {
                let mut start = String::from("");
                x.iter().for_each(
                    |x| start.push_str(&x.val.to_string())
                );
                start
            })
            .collect::<Vec<String>>()
        )
    }
}

impl Board {
    // I learned my mistake from day 9 of having to cast between usize and i32. Hence, this
    // function refactors that and uses the more convenient i32 type in the signature.
    // The tricky part is whether this signature is safe.
    // This says to return a mutable reference to a node?
    // If so, what happens if two threads have access to this ref?
    fn get(&mut self, row: i32, col: i32) -> &mut Node {
        self.board.get_mut(row as usize).unwrap().get_mut(col as usize).unwrap()
    }

    fn new(s: &str) -> Board {
        let ss: Vec<Vec<Node>> = s.split("\n").
            filter(|x| { !x.is_empty() }).
            map(|l| l.split("")
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.parse().unwrap())
                .map(|x| Node { last_flashed: 0, val: x })
                .collect::<Vec<Node>>()).
            collect();
        Board {
            num_rows: (&ss).len() as i32,
            num_cols: (&ss).get(0).unwrap().len() as i32,
            board: ss,
        }
    }

    fn incr_for_round(&mut self, i: i32, j: i32, round: i32) -> bool {
        let x1 = self.get(i, j);
        if x1.last_flashed == round {
            return false;
        }
        let x = x1.incr();
        if x > 9 {
            x1.update_flash_and_reset(round);
            return true;
        }
        return false;
    }

    fn neighbours(&self, i: i32, j: i32) -> Vec<(i32, i32)> {
        let x: Vec<i32> = vec![-1, 0, 1];
        let mut candidates: Vec<(i32, i32)> = Vec::new();
        for k in x.clone() {
            for l in x.clone() {
                if k == 0 && l == 0 {
                    continue;
                }
                let newi = i + k;
                let newj = j + l;
                if newi >= 0 && newi < self.num_rows {
                    if newj >= 0 && newj < self.num_cols {
                        candidates.push((newi, newj))
                    }
                }
            }
        }
        candidates
    }
}

fn main() {
    let s: String = fs::read_to_string("./src/input11.txt").unwrap();
    let mut board = Board::new(&s[..]);

    // println!("{}", board);
    let mut score = 0;
    // 1000 is set for the second part to ensure we iterate enough times.
    // It's interesting to wonder what criteria determine whether a board will have a synchronized
    // flash. Does it always happen? e.g. Not all 3x3 board flashes. 1xN board doesn't work either.
    // Also interesting is what bounds the round which it flashes?
    // I initially wonder if there is a monotonically decreasing metric e.g. For the input size of
    // 10x10, and suppose every 10 round, the relative phase difference of adjacent node decreases
    // by 1, then by 10 * 10 * 10 (the distance of the furthest node) we should be synchronized.
    // That seems to be the case for this input, but I wonder if this reasoning is sound.
    for n in 1..=1000 {
        let mut flash_count = 0; // For part 2

        // Our strategy: First pass increment everything and track things that flashed.
        // Second pass, use a queue to do a bfs of nodes that need to be considered after the flash
        // propagation.
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        // First pass.
        for i in 0..board.num_rows {
            for j in 0..board.num_cols {
                let flashed = board.incr_for_round(i, j, n);
                // Can we reuse this block somehow?
                if flashed {
                    score += 1;
                    flash_count += 1;
                    for pos in board.neighbours(i, j) {
                        queue.push_front(pos);
                    }
                }
            }
        }
        // Second pass.
        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();
            let flashed = board.incr_for_round(i, j, n);
            if flashed {
                for pos in board.neighbours(i, j) {
                    queue.push_front(pos);
                }
                score += 1;
                flash_count += 1;
            }
        }
        if flash_count == board.num_cols * board.num_rows {
            dbg!(n);
            if n > 100 {
                break;
            }
        }
        if n == 100 {
            dbg!(score);
        }
    }
}
