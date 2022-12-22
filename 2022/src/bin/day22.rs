use std::fs;

#[derive(Debug, Clone)]
struct Inst {
    steps: i64,
    dir: char,
}

fn parse_inst(s: &str) -> Vec<Inst> {
    let re = regex::Regex::new(r"(\d+)([LR]?)").unwrap();
    re.captures_iter(s)
        .map(|cap| {
            let (a, b) = (&cap[1], &cap[2]);
            let a = a.parse().unwrap();
            Inst {
                steps: a,
                dir: b.chars().nth(0).unwrap_or(' '),
            }
        })
        .collect()
}

fn main() {
    let s: String = fs::read_to_string("./src/input22.txt").unwrap();
    let (map, instructions) = s.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<char>> = map.split("\n").map(|x| x.chars().collect()).collect();
    // need to patch the map to fill in the gaps.
    patch_map(&mut map);

    let insts: Vec<Inst> = parse_inst(instructions);
    let part1 = score(solve1(&map, &insts));
    dbg!(part1);
    let part2 = score(solve2(map, insts));
    dbg!(part2);
}

fn score((a, b, c): (usize, usize, char)) -> i32 {
    let dir_score = match c {
        'U' => 3,
        'D' => 1,
        'L' => 2,
        'R' => 0,
        _ => todo!(),
    };
    4 * (a as i32 + 1) + 1000 * (b as i32 + 1) + dir_score
}

fn patch_map(map: &mut Vec<Vec<char>>) -> () {
    let length = map.iter().map(|r| r.len()).max().unwrap();
    map.iter_mut().for_each(|x| {
        if x.len() < length {
            x.resize(length, ' ');
        }
    });
}

fn solve2(map: Vec<Vec<char>>, insts: Vec<Inst>) -> (usize, usize, char) {
    let start = map[0]
        .iter()
        .enumerate()
        .find(|(_, x)| *x == (&'.'))
        .unwrap();
    let mut pos = (start.0, 0, 'R');
    for i in insts.iter() {
        for _ in 0..i.steps {
            let (new_pos, can_move) = try_move_cube(&map, pos);
            if !can_move {
                break;
            }
            pos = new_pos;
        }
        pos = (pos.0, pos.1, change_dir(pos.2, i.dir));
    }
    pos
}

fn solve1(map: &Vec<Vec<char>>, insts: &Vec<Inst>) -> (usize, usize, char) {
    let start = map[0]
        .iter()
        .enumerate()
        .find(|(_, x)| *x == (&'.'))
        .unwrap();
    let mut pos = (start.0, 0, 'R');
    for i in insts.iter() {
        for _ in 0..i.steps {
            let (new_pos, can_move) = try_move(&map, pos);
            if !can_move {
                break;
            }
            pos = new_pos;
        }
        pos = (pos.0, pos.1, change_dir(pos.2, i.dir));
    }
    pos
}

fn change_dir(curr_dir: char, change: char) -> char {
    match change {
        'L' => match curr_dir {
            'R' => 'U',
            'L' => 'D',
            'U' => 'L',
            'D' => 'R',
            _ => panic!("Unknown direction"),
        },
        'R' => match curr_dir {
            'R' => 'D',
            'L' => 'U',
            'U' => 'R',
            'D' => 'L',
            _ => panic!("Unknown direction"),
        },
        ' ' => curr_dir,
        _ => panic!("Unknown direction"),
    }
}

fn try_move(map: &[Vec<char>], pos: (usize, usize, char)) -> ((usize, usize, char), bool) {
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    let (x, y, dir) = pos;
    let (x, y) = (x as isize, y as isize);
    let try_new_pos = match dir {
        'R' => (x + 1, y),
        'L' => (x - 1, y),
        'U' => (x, y - 1),
        'D' => (x, y + 1),
        _ => panic!("Unknown direction"),
    };
    // wrap
    let try_new_pos = (
        ((try_new_pos.0 + width) % width) as usize,
        ((try_new_pos.1 + height) % height) as usize,
    );

    dbg!(try_new_pos);
    match map[try_new_pos.1][try_new_pos.0] {
        '#' => (pos, false),
        '.' => ((try_new_pos.0, try_new_pos.1, dir), true),
        ' ' => {
            let (new_pos, can_move) = try_move(map, (try_new_pos.0, try_new_pos.1, dir));
            if can_move {
                (new_pos, true)
            } else {
                (pos, false)
            }
        }
        _ => panic!("Unknown direction{}", &map[try_new_pos.1][try_new_pos.0]),
    }
}

fn try_move_cube(map: &[Vec<char>], pos: (usize, usize, char)) -> ((usize, usize, char), bool) {
    // Idea: Just map the 14 edges of the cube but this will only work for inputs with the same cube net as mine.
    let (x, y, dir) = pos;
    let (x, y) = (x as isize, y as isize);
    let mut try_new_pos = match dir {
        'R' => (x + 1, y, dir),
        'L' => (x - 1, y, dir),
        'U' => (x, y - 1, dir),
        'D' => (x, y + 1, dir),
        _ => panic!("Unknown direction"),
    };

    // Wrap around cases
    try_new_pos = match try_new_pos {
        // The following are 3 corners where it's ambiguous which edges they come from.
        (100, 50, 'R') => (100, 49, 'U'),
        (100, 50, 'D') => (99, 50, 'L'),
        (49, 99, 'U') => (50, 99, 'R'),
        (49, 99, 'L') => (49, 100, 'D'),
        (50, 150, 'R') => (50, 149, 'U'),
        (50, 150, 'D') => (49, 150, 'L'),
        // PS not the best way to iterate through the ledges as this is prone to errors.
        (x, y, _) if y == -1 => {
            if x < 100 {
                (0, 150 + (x - 50), 'R')
            } else {
                (x - 100, 199, 'U')
            }
        }
        (x, y, dir) if y < 50 => {
            if x < 50 {
                (0, 149 - y, 'R')
            } else if x >= 150 {
                (99, 149 - y, 'L')
            } else {
                (x, y, dir)
            }
        }
        (x, y, dir) if y == 50 && x > 100 => {
            assert!(x >= 49);
            if x >= 100 {
                (99, 50 + (x - 100), 'L')
            } else {
                (x, y, dir)
            }
        }
        (x, y, dir) if y == 99 && x < 50 => {
            assert!(x <= 100);
            if x < 50 {
                (50, 50 + x, 'R')
            } else {
                (x, y, dir)
            }
        }
        (x, y, dir) if y < 100 && (x < 50 || x >= 100) => {
            if x < 50 {
                (y - 50, 100, 'D')
            } else if x >= 100 {
                (y - 50 + 100, 49, 'U')
            } else {
                (x, y, dir)
            }
        }
        (x, y, dir) if y >= 100 && y < 150 && (x >= 100 || x < 0) => {
            assert!(x <= 100);
            if x < 0 {
                (50, 49 - (y - 100), 'R')
            } else if x >= 100 {
                (149, 49 - (y - 100), 'L')
            } else {
                (x, y, dir)
            }
        }
        (x, y, dir) if y == 150 && x >= 50 => {
            assert!(x <= 100 || x >= 49);
            if x >= 50 {
                (49, 150 + (x - 50), 'L')
            } else {
                (x, y, dir)
            }
        }
        (x, y, dir) if y >= 150 && y < 199 => {
            assert!(x <= 100);
            if x < 0 {
                (50 + (y - 150), 0, 'D')
            } else if x >= 50 {
                (50 + (y - 150), 149, 'U')
            } else {
                (x, y, dir)
            }
        }
        (x, y, _) if y > 199 => (100 + x, 0, 'D'),
        _ => try_new_pos,
    };

    dbg!(pos, &try_new_pos);
    match map[try_new_pos.1 as usize][try_new_pos.0 as usize] {
        '#' => (pos, false),
        '.' => (
            (
                try_new_pos.0 as usize,
                try_new_pos.1 as usize,
                try_new_pos.2,
            ),
            true,
        ),
        _ => panic!(
            "Unknown direction{}",
            &map[try_new_pos.1 as usize][try_new_pos.0 as usize]
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let s: String = fs::read_to_string("./src/input22.txt").unwrap();
        let (map, _) = s.split_once("\n\n").unwrap();
        let mut map: Vec<Vec<char>> = map.split("\n").map(|x| x.chars().collect()).collect();
        // PS: This is for testing whether I mapped the 14 edges correctly.
        patch_map(&mut map);
        assert!(try_move_cube(&map, (60, 0, 'U')).0 == (0, 160, 'R'));
        assert!(try_move_cube(&map, (110, 0, 'U')).0 == (10, 199, 'U'));

        assert!(try_move_cube(&map, (50, 21, 'L')).0 == (0, 128, 'R'));
        assert!(try_move_cube(&map, (149, 20, 'R')).0 == (99, 129, 'L'));

        assert!(try_move_cube(&map, (120, 49, 'D')).0 == (99, 70, 'L'));

        assert!(try_move_cube(&map, (50, 71, 'L')).0 == (21, 100, 'D'));
        assert!(try_move_cube(&map, (99, 73, 'R')).0 == (123, 49, 'U'));

        assert!(try_move_cube(&map, (20, 100, 'U')).0 == (50, 70, 'R'));

        assert!(try_move_cube(&map, (0, 121, 'L')).0 == (50, 28, 'R'));
        assert!(try_move_cube(&map, (99, 121, 'R')).0 == (149, 28, 'L'));

        assert!(try_move_cube(&map, (80, 149, 'D')).0 == (49, 180, 'L'));

        assert!(try_move_cube(&map, (0, 171, 'L')).0 == (71, 0, 'D'));
        assert!(try_move_cube(&map, (49, 170, 'R')).0 == (70, 149, 'U'));

        assert!(try_move_cube(&map, (30, 199, 'D')).0 == (130, 0, 'D'));
        assert!(try_move_cube(&map, (99, 99, 'R')).0 == (149, 49, 'U'));

        assert!(try_move_cube(&map, (50, 69, 'R')).0 == (51, 69, 'R'));
    }
}
