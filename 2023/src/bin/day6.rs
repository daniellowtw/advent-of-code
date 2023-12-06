use std::fs;

fn score(input: (i64, i64)) -> i64 {
    let mut count = 0;
    for i in 1..input.0 {
        if (input.0 - i) * i > input.1 {
            count += 1;
        }
    }
    // println!("{} {} {}", input.0, input.1, count);
    return count;
}

fn parse(s: String) -> Vec<(i64, i64)> {
    // Example input:
    // Time:        46     80     78     66
    // Distance:   214   1177   1402   1024
    let parts: Vec<&str> = s.split("\n").collect();
    let times: Vec<i64> = parts[0]
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let distances: Vec<i64> = parts[1]
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    return times.into_iter().zip(distances.into_iter()).collect();
}

fn main() {
    let s: String = fs::read_to_string("./src/input6.txt").unwrap();
    let inputs = parse(s);
    // println!("{:?}", inputs);
    // I actually just parsed this by hand when I first solved it.
    // let inputs = vec![(46, 214), (80, 1177), (78, 1402), (66, 1024)];
    let ss = inputs
        .iter()
        .map(|x| score(*x))
        .reduce(|x, y| x * y)
        .unwrap();
    println!("{}", ss);
    let inputs2 = inputs
        .iter()
        .fold((String::new(), String::new()), |acc, (a, b)| {
            return (acc.0 + &a.to_string(), acc.1 + &b.to_string());
        });
    let inputs2 = (
        inputs2.0.parse::<i64>().unwrap(),
        inputs2.1.parse::<i64>().unwrap(),
    );
    // let ss = score1((46807866, 214117714021024));
    let ss = score(inputs2);
    println!("{}", ss);
}
