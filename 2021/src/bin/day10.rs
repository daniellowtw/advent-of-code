use std::collections::HashMap;
use std::fs;

fn main() {
    let s: String = fs::read_to_string("./src/input10.txt").unwrap();
    let ss: Vec<&str> = s.split("\n").
        filter(|x| { !x.is_empty() }).
        collect(); // collect();

    // When parsing, match the ending with the starting expected in the stack.
    let mut lookup: HashMap<char, (char, i32)> = HashMap::new();
    lookup.insert(')', ('(', 3));
    lookup.insert(']', ('[', 57));
    lookup.insert('}', ('{', 1197));
    lookup.insert('>', ('<', 25137));

    let mut score = 0;

    // part 2
    let mut lookup2: HashMap<char, i32> = HashMap::new();
    lookup2.insert('(', 1);
    lookup2.insert('[', 2);
    lookup2.insert('{', 3);
    lookup2.insert('<', 4);
    let mut all_autocomplete_score: Vec<i64> = Vec::new();

    for x in ss {
        let mut stack: Vec<char> = Vec::new();
        let mut good = true;
        for c in x.chars() {
            let s = lookup.get(&c);
            if s.is_none() {
                stack.push(c);
                continue;
            }
            let (open_char, val) = s.unwrap();
            let top = stack.pop();
            if top.is_none() {
                dbg!("Closing when not open");
                score += val;
                good = false;
                break;
            }
            let top = top.unwrap();
            if top != *open_char {
                println!("Expected {} but got {}", open_char, top);
                score += val;
                good = false;
                break;
            }
        }
        // Part 2: Incomplete lines.
        if good {
            stack.reverse();
            let s = stack.iter().fold(0, |b: i64, x1| {
                b * 5 as i64 + *lookup2.get(x1).unwrap() as i64
            });
            all_autocomplete_score.push(s);
        }
    }

    dbg!(score);

    all_autocomplete_score.sort();
    let middle = all_autocomplete_score.len() / 2;
    let x2 = (&all_autocomplete_score).get(middle).unwrap();
    dbg!(x2, all_autocomplete_score.len()); // Sanity check
}
