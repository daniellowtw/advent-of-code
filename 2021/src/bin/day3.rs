use std::fs;

fn main() {
    // Note: I actually solved this first in golang. This is just me redoing the puzzle with rust.
    // This block is still the owner of the string.
    let s = fs::read_to_string("./src/input3.txt").unwrap();
    let lines: Vec<&str> = s.trim()
        .split("\n").collect();

    // Straight forward implementing what they say.
    let mut gamma: [bool; 12] = [false; 12];
    let mut epsilon: [bool; 12] = [false; 12]; // could also just xor with 111111111111.
    for i in 0..12 {
        // Fail if there's a tie
        if most_common_bit(&lines, i).unwrap() {
            gamma[i as usize] = true
        } else {
            epsilon[i as usize] = true
        }
    }
    let g = parse_bool_array_to_int(&gamma);
    let e = parse_bool_array_to_int(&epsilon);
    let ans = g * e;
    dbg!(ans);

    // Part 2
    let a = filter_lines(lines.clone(), |o: Option<bool>| o.unwrap_or(true)).unwrap();
    let b = filter_lines(lines.clone(), |o: Option<bool>| o.map(|x| !x).unwrap_or(false)).unwrap();
    fn parse_binary_string(x: &str) -> i32 {
        let mut t: [bool; 12] = [false; 12];
        for i in 0..12 {
            t[i] = if x.get(i..i + 1).unwrap() == "1" { true } else { false };
        };
        parse_bool_array_to_int(&t)
    }
    ;
    dbg!(a, b, parse_binary_string(a)*parse_binary_string(b));
}


fn filter_lines<T>(lines: Vec<&str>, should_take_one_at_pos: T) -> Option<&str>
// https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html
// I can't express in this signature what the argument represents.
    where T: Fn(Option<bool>) -> bool
{
    let mut candidate = lines;
    for i in 0..12 {
        let most_common_bit_at_pos = most_common_bit(&candidate, i);
        let should_take_one = should_take_one_at_pos(most_common_bit_at_pos);
        let choose = if should_take_one { "1" } else { "0" };
        // This into_iter() vs iter() is pretty confusing. I had to fight the compiler for a while
        // and only settled for this after some trial and error. I probably should revisit this
        // concept some other time.
        let x1 = candidate
            .into_iter()
            .filter(|x| { (x).get(i..i + 1).unwrap() == choose })
            .collect();
        candidate = x1;
        if candidate.len() == 1 {
            return Some(candidate.first().unwrap());
        }
    }
    None
}

// Returns None when there is a tie.
fn most_common_bit(lines: &Vec<&str>, pos: usize) -> Option<bool> {
    let n = lines.len();
    let mut num_ones = 0;
    for x in lines {
        let x1 = x.get(pos..pos + 1).unwrap();
        if x1 == "1" {
            num_ones += 1;
        }
    }
    // check that there's no ties
    if num_ones * 2 == n {
        return None;
    }
    return Some(num_ones > n / 2);
}

fn parse_bool_array_to_int(arr: &[bool; 12]) -> i32 {
    let mut res = 0;
    for x in 0..arr.len() {
        if arr[x] {
            res = res | 1;
        }
        if x != arr.len() - 1 {
            res = res << 1;
        }
    }
    res
}