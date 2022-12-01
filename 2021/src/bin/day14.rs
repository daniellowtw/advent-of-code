use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

struct State {
    count_map: HashMap<[char; 2], i64>,
    char_count_map: HashMap<char, i64>,
}

fn str_to_char_array(x: &str) -> [char; 2] {
    [x.chars().nth(0).unwrap(), x.chars().nth(1).unwrap()]
}

impl State {
    fn next(&mut self, lookup: &HashMap<[char; 2], char>) -> State {
        let gen = self.count_map.keys().collect::<Vec<&[char; 2]>>();
        let mut next_count_map: HashMap<[char; 2], i64> = HashMap::new();

        for x in gen {
            match lookup.get(x) {
                Some(&c) => {
                    let num_occ_of_pairs = self.count_map.get(x).unwrap();
                    *next_count_map.entry([x[0], c]).or_insert(0) += num_occ_of_pairs;
                    *next_count_map.entry([c, x[1]]).or_insert(0) += num_occ_of_pairs;
                    *self.char_count_map.entry(c).or_insert(0) += num_occ_of_pairs;
                }
                None => {
                    *next_count_map.entry(*x).or_insert(0) += self.count_map.get(x).unwrap();
                }
            }
        }
        State {
            count_map: next_count_map,
            // Maybe there's a better way than cloning.
            char_count_map: self.char_count_map.clone(),
        }
    }
}

fn main() {
    let s: String = fs::read_to_string("./src/input14.txt").unwrap();
    let (initial_string, mappings) = s.split_once("\n\n").unwrap();

    let mut lookup: HashMap<[char; 2], char> = HashMap::new();
    mappings.split("\n")
        .filter(|x| { !x.is_empty() })
        .map(|x| x.split_once(" -> ").unwrap())
        .for_each(|(left, right)| {
            lookup.insert(str_to_char_array(left), right.chars().next().unwrap());
        });

    // Initial state.
    let mut initial_count: HashMap<[char; 2], i64> = HashMap::new();
    for i in 0..initial_string.len() - 1 {
        *initial_count.entry(str_to_char_array(&initial_string[i..i + 2])).or_insert(0) += 1;
    }
    let mut char_count_map: HashMap<char, i64> = HashMap::new();
    for x in initial_string.chars() {
        *char_count_map.entry(x).or_insert(0) += 1;
    }

    // Simulate.
    let mut final_result: State = State {
        count_map: initial_count,
        char_count_map,
    };
    for _x in 1..=40 {
        final_result = final_result.next(&lookup);
    }

    // Read result.
    dbg!(&final_result.char_count_map);
    let counts = final_result.char_count_map.iter().sorted_by(|x1, x2| { x1.1.cmp(x2.1) }).collect_vec();
    let diff = *counts.last().unwrap().1 - *counts.first().unwrap().1;
    dbg!(diff);
}

// This was my first attempt doing what the problem proposes naively. It obviously doesn't work for
// the large dataset.
fn _simulate10(x: &str, lookup: HashMap<&str, char>) -> String {
    let aux = |x: &str| -> String {
        let mut res = String::new();
        for i in 0..x.len() - 1 {
            res.push(x.chars().nth(i).unwrap());
            match lookup.get(&x[i..i + 2]) {
                Some(c) => res.push(*c),
                None => ()
            }
        }
        res.push(x.chars().last().unwrap());
        res
    };
    let mut final_result: String = String::from(x);
    for _x in 1..=10 {
        final_result = aux(&final_result);
// dbg!(x, final_result.len(), &final_result);
    }

    final_result
}
