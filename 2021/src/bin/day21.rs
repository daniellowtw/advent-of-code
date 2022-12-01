use std::collections::HashMap;

fn main() {
    // The example given.
    // let pos1 = 4;
    // let pos2 = 8;

    // Actual input
    // Input is small enough to just type in.
    let pos1 = 8;
    let pos2 = 9;

    part1(pos1, pos2);

    part2(pos1, pos2);
    // 444356092776315
    // 341960390180808
}

fn part2(pos1: i32, pos2: i32) {
    let quantum_die: HashMap<i32, i32> = HashMap::from([
        (3, 1), //111
        (4, 3), //112
        (5, 6), //113,122
        (6, 7), //123,222
        (7, 6),
        (8, 3),
        (9, 1)
    ]);

    // Our strategy is to keep track of the number of universes where the player reach
    // the state {step, pos, score, num_universe}.
    // I'm encoding this where the vectors are the steps. Then the score and pos are the key.
    let mut p1: Vec<HashMap<(i32, i32), i64>> = vec![HashMap::from([((pos1, 0), 1)])];
    let mut p2: Vec<HashMap<(i32, i32), i64>> = vec![HashMap::from([((pos2, 0), 1)])];
    for _i in 0..24 { // Game must end in 24 steps assuming worst case score of 1 each step.
        simulate_quantum(&quantum_die, &mut p1);
        simulate_quantum(&quantum_die, &mut p2);
    }

    let mut score1 = 0;
    for (step, x) in p1.iter().enumerate() {
        for ((_, score), &num) in x {
            if *score >= 21 {
                // Now find the universes where player 2 did not win by the previous step.
                for ((_, score2), num2) in &p2[step - 1] {
                    if *score2 >= 21 {
                        continue;
                    }
                    score1 += num2 * num
                }
            }
        }
    }

    let mut score2 = 0;
    for (step, x) in p2.iter().enumerate() {
        for ((_, score), &num) in x {
            if *score >= 21 {
                // Now find the universes where player 1 did not win by the same step.
                for ((_, score1), num1) in &p1[step] {
                    if *score1 >= 21 {
                        continue;
                    }
                    score2 += num1 * num
                }
            }
        }
    }

    dbg!(score1, score2);
}

fn simulate_quantum(quantum_die: &HashMap<i32, i32>, player: &mut Vec<HashMap<(i32, i32), i64>>) {
    let p1u = player.last().unwrap().clone();
    let mut next_possible_state: HashMap<(i32, i32), i64> = HashMap::new();
    for ((pos, score), num_universe) in p1u {
        if score >= 21 {
            continue;
        }
        for (die_number, num_universe_next) in quantum_die.into_iter() {
            let mut pos = pos + die_number;
            while pos > 10 { pos -= 10 };
            let score = score + pos;
            *next_possible_state.entry((pos, score)).or_insert(0) += num_universe * *num_universe_next as i64;
        }
    }
    player.push(next_possible_state);
}

fn part1(pos1: i32, pos2: i32) {
    let mut pos1 = pos1;
    let mut pos2 = pos2;
    let mut score1 = 0;
    let mut score2 = 0;

    // Trying out writing my own iterator.
    let die = Die::new();
    let mut is_first_player = true;
    let mut ans = 0;
    for (iteration, x) in die.enumerate() {
        let iteration = iteration + 1;
        let x = x % 10;
        if is_first_player {
            pos1 = pos1 + x;
            while pos1 > 10 { pos1 -= 10 };
            score1 += pos1;
            if score1 >= 1000 {
                dbg!(score1, score2, iteration);
                ans = 3 * score2 * iteration as i32;
                break;
            }
        } else {
            pos2 = pos2 + x;
            while pos2 > 10 { pos2 -= 10 };
            score2 += pos2;
            if score2 >= 1000 {
                dbg!(score1, score2, iteration);
                ans = 3 * score1 * iteration as i32;
                break;
            }
        }
        is_first_player = !is_first_player;
    }
    dbg!(ans);
}

struct Die {
    num: i32,
}

impl Die {
    fn new() -> Die {
        Die { num: 1 }
    }
    fn next_one(&mut self) -> i32 {
        let ret = self.num;
        self.num += 1;
        if self.num > 100 {
            self.num = 1
        };
        ret
    }
}

impl Iterator for Die {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.next_one();
        let b = self.next_one();
        let c = self.next_one();
        Some(a + b + c)
    }
}
