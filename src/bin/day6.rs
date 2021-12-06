use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

fn main() {
    let s = fs::read_to_string("./src/input6.txt").unwrap();
    let ns: Vec<i32> = s.trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", &ns.len());

    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value
    // Not 100% clear I understand what the return value of or_insert when the value is some other
    // objects.
    let mut stats = HashMap::new();
    for x in ns {
        let count = stats.entry(x).or_insert(0);
        *count += 1;
    }
    println!("{:?}", stats);

    for i in 0..256 {
        let mut next_stat: HashMap<i32, i64> = HashMap::new();
        for (k, v) in stats {
            match k {
                0 => {
                    *(next_stat).entry(6).or_insert(0) += v;
                    *(next_stat).entry(8).or_insert(0) += v;
                }
                n => {
                    *(next_stat).entry(n - 1).or_insert(0) += v;
                }
            }
        }
        if i == 79 {
            // Part 1
            println!("{:?}", next_stat);
            println!("{:?}", next_stat.iter().fold(0, |x, b| x + b.1));
        }
        stats = next_stat;
    }
    println!("{:?}", stats);
    println!("{:?}", stats.iter().fold(0, |x, b| x + b.1));
}