use std::fs;

#[derive(Clone)]
enum Slot {
    Num(i64, i8),
    // This type is for part 2. Part one only requires the second element which keeps track of how big the space is.
    // For part 2, I wanted a data structure such that when I modify the array of slot, it is done in place, without moving elements.
    // So this data structure mimics a collision list. Because we know the maximum number of space is 9, we can use a fixed size array
    // of size 9 to cover the extreme case where we fill in 9 blocks of 1 (I reduced this to 5 with no issues for my input).
    // The three elements here are: The linked list, the space left, and the index of the linked list that is filled.
    Space([(i64, i8); 9], i8, usize),
}

fn part1(slots: &Vec<Slot>) -> i64 {
    let mut final_arr: Vec<Slot> = Vec::new();

    let mut candidate_idx: usize = slots.len() - 1;
    let mut candidate: Slot = (&slots[candidate_idx]).clone();

    // The idea is to keep a pointer to the block that we want to move
    // This block is mutable for cases where we only partially fill the space.
    // Then build out the final array from the left.
    for (idx, i) in slots.iter().enumerate() {
        // This is the terminal condition. This means there is no more gap left to fill.
        if idx > candidate_idx {
            break;
        }
        if idx == candidate_idx {
            final_arr.push(candidate.clone());
            continue;
        }
        match i {
            Slot::Num(_, _) => final_arr.push(i.clone()),
            Slot::Space(_, space_left, _) => {
                let mut num_to_fill = *space_left;
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

    return calculate_score(final_arr);
}

fn part2(slots: &Vec<Slot>) -> i64 {
    // My original solution basically called a lot of .remove() and .insert() on this vector.
    // That resulted a runtime of about 4.5s. With this new solution, I chose the datastructure to avoid these operations.
    let mut final_arr: Vec<Slot> = slots.clone();
    // candidate_val tracks the block that we want to move. We can derive the index to find this block easily.
    let mut candidate_val: usize = slots.len() / 2;
    while candidate_val > 1 {
        // The idea is simple, we locate the candidate, figure out the size of the block.
        // Then we iterate to find a slot that fits.
        // The key idea is we can update that slot keeping the indices the same by adding to the array within the slot.
        let curr_pos = candidate_val * 2;
        let (v, need) = match &final_arr[curr_pos] {
            Slot::Num(v, need) => (*v, *need),
            _ => panic!("should not happen {}", curr_pos),
        };

        // Find a space that fits it
        let mut pos_option: Option<usize> = None;
        for i in (1..curr_pos).step_by(2) {
            let x = &final_arr[i];
            match x {
                Slot::Num(_, _) => continue,
                Slot::Space(_, x, _) => {
                    if *x >= need {
                        pos_option = Some(i);
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }

        if let Some(pos) = pos_option {
            // println!("Found a place to move {}  to {} {:?}", val, pos, s);
            // Remove the candidate because we can move!
            final_arr[curr_pos] = Slot::Space([(0, 0); 9], need, 0);
            let current_val = &final_arr[pos];
            match current_val {
                Slot::Space(mut freq, space, idx) => {
                    freq[*idx] = (v, need);
                    final_arr[pos] = Slot::Space(freq, space - need, idx + 1);
                }
                _ => {
                    panic!("should not happen");
                }
            }
            // dbg!(&final_arr);
        }
        candidate_val -= 1;
    }

    let score = calculate_score(final_arr);
    return score;
}

fn calculate_score(final_arr: Vec<Slot>) -> i64 {
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
            Slot::Space(freq, left, length) => {
                // Small optimization
                if length == &0 {
                    idx += *left as i64;
                    continue;
                }

                for j in 0..*length {
                    let (num, freq) = freq[j];
                    for _ in 0..freq {
                        score += num * idx;
                        idx += 1;
                    }
                }
                idx += *left as i64;
            }
        }
    }
    score
}

fn main() {
    let s: String = fs::read_to_string("./input/09.txt").unwrap();
    // let s: String = fs::read_to_string("./input/example-09.txt").unwrap();
    let ss: Vec<i8> = s.trim_end().chars().map(|x| x as i8 - 48).collect();
    let slots: Vec<Slot> = ss
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            if i % 2 == 0 {
                Slot::Num(i as i64 / 2, x)
            } else {
                Slot::Space([(0, 0); 9], x, 0)
            }
        })
        .collect();
    println!("{}", part1(&slots));
    println!("{}", part2(&slots));
    // Both parts ran in 0.5s when compiled with `cargo build --bin day09`
    // Both parts ran in 0.2 when compiled with `cargo build --release --bin day09`
    // With `cargo run --bin day09`, it took 1.4s.
}
