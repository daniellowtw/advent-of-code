use std::{
    collections::{HashSet, VecDeque},
    fs, vec,
};

// PS: This is a very slow solution (4min for each part). TBH I'm still not
// too sure where I can optimize.
// My first solution was creating the State struct and then generating next
// states from that. This was very slow. I then tried to use a tuple instead
// thinking that it's due to the excessive cloning.

type State2 = (i32, i32, i32, i32, i32, i32, i32, i32, i32);

#[allow(unused)]
fn potential2(state: &State2, blueprint: &Blueprint) -> i32 {
    let (ore, ore_bot, clay, clay_bot, obsidian, obsidian_bot, geode, geode_bot, time) = *state;
    // PS: Some attempt to bring down the max potential to prune the search tree even more, but the following doesn't
    // quite work as it actually increases the potential
    // let mut max_ore = ore;
    // for i in ore_bot..(ore_bot + time) {
    //     max_ore += i;
    // }

    // let interval = max_ore / blueprint.clay as i32;
    // let mut max_clay = clay;
    // for i in clay_bot..(clay_bot + interval - 1) {
    //     max_clay += i;
    // }

    // let mut max_obsidian = obsidian;
    // let interval = max_clay / blueprint.obsidian.1 as i32;
    // for i in obsidian_bot..(clay_bot + interval - 1) {
    //     max_obsidian += i;
    // }

    // let interval = max_obsidian / blueprint.geode.1 as i32;
    let mut max = geode;
    for i in geode_bot..(geode_bot + time - 1) {
        max += i;
    }
    max
}
fn add(state: &mut State2, other: &State2) -> () {
    let (ore, _, clay, _, obsidian, _, geode, _, time) = state;
    let (_, ore_bot, _, clay_bot, _, obsidian_bot, _, geode_bot, _) = other;
    *time -= 1;
    *ore += ore_bot;
    *clay += clay_bot;
    *obsidian += obsidian_bot;
    *geode += geode_bot;
}

fn maximize(blueprint: &Blueprint, time: i32) -> i32 {
    let mut seen: HashSet<State2> = HashSet::new();
    let start: State2 = (0, 1, 0, 0, 0, 0, 0, 0, time);
    seen.insert(start);
    let mut queue: VecDeque<State2> = VecDeque::new();
    queue.push_back(start.clone());

    let mut score_max: i32 = 0;
    loop {
        if queue.is_empty() {
            break;
        }
        let s: State2 = queue.pop_back().unwrap();
        if s.8 == 0 {
            if s.6 > score_max {
                score_max = s.6;
                dbg!(s, score_max);
            }
        } else {
            // PS: I think potential2 can be more aggressive here to prune the search tree.
            let max = potential2(&s, blueprint);
            if max < score_max {
                continue;
            }
            for s in possible_next_states2(&s, blueprint).into_iter().rev() {
                if seen.contains(&s) {
                    continue;
                }
                if potential2(&s, blueprint) >= score_max {
                    queue.push_back(s);
                    seen.insert(s);
                }
            }
        }
    }
    return score_max;
}

fn main() {
    let s: String = fs::read_to_string("./src/input19.txt").unwrap();
    let re_obsidian = regex::Regex::new(r".*?(\d+) ore and (\d+) clay").unwrap();
    let re_geode = regex::Regex::new(r".*?(\d+) ore and (\d+) obsidian").unwrap();
    let blueprints: Vec<Blueprint> = s
        .trim_end()
        .split("\n")
        .map(|x| {
            let (_, rest) = x.split_once(":").unwrap();
            let parts: Vec<&str> = rest.split(".").map(|x| x.trim()).collect();
            let o = parts[0].chars().nth("Each ore robot costs ".len()).unwrap() as u8 - '0' as u8;
            let c = parts[1]
                .chars()
                .nth("Each clay robot costs ".len())
                .unwrap() as u8
                - '0' as u8;
            let ob = re_obsidian.captures(parts[2]).unwrap();
            let g = re_geode.captures(parts[3]).unwrap();
            Blueprint {
                clay: c,
                ore: o,
                obsidian: (ob[1].parse::<u8>().unwrap(), ob[2].parse::<u8>().unwrap()),
                geode: (g[1].parse::<u8>().unwrap(), g[2].parse::<u8>().unwrap()),
            }
        })
        .collect();

    let part1 = solve1(&blueprints);
    dbg!(part1);
    let part2 = solve2(&blueprints[..3]);
    dbg!(part2);
}

fn solve1(blueprint: &[Blueprint]) -> i32 {
    blueprint
        .iter()
        .enumerate()
        .map(|(x, b)| {
            dbg!(x, &b);
            let best = maximize(&b, 24);
            dbg!(&best);
            (x + 1) as i32 * best
        })
        .fold(0, |acc, x| acc + x)
}

fn solve2(blueprint: &[Blueprint]) -> i32 {
    blueprint
        .iter()
        .enumerate()
        .map(|(x, b)| {
            let s = maximize(&b, 32);
            dbg!(x, &b, s);
            s
        })
        .fold(1, |acc, x| acc * x)
}

fn possible_next_states2(state: &State2, blueprint: &Blueprint) -> Vec<State2> {
    let mut res = vec![];
    if state.0 >= blueprint.geode.0 as i32 && state.4 >= blueprint.geode.1 as i32 {
        // make geode bot
        let mut tmp = (
            state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7, state.8,
        );
        tmp.7 += 1;
        tmp.0 -= blueprint.geode.0 as i32;
        tmp.4 -= blueprint.geode.1 as i32;
        add(&mut tmp, state);
        res.push(tmp);
    }

    if state.0 >= blueprint.obsidian.0 as i32 && state.2 >= blueprint.obsidian.1 as i32 {
        // make obsidian bot
        let mut tmp = (
            state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7, state.8,
        );
        tmp.5 += 1;
        tmp.0 -= blueprint.obsidian.0 as i32;
        tmp.2 -= blueprint.obsidian.1 as i32;
        add(&mut tmp, state);
        res.push(tmp);
    }

    if state.0 >= blueprint.clay as i32 {
        // make clay bot
        let mut tmp = (
            state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7, state.8,
        );
        tmp.3 += 1;
        tmp.0 -= blueprint.clay as i32;
        add(&mut tmp, state);
        res.push(tmp);
    }

    if state.0 >= blueprint.ore as i32 {
        // make ore bot
        let mut tmp = (
            state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7, state.8,
        );
        tmp.1 += 1;
        tmp.0 -= blueprint.ore as i32;
        add(&mut tmp, state);
        res.push(tmp);
    }

    let mut tmp = (
        state.0, state.1, state.2, state.3, state.4, state.5, state.6, state.7, state.8,
    );
    add(&mut tmp, state);
    res.push(tmp);
    res
}

#[derive(Debug)]
struct Blueprint {
    ore: u8,
    clay: u8,
    obsidian: (u8, u8),
    geode: (u8, u8),
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    ore: i32,
    ore_bot: i32,
    clay: i32,
    clay_bot: i32,
    obsidian: i32,
    obsidian_bot: i32,
    geode: i32,
    geode_bot: i32,
    time: i32,
}

#[allow(dead_code)]
impl State {
    fn new(time: i32) -> Self {
        Self {
            ore: 0,
            ore_bot: 1,
            clay: 0,
            clay_bot: 0,
            obsidian: 0,
            obsidian_bot: 0,
            geode: 0,
            geode_bot: 0,
            time,
        }
    }

    // Provide something to prune branches quick
    fn potential(&self) -> (i32, i32) {
        // Max potential is if we produce 1 geode bot per turn from now on
        // Min potential is if we do not produce anymore geode bot.
        let min = self.geode + self.geode_bot * (self.time - 1);
        let max =
            self.geode + (self.geode_bot..self.geode_bot + self.time).fold(0, |acc, x| acc + x);
        (min, max)
    }

    fn add(&mut self, other: &State) -> () {
        self.ore += other.ore_bot;
        self.clay += other.clay_bot;
        self.obsidian += other.obsidian_bot;
        self.geode += other.geode_bot;
    }
}

#[allow(dead_code)]
fn possible_next_states(state: &State, blueprint: &Blueprint) -> Vec<State> {
    let mut res = vec![];
    let (curr_min, _) = state.potential();

    if state.ore >= blueprint.geode.0 as i32 && state.obsidian >= blueprint.geode.1 as i32 {
        // make geode bot
        let mut tmp = state.clone();
        tmp.geode_bot += 1;
        tmp.ore -= blueprint.geode.0 as i32;
        tmp.obsidian -= blueprint.geode.1 as i32;
        tmp.time -= 1;
        tmp.add(state);
        res.push(tmp);
    }

    if state.ore >= blueprint.obsidian.0 as i32 && state.clay >= blueprint.obsidian.1 as i32 {
        // make obsidian bot
        let mut tmp = state.clone();
        tmp.obsidian_bot += 1;
        tmp.ore -= blueprint.obsidian.0 as i32;
        tmp.clay -= blueprint.obsidian.1 as i32;
        tmp.time -= 1;
        tmp.add(state);
        if tmp.potential().1 >= curr_min {
            res.push(tmp);
        }
    }

    if state.ore >= blueprint.ore as i32 {
        // make ore bot
        let mut tmp = state.clone();
        tmp.ore_bot += 1;
        tmp.ore -= blueprint.ore as i32;
        tmp.time -= 1;
        tmp.add(state);
        if tmp.potential().1 >= curr_min {
            res.push(tmp);
        }
    }
    if state.ore >= blueprint.clay as i32 {
        // make clay bot
        let mut tmp = state.clone();
        tmp.clay_bot += 1;
        tmp.ore -= blueprint.clay as i32;
        tmp.time -= 1;
        tmp.add(state);
        if tmp.potential().1 >= curr_min {
            res.push(tmp);
        }
    }

    let mut tmp = state.clone();
    tmp.time -= 1;
    tmp.add(state);
    if tmp.potential().1 >= curr_min {
        res.push(tmp);
    }
    res
}
