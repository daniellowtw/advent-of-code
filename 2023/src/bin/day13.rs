use std::fs;

#[derive(Debug)]
struct PuzzleInput {
    patterns: Vec<Vec<Vec<char>>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Ans {
    Row(usize),
    Column(usize),
    NotFound,
}

impl Ans {
    fn score(&self) -> i32 {
        match self {
            Ans::Row(i) => (*i as i32 + 1) * 100,
            Ans::Column(i) => *i as i32 + 1,
            Ans::NotFound => 0,
        }
    }
}

fn parse(s: String) -> PuzzleInput {
    let patterns: Vec<Vec<Vec<char>>> = s
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let single_pattern: Vec<Vec<char>> = x
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.chars().collect())
                .collect();
            return single_pattern;
        })
        .collect();
    return PuzzleInput { patterns };
}

fn is_equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

fn part1(a: &Vec<Vec<char>>) -> i32 {
    // Strategy: just iterate through each row and column and check if they are symmetric.
    return find_symmetry(a, Ans::NotFound).score();
}

fn find_symmetry(a: &Vec<Vec<char>>, prev: Ans) -> Ans {
    // Find reflection in rows
    for i in 0..a.len() - 1 {
        let mut is_row_symmetric = true;
        for offset in 0..=i {
            let row_1 = a.get(i - offset);
            let row_2 = a.get(i + 1 + offset);
            if row_1.is_none() || row_2.is_none() {
                break;
            }
            if !is_equal(row_1.unwrap(), row_2.unwrap()) {
                is_row_symmetric = false;
                break;
            }
        }
        if is_row_symmetric {
            match prev {
                Ans::Row(old_i) => {
                    if old_i == i {
                        continue;
                    } else {
                        return Ans::Row(i);
                    }
                }
                _ => {
                    return Ans::Row(i);
                }
            }
        }
    }
    // Find reflections in column
    for i in 0..a[0].len() - 1 {
        let mut is_symmetric = true;
        for offset in 0..=i {
            if i < offset {
                break;
            }
            if i + 1 + offset >= a[0].len() {
                break;
            }
            let row_1: Vec<char> = a.iter().map(|x| x[i - offset]).collect();
            let row_2: Vec<char> = a
                .iter()
                .map(|x| {
                    let y = x[i + offset + 1];
                    return y;
                })
                .collect();
            if !is_equal(&row_1, &row_2) {
                is_symmetric = false;
                break;
            }
        }
        if is_symmetric {
            match prev {
                Ans::Column(old_i) => {
                    if old_i == i {
                        continue;
                    } else {
                        return Ans::Column(i);
                    }
                }
                _ => {
                    return Ans::Column(i);
                }
            }
        }
    }
    return Ans::NotFound;
}

fn main() {
    let s: String = fs::read_to_string("./src/input13.txt").unwrap();
    let inputs = parse(s);
    let ans: i32 = inputs.patterns.iter().map(|x| part1(x)).sum();
    println!("{}", ans);
    let ans: i32 = inputs.patterns.iter().map(|x| part2(x).score()).sum();
    println!("{}", ans);
}

fn part2(a: &Vec<Vec<char>>) -> Ans {
    // Strategy: Just iterate through each cell,
    // flip it, and see if the answer matches the one from part 1.
    let ans = find_symmetry(a, Ans::NotFound);
    let mut x = a.clone();
    for i in 0..x.len() {
        for j in 0..x[i].len() {
            if x[i][j] == '#' {
                x[i][j] = '.';
                let new_ans = find_symmetry(&x, ans);
                if new_ans != Ans::NotFound {
                    return new_ans;
                }
                x[i][j] = '#';
            } else if x[i][j] == '.' {
                x[i][j] = '#';
                let new_ans = find_symmetry(&x, ans);
                if new_ans != Ans::NotFound {
                    return new_ans;
                }
                x[i][j] = '.';
            } else {
                panic!("Invalid char");
            }
        }
    }
    // Debug purposes
    // for i in 0..x.len() {
    //     for j in 0..x[i].len() {
    //         print!("{}", x[i][j]);
    //     }
    //     println!();
    // }
    // println!("Prev ans {:?}", ans);
    // panic!("No answer found.");
    return Ans::NotFound;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        // Failed test case where it keeps returning the old answer (row before column).
        let input_str = "\
#...###.#..#...
#....#...###..#
#.##.###..###..
#########.#.##.
.#..#.##.###...
.#..#.##.###...
#########.#.##.
";
        let input = parse(input_str.to_string());
        let old_ans = find_symmetry(&input.patterns[0], Ans::NotFound);
        assert!(old_ans == Ans::Row(4));
        let ans = part2(&input.patterns[0]);
        assert!(ans == Ans::Column(2), "{:?}", ans);
    }

    #[test]
    fn test_2() {
        // Test case where the answer is either row 0 or col 0, and I had an off-by-1 error
        let input_str = "\
.#.#.#.##..##.#
.###.#.##..##.#
..###..#....#..
#.......####...
.#......####...
.....##.####.##
.#..#.###..###.
#####..######..
##..###..##..##
";
        let input = parse(input_str.to_string());
        let x = &input.patterns[0];
        let old_ans = find_symmetry(x, Ans::NotFound);
        print!("{:?} -> ", old_ans);
        let new_ans = part2(x);
        assert!(new_ans == Ans::Row(0), "{:?}", new_ans);
    }
}
