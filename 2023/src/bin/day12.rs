use std::{fs, vec};

#[derive(Debug)]
struct PuzzleInput {
    reports: Vec<(Vec<char>, Vec<usize>)>,
    reports2: Vec<(Vec<char>, Vec<usize>)>, // For part 2
}

fn parse(s: String) -> PuzzleInput {
    let reports: Vec<(Vec<char>, Vec<usize>)> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let parts: Vec<&str> = x.split(" ").collect();
            let a = parts[0].chars().collect();
            let b = parts[1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            return (a, b);
        })
        .collect();
    let reports2 = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let parts: Vec<&str> = x.split(" ").collect();
            let a = [parts[0]; 5].join("?").chars().collect();
            let mut b: Vec<usize> = parts[1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            b = b.repeat(5);
            return (a, b);
        })
        .collect();
    return PuzzleInput { reports, reports2 };
}

fn part1(a: &Vec<char>, b: &Vec<usize>, pos: usize) -> i64 {
    // Strategy:
    // Do a DFS
    let mut a = a.clone();
    if !is_valid(&a, b) {
        return 0;
    }
    if pos == a.len() {
        return 1;
    }
    let mut count = 0;
    if a[pos] == '?' {
        a[pos] = '.';
        count += part1(&a, b, pos + 1);
        a[pos] = '#';
        count += part1(&a, b, pos + 1);
    } else {
        count += part1(&a, b, pos + 1);
    }
    return count;
}

fn is_valid(a: &Vec<char>, b: &Vec<usize>) -> bool {
    let mut b_idx = 0;
    let mut i = 0;
    while i < a.len() {
        if a[i] == '?' {
            return true;
        } else if a[i] == '.' {
            i += 1;
            continue;
        } else if a[i] == '#' {
            if b_idx >= b.len() {
                return false;
            }
            let b_val = b[b_idx];
            for j in 0..b_val {
                if i + j >= a.len() {
                    return false;
                }
                if a[i + j] != '#' && a[i + j] != '?' {
                    return false;
                }
            }
            if i + b_val != a.len() {
                let next = a[i + b_val];
                if next != '.' && next != '?' {
                    return false;
                }
            }
            i += b_val;
            b_idx += 1;
        } else {
            panic!()
        }
    }
    if b_idx != b.len() {
        return false;
    }
    return true;
}

fn part2(a1: &Vec<char>, b: &Vec<usize>) -> i64 {
    // This took me a very long time to figure out.
    // At first I tried to do a DFS like in part 1, but that was too slow, as it visited similar trees too many times.
    // Then I considered trying to group the group of '.' and '?' and count them by chunks i.e. 2.pow(# of ? in chunk).
    // But that also had a huge tree to traverse as well, and wasn't much better than DFS.

    // Finally I noticed that DP could work. But getting the recursion right with all the corner cases was
    // extremely finicky. Even with a paper to visualize the diagram I hit several bugs.

    // Let dp[i][j] = # of valid string from a[0..i] with # patterns defined by b[0..j].
    // The recursion to notice is:
    // if a[i] == '.':, then dp[i][j] = dp[i-1][j]
    // if a[i] == '#':, then we try to fill b[j] such that the last block ends on a[i].
    // This must take into account that the block must have a separator '.' or '?' before it.
    // Hence, it looks something like:
    // a. check that a[i-b[j]] is '.' or '?'
    // b. check that a[i-b[j]+1..=i] is all '#' or '?'
    // Then we can get here exactly by taking the value from dp[i-b[j] - 1][j-1]
    // if a[i] == '?':, then we can do both of the above and sum them

    // insert '.' at the beginning to add a sentinal value so that handling the case for # is easier.
    let mut a = vec!['.'];
    for x in a1 {
        a.push(*x);
    }
    let mut dp = vec![vec![0; b.len()]; a.len()];

    for i in 0..a.len() {
        for j in 0..b.len() {
            match a[i] {
                '.' => {
                    if i > 0 {
                        dp[i][j] += dp[i - 1][j]
                    }
                }
                '#' => {
                    let val = handle_hash_case(i, j, &a, &b, &dp);
                    dp[i][j] += val;
                }
                '?' => {
                    // Do both
                    // As a .
                    if i > 0 {
                        dp[i][j] += dp[i - 1][j]
                    }
                    // As a #
                    let val = handle_hash_case(i, j, &a, &b, &dp);
                    dp[i][j] += val;
                }
                _ => panic!(),
            }
        }
    }

    return dp[a.len() - 1][b.len() - 1];
}

fn handle_hash_case(i: usize, j: usize, a: &Vec<char>, b: &Vec<usize>, dp: &Vec<Vec<i64>>) -> i64 {
    let block_size = b[j];
    let space_needed = block_size + 1;
    // e.g. i = 1 (row 2), space needed = 2
    if i < block_size {
        // Not enough space. Need this to be '.' or '?'
        return 0;
    }
    // Careful of off-by-1
    if a[i - block_size] == '#' {
        // Not enough space. Need this to be '.' or '?'
        // NB: Inserted . at the start of each line to make the maths easier.
        return 0;
    }
    // Enough space
    // Need all to be '#' or '?'
    if !a[i - block_size + 1..=i].iter().all(|x| *x != '.') {
        return 0;
    }

    // Handle first column specially since we can't index into dp[i][j-1]
    if j == 0 {
        if a[0..i - block_size + 1].iter().all(|x| *x != '#') {
            return 1;
        }
    } else {
        if i >= space_needed {
            return dp[i - space_needed][j - 1];
        }
    }
    return 0
}

fn _debug_dp(a: &Vec<char>, b: &Vec<usize>, dp: &Vec<Vec<i64>>) {
    for i in 0..a.len() {
        print!("{} ", a[i]);
        for j in 0..b.len() {
            print!("{} ", dp[i][j])
        }
        println!()
    }
}

fn main() {
    let s: String = fs::read_to_string("./src/input12.txt").unwrap();
    let inputs = parse(s);
    let ans: i64 = inputs
        .reports
        .iter()
        .map(|(a, b)| {
            let count = part1(a, b, 0);
            // println!("{:?}, {:?} -> {}", a, b, count);
            return count;
        })
        .sum();
    println!("{}", ans);
    let ans: i64 = inputs
        .reports2
        .iter()
        .map(|(a, b)| {
            let count = part2(&a, &b);
            // println!("{:?}, {:?} -> {}", a, b, count);
            return count;
        })
        .sum();
    println!("{}", ans);
}
