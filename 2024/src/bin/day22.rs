use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn part1(pi: &Vec<i64>) -> i64 {
    pi.iter()
        .map(|&seed| {
            let mut seed = seed;
            for _ in 0..2000 {
                seed = f(seed);
            }
            seed
        })
        .sum()
}

fn seq(seed: i64) -> [i8; 2001] {
    let mut res = [0_i8; 2001];
    let mut seed = seed;
    res[0] = (seed % 10) as i8;
    for i in 1..=2000 {
        seed = f(seed);
        res[i] = (seed % 10) as i8;
    }
    return res;
}

fn _check(secrets: [i8; 2001], seq: &[i8; 4]) -> i8 {
    // Secrets = hash chain mod 10
    // Seq is the needle we want to find.
    // This finds the value of the seq in this chain.
    for i in 4..=2000 {
        if secrets[i] - secrets[i - 1] == seq[3]
            && secrets[i - 1] - secrets[i - 2] == seq[2]
            && secrets[i - 2] - secrets[i - 3] == seq[1]
            && secrets[i - 3] - secrets[i - 4] == seq[0]
        {
            return secrets[i];
        }
    }
    return 0;
}

fn _check_old(seed: i64, seq: &[i64; 4]) -> i64 {
    // My original implementation without precomputation.
    // The computation of the chain should be cached!
    let mut seed = seed;
    let mut prev = seed % 10;
    let mut changes = VecDeque::new();
    for _ in 0..2000 {
        seed = f(seed);
        let curr = seed % 10;
        let diff = curr - prev;
        changes.push_back(diff);
        if changes.len() > 4 {
            changes.pop_front();
        }
        if changes == seq {
            return curr;
        }
        prev = curr;
    }
    return 0;
}

fn seq_to_4diff_map(seq: [i8; 2001]) -> HashMap<[i8; 4], i8> {
    let mut res = HashMap::new();
    seq.windows(5).for_each(|x| {
        let key = [x[1] - x[0], x[2] - x[1], x[3] - x[2], x[4] - x[3]];
        let value = x[4];
        if !res.contains_key(&key) {
            res.insert(key, value);
        }
    });
    res
}

fn part2(pi: &Vec<i64>) -> i32 {
    // Optimized from 6m+ to 90s using parallelism to 7s using precomputation and pruning search space to <1s using even more parallelism.
    // Optimized from the original part 2 solution:
    // 1. Precompute the diff, don't evaluate on each loop. So for a given seed, precompute the hash chain mod 10, and precompute the diff.
    // 2. Precompute possible values for the diffs so that checking a solution is O(1) instead of O(N) where N is the length of the hash chain.
    // 3. Don't iterate over all 19^4 = 130k possibilities. Instead union all possible keys and iterate only those (40k)
    let pi: Vec<HashMap<[i8; 4], i8>> = pi
        .par_iter()
        .map(|&seed| seq_to_4diff_map(seq(seed)))
        .collect();
    let candidate_sequences: HashSet<&[i8; 4]> = pi.iter().flat_map(|p| p.keys()).collect();

    candidate_sequences
        .par_iter()
        .map(|seq| {
            pi.par_iter()
                .map(|seen| *seen.get(*seq).unwrap_or(&0) as i32)
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn _part2_original(pi: &Vec<i64>) -> i64 {
    let mut max = 0;
    let pi = pi
        .iter()
        .map(|&seed| seq(seed))
        .collect::<Vec<[i8; 2001]>>();
    for i in -9..=9 {
        for j in -9..=9 {
            for k in -9..=9 {
                for l in -9..=9 {
                    if i + j < -9 || i + j > 9 {
                        continue;
                    }
                    if j + k < -9 || j + k > 9 {
                        continue;
                    }
                    if k + l < -9 || k + l > 9 {
                        continue;
                    }

                    let seq = [i, j, k, l];
                    // println!("{:?}", seq);
                    let candidate = pi
                        .par_iter()
                        .map(|&seed| {
                            let ans = _check(seed, &seq);
                            // println!("{:?} -> {}, {:?}", ans, seed, seq);
                            ans as i64
                        })
                        .sum();
                    if candidate > max {
                        max = candidate;
                        println!("{:?} -> {:?}", candidate, seq);
                    }
                }
            }
        }
    }

    return max;
}

fn f(x: i64) -> i64 {
    //     Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
    let x1 = ((x * 64) ^ x) % 16777216;
    let x2 = ((x1 / 32) ^ x1) % 16777216;
    let x3 = ((x2 * 2048) ^ x2) % 16777216;
    return x3;
}

fn main() {
    let s: String = fs::read_to_string("./input/22.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-22.txt").unwrap();
    let ss: Vec<i64> = s
        .trim()
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    println!("{}", part1(&ss));
    println!("{}", part2(&ss));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let mut seed = 123;
        for i in 0..10 {
            seed = f(seed);
            assert_eq!(seed, expected[i]);
        }
    }

    #[test]
    fn test_part2() {
        assert_eq!(_check(seq(123), &[-1, -1, 0, 2]), 6);
    }
    #[test]
    fn test_part2_a() {
        assert_eq!(_check_old(1, &[-2, 1, -1, 3]), 7);
        assert_eq!(_check_old(2, &[-2, 1, -1, 3]), 7);
        assert_eq!(_check_old(3, &[-2, 1, -1, 3]), 0);
        assert_eq!(_check_old(2024, &[-2, 1, -1, 3]), 9);
    }
}
