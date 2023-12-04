use std::collections::HashSet;
use std::fs;

struct Card {
    id: i32,
    numbers: HashSet<i32>,
    winning_numbers: HashSet<i32>,
}

impl Card {
    fn score(&self) -> i32 {
        return self.numbers.intersection(&self.winning_numbers).count() as i32;
    }
}

fn parse_card(s: String) -> Card {
    // Example: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let id = s
        .split(":")
        .nth(0)
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let numbers_portion = s.split(":").nth(1).unwrap();
    let numbers_1 = numbers_portion.split("|").nth(0).unwrap().trim_end();
    let numbers_2 = numbers_portion.split("|").nth(1).unwrap().trim_end();

    let numbers = (0..numbers_1.len())
        .step_by(3)
        .map(|i| numbers_1[i..i + 3].trim_start().parse::<i32>().unwrap())
        .collect();

    let winning_numbers = (0..numbers_2.len())
        .step_by(3)
        .map(|i| numbers_2[i..i + 3].trim_start().parse::<i32>().unwrap())
        .collect();

    return Card {
        id,
        numbers,
        winning_numbers,
    };
}

fn score_card_1(c: &Card) -> i32 {
    let score = c.score();
    if score == 0 {
        return 0;
    }
    return 2_i32.pow((score - 1) as u32);
}

fn part_2(cards: &Vec<Card>) -> i32 {
    let mut num_cards: Vec<i32> = Vec::new();
    for _ in 0..cards.len() {
        num_cards.push(1);
    }

    for c in cards {
        let s = c.score();
        let id = c.id;
        for n in (c.id)..(c.id + s) {
            if n >= num_cards.len() as i32 {
                continue;
            }
            num_cards[n as usize] += num_cards[(id - 1) as usize];
        }
    }
    // println!("{:?}", num_cards);
    return num_cards.iter().sum();
}

fn main() {
    let s: String = fs::read_to_string("./src/input4.txt").unwrap();
    let cards: Vec<Card> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| parse_card(x.to_string()))
        .collect();
    let score1: i32 = cards.iter().map(score_card_1).sum();
    println!("{}", score1);

    let score2: i32 = part_2(&cards);
    println!("{}", score2);
}
