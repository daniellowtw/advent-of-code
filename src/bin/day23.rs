use std::fs;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let s = fs::read_to_string("./src/input23.txt").unwrap();
    // Reduce input to 2 rows of letters,
    let mut ss: Vec<String> = s
        .split("\n")
        .filter(|x| !x.trim().is_empty())
        .dropping(2).map(|x| x.trim().replace("#", ""))
        .filter(|x| !x.trim().is_empty())
        .collect_vec();

    let start = Instant::now();
    // Part 1;
    // solve(ss);
    // Part 2;
    ss.insert(1, String::from("DCBA"));
    ss.insert(2, String::from("DCBA"));
    solve(ss);
    dbg!(start.elapsed());
}

// #############
// #...........#
// ###D#A#D#C###
//   #C#A#B#B#
//   #########

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct State {
    // Uses the following frame of reference.
    // #############
    // #0123456789a#
    // ###A#B#C#D###
    //   #A#B#C#D#
    //   #########

    // Note that the mapping here is that <char> - 'a' * 2 + 2
    // e.g. to get from 0 to d, we need to make sure path from 1 to 8 are free.
    // 8 = ('d' - 'a') * 2 + 2
    rooms: Vec<VecDeque<char>>,
    // Actually an array of 4
    hallway: [char; 11],
    // Represents the number 0 -> 'a'
    num_rows: usize,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut transposed = [['.'; 4]; 2];
        for i in 0..2 {
            for (j, x) in self.rooms.iter().enumerate() {
                transposed[i][j] = *x.get(i).unwrap_or(&'X')
            }
        };
        write!(f, "\n#############\n#{}#\n###{}###\n  #{}#\n  #########\n", self.hallway.iter().collect::<String>(),
               transposed[0].iter().map(|x| x.to_string()).join("#"),
               transposed[1].iter().map(|x| x.to_string()).join("#"))
    }
}

impl State {
    // Takes input as the natural format. i.e. First row, and then second row.
    fn new(input: Vec<String>) -> State {
        // either 2 or 4 rows
        let num_rows = input.len();
        let mut rooms: Vec<VecDeque<char>> = Vec::new();
        for _ in 0..4 {
            rooms.push(VecDeque::new());
        }

        for j in 0..num_rows {
            for i in 0..4 {
                rooms[i as usize].push_back(input[j].chars().nth(i as usize).unwrap());
            }
        }

        State {
            rooms,
            hallway: ['.'; 11],
            num_rows,
        }
    }
    // Return possible next states and associated costs
    fn next_states(&self) -> Vec<(State, i32)> {
        let mut res: Vec<(State, i32)> = Vec::new();
        self.hallway_to_room_next_states(&mut res);
        self.room_to_hallway_next_states(&mut res);
        res
    }

    // exclusive!!
    fn can_go_in_hallway(&self, room: usize, hall: usize) -> bool {
        for i in room.min(hall)..=room.max(hall) {
            if i == hall || i == room {
                continue;
            }
            if self.hallway[i] != '.' {
                return false;
            }
        }
        return true;
    }

    fn room_to_hallway_next_states(&self, acc: &mut Vec<(State, i32)>) {
        // We can move out
        for (room_idx, room) in self.rooms.iter().enumerate() {
            // Room is already good. Do not move.
            if room.iter().all(|x| *x as u8 == room_idx as u8 + 'A' as u8) {
                continue;
            }
            let &c = room.front().unwrap();
            let starting_hall_idx = room_to_hall_idx(room_idx);
            for candidate_hall_idx in 0..11 {
                if candidate_hall_idx == starting_hall_idx || (candidate_hall_idx == 2 || candidate_hall_idx == 4 || candidate_hall_idx == 6 || candidate_hall_idx == 8) {
                    continue;
                }
                if self.can_go_in_hallway(candidate_hall_idx, starting_hall_idx) {
                    let mut steps = 0;
                    let mut new_state = self.clone();
                    new_state.rooms[room_idx].pop_front();
                    steps += self.num_rows - new_state.rooms[room_idx].len(); // Cost to leave room
                    steps += abs_diff(candidate_hall_idx, starting_hall_idx); // Cost to move to hallway.
                    new_state.hallway[candidate_hall_idx] = c;
                    let cost = steps as i32 * 10_i32.pow((c as u8 - 'A' as u8) as u32);
                    // dbg!(&self, &new_state, &steps, &cost); // Use this to manually verify
                    acc.push((new_state, cost));
                }
            }
        }
    }

    fn hallway_to_room_next_states(&self, acc: &mut Vec<(State, i32)>) {
        for (hall_idx, &c) in self.hallway.iter().enumerate() {
            if c == '.' {
                continue;
            }
            // The only place that this can go to is the designated cell.
            let room_idx = (c as u8 - 'A' as u8) as usize;
            let destination_idx = room_to_hall_idx(room_idx);

            // Check room is eligible:
            let is_eligible = self.rooms[room_idx].iter().all(|&x| x == c);

            if is_eligible && self.can_go_in_hallway(destination_idx, hall_idx) {
                let mut steps = 0; // TODO
                let mut new_state = self.clone();
                new_state.hallway[hall_idx] = '.';
                steps += abs_diff(hall_idx, destination_idx);
                steps += self.num_rows - new_state.rooms[room_idx].len();
                new_state.rooms[room_idx].push_front(c);

                let cost = steps as i32 * 10_i32.pow((c as u8 - 'A' as u8) as u32);
                // dbg!(&self, &new_state, &steps, &cost); // Use this to manually verify
                acc.push((new_state, cost));
            }
        }
    }
}

fn solve(ss: Vec<String>) {
    // Idea is to use dijkstra. Main diff from this and earlier problem
    // is that cost map is now dynamically populated.
    // This time, I'm also storing the path to get to the solution.
    println!("Solving for {}", &ss.join("\n"));

    let expected = State::new(vec![String::from("ABCD"); (&ss).len()]);
    let input = State::new(ss);

    // Store a map of state, and the cheapest way to get there.
    // We represent infinity by the absence of the state.
    let mut cost_map: HashMap<State, (i32, Vec<Box<State>>)> = HashMap::new();

    // This is a max heap by default, we store the cost as negative to get a min heap.
    let mut queue: BinaryHeap<(i32, Box<State>)> = std::collections::BinaryHeap::new();
    let boxed_start = Box::new(input.clone());
    cost_map.insert(input, (0, vec![boxed_start.clone()]));
    queue.push((0, boxed_start));

    while let Some((_, grid)) = queue.pop() {
        // We must have inserted this into the map before.
        let (cost, path) = cost_map.get(&grid).unwrap();
        if *grid == expected {
            dbg!(path, cost);
            // We're processing the destination node and it's the least cost. Print info
            break;
        }
        let cost = *cost;
        let path = path.clone();

        for (state, transition_cost) in grid.next_states() {
            let mut new_path = path.clone();
            new_path.push(Box::from(state.clone()));
            let total_cost = transition_cost + cost;

            if let Some((prev_cost, _)) = cost_map.get(&state) {
                // We've seen this before
                if *prev_cost <= total_cost {
                    // Whatever we found before was cheaper. So don't get to this state via this path.
                    continue;
                }
            }
            // upsert
            cost_map.insert(state.clone(), (total_cost, new_path));
            // Remember! Negative cost because it's a min heap.
            queue.push((-total_cost, Box::from(state)));
        }
    }
}

fn room_to_hall_idx(room_idx: usize) -> usize {
    room_idx * 2 + 2
}

fn abs_diff(a: usize, b: usize) -> usize {
    a.max(b) - a.min(b)
}