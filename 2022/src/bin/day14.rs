use core::cmp::{max, min};
use std::fs;

fn main() {
    let s: String = fs::read_to_string("./src/input14.txt").unwrap();
    let tmp: Vec<Vec<(i32, i32)>> = s
        .trim_end()
        .split("\n")
        .map(|x| {
            x.split(" -> ")
                .map(|y| {
                    let (a1, a2) = y.split_once(",").unwrap();
                    let b1 = a1.parse::<i32>().unwrap();
                    let b2 = a2.parse::<i32>().unwrap();
                    (b1, b2)
                })
                .collect()
        })
        .collect();

    let (mut map, offset) = make_map(&tmp, 1, 1, 0);
    let part1 = simulate(&mut map, (500 - offset, 0));
    // print_map(&map);
    dbg!(part1);

    let (mut map, offset) = make_map(&tmp, 1, 1, 2);
    let height = map[0].len();
    for i in 0..map.len() {
        map[i][height - 1] = '#';
    }
    let mut part2 = simulate(&mut map, (500 - offset, 0));
    // print_map(&map);
    let left_height = map[0]
        .iter()
        .fold(0, |acc, x| if x == &'o' { acc + 1 } else { acc });
    let right_height = map[map.len() - 1]
        .iter()
        .fold(0, |acc, x| if x == &'o' { acc + 1 } else { acc });
    for i in 0..left_height {
        part2 += i;
    }
    for i in 0..right_height {
        part2 += i;
    }

    dbg!(part2);
}

fn make_map(
    tmp: &Vec<Vec<(i32, i32)>>,
    left_pad: i32,
    right_pad: i32,
    top_pad: i32,
) -> (Vec<Vec<char>>, usize) {
    let (mut left, mut right, mut top) = (500, 500, 0);
    tmp.iter().for_each(|row| {
        row.iter().for_each(|coord| {
            if coord.0 < left {
                left = coord.0
            }
            if coord.0 > right {
                right = coord.0
            }
            if coord.1 > top {
                top = coord.1
            }
        });
    });
    let mut map = vec![
        vec!['.'; (top + 1 + top_pad) as usize];
        (right - left + 1 + left_pad + right_pad) as usize
    ];
    tmp.iter().for_each(|row| {
        row.iter().reduce(|p1, p2| {
            let (x1, y1) = (p1.0 - left + left_pad, p1.1);
            let (x2, y2) = (p2.0 - left + left_pad, p2.1);
            if x1 == x2 {
                for i in min(y1, y2)..=max(y1, y2) {
                    map[x1 as usize][i as usize] = '#';
                }
            } else {
                for i in min(x1, x2)..=max(x1, x2) {
                    map[i as usize][y1 as usize] = '#';
                }
            }
            p2
        });
    });
    return (map, (left - left_pad) as usize);
}

fn simulate(map: &mut Vec<Vec<char>>, start: (usize, usize)) -> i32 {
    let mut flag = false;
    let mut count = 0;
    loop {
        let (mut x, mut y) = start.clone();
        if flag {
            break;
        }
        loop {
            if map[x][y] == 'o' {
                flag = true;
                break;
            }
            if y + 1 == map[0].len() as usize {
                flag = true;
                break;
            }

            // Try to go down
            if map[x][y + 1] == '.' {
                y += 1;
                continue;
            }
            // Try to go down left
            if x >= 1 {
                if map[x - 1][y + 1] == '.' {
                    x -= 1;
                    y += 1;
                    continue;
                }
            }
            // Try to go down right
            if x + 1 < map.len() as usize {
                if map[x + 1][y + 1] == '.' {
                    x += 1;
                    y += 1;
                    continue;
                }
            }
            // Stop
            map[x][y] = 'o';
            count += 1;
            break;
        }
    }
    return count;
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map[0].len() {
        println!("{}", map.iter().map(|x| x[i]).collect::<String>());
    }
}
