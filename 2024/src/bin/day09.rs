use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Num(i64, i8),
    Space(i8),
}

fn part1(pi: &Vec<i8>) -> i64 {
    if pi.len() % 2 != 1 {
        panic!("should not happen");
    }
    let slots: Vec<Slot> = pi
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            if i % 2 == 0 {
                Slot::Num(i as i64 / 2, x)
            } else {
                Slot::Space(x)
            }
        })
        .collect();

    let mut score: i64 = 0;
    let mut final_arr: Vec<Slot> = Vec::new();

    let mut candidate_idx: usize = pi.len() - 1;
    let mut candidate: Slot = (&slots[candidate_idx]).clone();

    for (idx, &i) in slots.iter().enumerate() {
        if idx > candidate_idx {
            break;
        }
        if idx == candidate_idx {
            final_arr.push(candidate);
            continue;
        }
        match i {
            Slot::Num(_, _) => final_arr.push(i.clone()),
            Slot::Space(num2) => {
                let mut num_to_fill = num2;
                while num_to_fill > 0 {
                    if let Slot::Num(num_val, num_freq) = candidate {
                        if num_to_fill >= num_freq {
                            final_arr.push(Slot::Num(num_val, num_freq));
                            candidate_idx -= 2;
                            candidate = (&slots[candidate_idx]).clone();
                            num_to_fill -= num_freq;
                        } else {
                            final_arr.push(Slot::Num(num_val, num_to_fill));
                            candidate = Slot::Num(num_val, num_freq - num_to_fill);
                            num_to_fill = 0;
                        }
                    } else {
                        panic!("should not happen");
                    }
                }
            }
        }
    }

    // dbg!(&final_arr);
    let mut idx = 0;
    for i in final_arr.iter() {
        match i {
            Slot::Num(num, freq) => {
                for _ in 0..*freq {
                    score += *num as i64 * idx;
                    idx += 1;
                }
            }
            Slot::Space(_) => {
                panic!("should not happen");
            }
        }
    }
    return score;
}

fn part2(pi: &Vec<i8>) -> i64 {
    if pi.len() % 2 != 1 {
        panic!("should not happen");
    }
    let slots: Vec<Slot> = pi
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            if i % 2 == 0 {
                Slot::Num(i as i64 / 2, x)
            } else {
                Slot::Space(x)
            }
        })
        .collect();

    let mut final_arr: Vec<Slot> = slots.clone();
    let mut candidate_val: usize = pi.len() / 2;

    while candidate_val > 2 {
        // Find the target block we want to move
        let (curr_pos, v, need) = final_arr
            .iter()
            .enumerate()
            .find_map(|(idx, x)| match x {
                &Slot::Num(v, need) => {
                    if v == candidate_val as i64 {
                        Some((idx, v, need))
                    } else {
                        None
                    }
                }
                Slot::Space(_) => None,
            })
            .unwrap();

        // Find a space that fits it
        if let Some((pos, space_available)) =
            final_arr.iter().enumerate().find_map(|(idx, x)| match x {
                Slot::Num(_, _) => None,
                &Slot::Space(x) => {
                    if x >= need && idx < curr_pos {
                        Some((idx, x))
                    } else {
                        None
                    }
                }
            })
        {
            // println!("Found a place to move {}  to {} {:?}", val, pos, s);
            // Remove the candidate because we can move!
            final_arr[curr_pos] = Slot::Space(need);
            let remaining = space_available - need;
            final_arr.remove(pos);
            if remaining != 0 {
                final_arr.insert(pos, Slot::Space(remaining));
            }
            final_arr.insert(pos, Slot::Num(v, need));
            // dbg!(&final_arr);
        }
        candidate_val -= 1;
    }

    let mut score: i64 = 0;
    let mut idx = 0;
    for i in final_arr.iter() {
        match i {
            Slot::Num(num, freq) => {
                for _ in 0..*freq {
                    score += *num as i64 * idx;
                    idx += 1;
                }
            }
            Slot::Space(freq) => {
                idx += *freq as i64;
            }
        }
    }
    // dbg!(&final_arr);
    return score;
}

fn main() {
    let s: String = fs::read_to_string("./input/09.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-09.txt").unwrap();
    let ss: Vec<i8> = s.trim_end().chars().map(|x| x as i8 - 48).collect();
    println!("{}", part1(&ss));
    println!("{}", part2(&ss));
}
