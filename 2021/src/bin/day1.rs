use std::fs;

fn main() {
    let s: String = fs::read_to_string("./src/input1.txt").unwrap();
    let ss: Vec<i32> = s.split("\n").
        filter(|x| { !x.is_empty() }).
        map(|x| x.parse().unwrap()).
        collect(); // collect();

    // part 1
    let mut x = 0;
    for i in 0..ss.len() - 1 {
        if ss[i + 1] > ss[i] {
            x += 1;
        }
    }
    println!("{}", x);

    // part 2
    x = 0;
    for i in 0..ss.len() - 3 {
        if ss[i + 3] > ss[i] {
            x += 1;
        }
    }
    println!("{}", x);
}
