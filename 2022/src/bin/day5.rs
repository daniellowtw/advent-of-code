use std::fs;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<char>>,
}

impl Board {
    fn play(&mut self, s: &str) {
        let parts: Vec<&str> = s.split(" ").collect();
        let num: i8 = parts[1].parse().unwrap();
        let start: usize = parts[3].parse().unwrap();
        let end: usize = parts[5].parse().unwrap();
        for _ in 0..num {
            let c = self.board[start - 1].pop().unwrap();
            self.board[end - 1].push(c)
        }
    }
    fn play9001(&mut self, s: &str) {
        let parts: Vec<&str> = s.split(" ").collect();
        let num: i8 = parts[1].parse().unwrap();
        let start: usize = parts[3].parse().unwrap();
        let end: usize = parts[5].parse().unwrap();
        // Lazy man's move.
        let mut intermediate = Vec::<char>::new();
        for _ in 0..num {
            let c = self.board[start - 1].pop().unwrap();
            intermediate.push(c);
        }
        for _ in 0..num {
            let c = intermediate.pop().unwrap();
            self.board[end - 1].push(c)
        }
    }
}

fn parse_2d_board(s: &[&str]) -> Board {
    // PS: I guess if I were lazy, I could just hard code this board...
    let mut board = vec![vec![]; 9];
    // Parse rows from bottom to top
    let mut r = s.len() - 1;
    loop {
        let line = s[r];
        let mut chars = line.chars();

        chars.nth(0);
        let mut i = 0;
        loop {
            if let Some(c) = chars.next() {
                if c != ' ' {
                    board[i].push(c);
                }
                i += 1;
                chars.nth(2);
            } else {
                break;
            }
        }
        if r == 0 {
            break;
        } else {
            r -= 1;
        }
    }
    return Board { board: board };
}

fn main() {
    let s: String = fs::read_to_string("./src/input5.txt").unwrap();
    let ss: Vec<&str> = s.trim_end().split("\n").collect();

    let mut board = parse_2d_board(&ss[0..8]);
    for line in &ss[10..] {
        board.play(line);
    }
    for i in board.board {
        print!("{}", i.last().unwrap());
    }
    println!();

    let mut board = parse_2d_board(&ss[0..8]);
    for line in &ss[10..] {
        board.play9001(line);
    }

    for i in board.board {
        print!("{}", i.last().unwrap());
    }
    println!();
}
