use std::{collections::HashMap, fs};

fn parse(s: String) -> Vec<(String, i32)> {
    let parts: Vec<(String, i32)> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let tmp: Vec<&str> = x.split(" ").collect();
            return (tmp[0].to_string(), tmp[1].parse::<i32>().unwrap());
        })
        .collect();
    return parts;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn cards_to_rank(cards: String) -> Rank {
    // Strategy:
    // 1. Count cards
    // 2. Map to rank by exhaustion
    let cards: Vec<char> = cards.chars().collect();
    let mut card_count: HashMap<char, i32> = HashMap::new();
    for c in cards {
        let count = card_count.entry(c).or_insert(0);
        *count += 1;
    }
    let card_count: Vec<i32> = card_count.values().map(|x| *x).collect();
    match card_count.len() {
        1 => return Rank::FiveOfAKind,
        2 => {
            // Only possible cases:
            // 4, 1
            // 3, 2
            if card_count[0] == 2 || card_count[0] == 3 {
                return Rank::FullHouse;
            } else {
                return Rank::FourOfAKind;
            }
        }
        3 => {
            // Only possible cases:
            // 3, 1, 1
            // 2, 2, 1
            if card_count[0] == 3 || card_count[1] == 3 || card_count[2] == 3 {
                return Rank::ThreeOfAKind;
            } else {
                return Rank::TwoPairs;
            }
        }
        4 => return Rank::OnePair,
        _ => return Rank::HighCard,
    }
}

fn part1(input: &Vec<(String, i32)>) -> i64 {
    // Strategy:
    // 1. Map hand to rank
    // 2. Sort by rank
    // 3. Tie break by card order
    let mut input = input.clone();
    let card_order = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    input.sort_by(|(a, _), (c, _)| {
        let r1 = cards_to_rank(a.clone());
        let r2 = cards_to_rank(c.clone());
        if r1 != r2 {
            return r1.cmp(&r2);
        }
        // Tie breaker
        let a: Vec<char> = a.chars().collect();
        let c: Vec<char> = c.chars().collect();

        for i in 0..5 {
            let a = a[i];
            let c = c[i];
            let a = card_order.iter().position(|x| x == &a).unwrap();
            let c = card_order.iter().position(|x| x == &c).unwrap();
            if a != c {
                return a.cmp(&c);
            }
        }
        panic!("Tie breaker failed");
    });

    let mut score: i64 = 0;
    let mut multiplier = input.len();
    for (_, v) in input {
        score += v as i64 * multiplier as i64;
        multiplier -= 1;
    }
    return score;
}

fn cards_to_rank_with_joker(cards: String) -> Rank {
    let joker_candidates = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    let mut best_rank = cards_to_rank(cards.clone());
    for i in 0..joker_candidates.len() {
        let new_cards = cards.replace("J", &joker_candidates[i].to_string());
        // println!("{} -> {}", cards, new_cards);
        let rank = cards_to_rank(new_cards);
        // ORDER IS REVERSED!
        if rank.cmp(&best_rank) == std::cmp::Ordering::Less {
            best_rank = rank;
        }
    }
    // println!("{} -> {:?}", cards, &best_rank);
    return best_rank;
}

fn part2(input: &Vec<(String, i32)>) -> i64 {
    // Strategy:
    // 1. Replace J with all possible cards
    // 2. Sort by rank
    // 3. Tie break by card order
    // 4. Score

    let mut input = input.clone();
    let card_order = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    input.sort_by(|(a, _), (c, _)| {
        let r1 = cards_to_rank_with_joker(a.clone());
        let r2 = cards_to_rank_with_joker(c.clone());
        if r1 != r2 {
            return r1.cmp(&r2);
        }
        // Tie breaker
        let a: Vec<char> = a.chars().collect();
        let c: Vec<char> = c.chars().collect();

        for i in 0..5 {
            let a = a[i];
            let c = c[i];
            let a = card_order.iter().position(|x| x == &a).unwrap();
            let c = card_order.iter().position(|x| x == &c).unwrap();
            if a != c {
                return a.cmp(&c);
            }
        }
        panic!("Tie breaker failed");
    });

    // Debug
    // input
    //     .iter()
    //     .map(|x| (x, cards_to_rank_with_joker(x.0.clone())))
    //     .for_each(|x| {
    //         println!("{:?}", x);
    //     });

    let mut score: i64 = 0;
    let mut multiplier = input.len();
    for (_, v) in input {
        score += v as i64 * multiplier as i64;
        multiplier -= 1;
    }
    return score;
}

fn main() {
    let s: String = fs::read_to_string("./src/input7.txt").unwrap();
    let inputs = parse(s);
    let ans1 = part1(&inputs);
    println!("{}", ans1);
    let ans2 = part2(&inputs);
    println!("{}", ans2);
}
