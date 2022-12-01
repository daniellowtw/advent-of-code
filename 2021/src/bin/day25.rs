use std::fs;
use itertools::Itertools;

fn main() {
    let s: String = fs::read_to_string("./src/input25.txt").unwrap();
    let mut ss = s.split("\n").
        filter(|x| !x.is_empty()).
        map(|l| l.split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().next().unwrap())
            .collect_vec())
        .collect_vec(); // collect();

    let num_row = ss.len();
    let num_col = ss.first().unwrap().len();
    dbg!(num_row, num_col);

    let mut step = 0;
    let mut ss_next = ss.clone();
    let mut ss_prev = ss.clone();

    loop {
        // Simulate east moving
        for i in 0..num_row {
            for j in 0..num_col {
                if ss[i][j] != '>' {
                    continue
                }
                let new_j = (j+1) % num_col;
                let can_move = ss[i][new_j] == '.';
                if can_move {
                    *ss_next.get_mut(i).unwrap().get_mut(new_j).unwrap() = '>';
                    *ss_next.get_mut(i).unwrap().get_mut(j).unwrap() = '.';
                }
            }
        }
        ss = ss_next.clone();
        for i in 0..num_row {
            for j in 0..num_col {
                if ss[i][j] != 'v' {
                    continue
                }
                let new_i = (i+1) % num_row;
                let can_move = ss[new_i][j] == '.';
                if can_move {
                    *ss_next.get_mut(new_i).unwrap().get_mut(j).unwrap() = 'v';
                    *ss_next.get_mut(i).unwrap().get_mut(j).unwrap() = '.';
                }
            }
        }
        step += 1;
        if ss_prev == ss_next {
            break;
        }
        ss = ss_next.clone();
        ss_prev = ss_next.clone();
    }

    print_grid(ss_next);
    dbg!(step);
}

fn print_grid(x: Vec<Vec<char>>) {
    for row in x {
        let string = row.iter().collect::<String>();
        println!("{}", string)
    }
    println!()
}