use std::fs;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    // I actually solved this problem by hand (with great difficulty I must say)
    // I wrote code after the fact to practice a more general version of dijkstra with
    // memory of the path.

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
    ss.insert(2, String::from("DBAC"));
    // dbg!(&ss);
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
    // Use Vec instead of VecDeque because it's simpler and speeds up performance.
    rooms: [Vec<char>; 4],
    // Actually an array of 4
    hallway: [char; 11],
    // Represents the number 0 -> 'a'
    num_rows: usize,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.num_rows == 2 {
            return self.fmt2(f);
        }
        let mut transposed = [['.'; 4]; 4];
        for i in 0..4 {
            for (j, x) in self.rooms.iter().enumerate() {
                transposed[i][j] = *x.get(i).unwrap_or(&'X')
            }
        };
        write!(f, "\n#############\n#{}#\n###{}###\n  #{}#\n  #{}#\n  #{}#\n  #########\n", self.hallway.iter().collect::<String>(),
               transposed[3].iter().map(|x| x.to_string()).join("#"),
               transposed[2].iter().map(|x| x.to_string()).join("#"),
               transposed[1].iter().map(|x| x.to_string()).join("#"),
               transposed[0].iter().map(|x| x.to_string()).join("#"))
    }
}

impl State {
    // Takes input as the natural format. i.e. First row, and then second row.
    fn new(input: Vec<String>) -> State {
        // either 2 or 4 rows
        let num_rows = input.len();
        let mut rooms: [Vec<char>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for j in (0..num_rows).rev() {
            for i in 0..4 {
                rooms[i as usize].push(input[j].chars().nth(i as usize).unwrap());
            }
        }

        State {
            rooms,
            hallway: ['.'; 11],
            num_rows,
        }
    }
    // Equivalent of hash idea
    // Writing this signature function makes the program runs almost twice as fast.
    fn sig(&self) -> String {
        let rooms_sig = self.rooms.iter().map(|room| {
            // &str is immutable!
            let mut ss = vec!('.','.','.','.');
            for (pos, c) in room.iter().enumerate() {
                let i = 3 - pos;
                *ss.get_mut(i).unwrap() = *c;
                // *ss.get_mut(i).unwrap() = *c;
            }
            ss.iter().collect::<String>()
        }).join(",");
        format!("{}-{}", self.hallway.iter().collect::<String>(), rooms_sig)
    }

    // Easier to just hardcode 2 than generalize print function
    fn fmt2(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut transposed = [['.'; 4]; 2];
        for i in 0..2 {
            for (j, x) in self.rooms.iter().enumerate() {
                transposed[i][j] = *x.get(i).unwrap_or(&'X')
            }
        };
        write!(f, "\n#############\n#{}#\n###{}###\n  #{}#\n  #########\n", self.hallway.iter().collect::<String>(),
               transposed[1].iter().map(|x| x.to_string()).join("#"),
               transposed[0].iter().map(|x| x.to_string()).join("#"))
    }

    // Return possible next states and associated costs
    fn next_states(&self) -> Vec<(State, i32)> {
        let mut res: Vec<(State, i32)> = Vec::new();
        self.hallway_to_room_next_states(&mut res);
        self.room_to_hallway_next_states(&mut res);
        res
    }

    // I realized this is a huge branching factor of 7, so we should prune this.
    // This cut the running time by at least 5x.
    fn eligible_hallways(&self, hall_idx: usize) -> Vec<usize> {
        let mut res = Vec::new();
        // left side
        for i in (0..hall_idx).rev() {
            if i == 2 || i == 4 || i == 6 || i == 8 {
                continue;
            }
            if self.hallway[i..hall_idx].iter().all(|&x| x == '.') {
                res.push(i)
            } else {
                break;
            }
        }
        // right side
        for i in (hall_idx + 1)..11 {
            if i == 2 || i == 4 || i == 6 || i == 8 {
                continue;
            }
            if self.hallway[(hall_idx + 1)..=i].iter().all(|&x| x == '.') {
                res.push(i)
            } else {
                break;
            }
        }
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
            if room.iter().all(|&c| c as u8 == (room_idx as u8 + 'A' as u8)) {
                continue;
            }
            let &c = room.last().unwrap();
            let starting_hall_idx = room_to_hall_idx(room_idx);
            for candidate_hall_idx in self.eligible_hallways(starting_hall_idx) {
                let mut steps = 0;
                let mut new_state = self.clone();
                new_state.rooms[room_idx].pop();
                steps += self.num_rows - new_state.rooms[room_idx].len(); // Cost to leave room
                steps += abs_diff(candidate_hall_idx, starting_hall_idx); // Cost to move to hallway.
                new_state.hallway[candidate_hall_idx] = c;
                let cost = steps as i32 * 10_i32.pow((c as u8 - 'A' as u8) as u32);
                // dbg!(&self, &new_state, &steps, &cost); // Use this to manually verify
                acc.push((new_state, cost));
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
                new_state.rooms[room_idx].push(c);

                let cost = steps as i32 * 10_i32.pow((c as u8 - 'A' as u8) as u32);
                // dbg!(&self, &new_state, &steps, &cost); // Use this to manually verify
                acc.push((new_state, cost));
            }
        }
    }
}

struct Node {
    path: Vec<Rc<State>>,
    // Share to save on memory
    state: Rc<State>,
    cost: i32,
    // done: bool,
}

fn solve(ss: Vec<String>) {
    // Idea is to use dijkstra. Main diff from this and earlier problem
    // is that cost map is now dynamically populated.
    // This time, I'm also storing the path to get to the solution.

    let expected = State::new(vec![String::from("ABCD"); (&ss).len()]);
    let input = State::new(ss);
    println!("Solving for {:?} {:?}", &input, &expected);

    // Store a map of state, and the cheapest way to get there.
    // We represent infinity by the absence of the state.
    let mut cost_map: HashMap<String, Node> = HashMap::new();
    // This is a max heap by default, we store the cost as negative to get a min heap.
    let mut queue: BinaryHeap<(i32, String)> = std::collections::BinaryHeap::new();

    let sig = (&input).sig();
    let boxed_start = Rc::new(input);
    cost_map.insert(sig.clone(), Node {
        path: vec![boxed_start.clone()],
        state: boxed_start,
        // done: false,
        cost: 0,
    });
    queue.push((0, sig));

    while let Some((_, sig)) = queue.pop() {
        // We must have inserted this into the map before.
        let Node { cost, state, path } = cost_map.get(&sig).unwrap();
        if (**state) == expected {
            dbg!(path, cost);
            // We're processing the destination node and it's the least cost. Print info
            break;
        }

        // Make copy so that we can use it for next states below.
        let cost = *cost;
        let new_path = path.clone();

        for (next_state, transition_cost) in state.next_states() {
            let sig = (&next_state).sig();
            let current_cost = transition_cost + cost;
            if let Some(Node { cost, .. }) = cost_map.get(&sig) {
                // We've seen this before
                if *cost <= current_cost {
                    // Whatever we found before was cheaper. So don't get to this state via this path.
                    continue;
                }
            }

            let boxed_next_state = Rc::from(next_state);
            let mut new_path = new_path.clone();
            new_path.push(boxed_next_state.clone());

            // upsert
            cost_map.insert(sig.clone(), Node {
                // state: boxed_next_state.clone(),
                path: new_path,
                state: boxed_next_state,
                // done: false,
                cost: current_cost,
            });
            // Remember! Negative cost because it's a min heap.
            queue.push((-current_cost, sig));
        }
        // let n = cost_map.get_mut(&sig).unwrap();
        // (*n).done= true;
    }
}

fn room_to_hall_idx(room_idx: usize) -> usize {
    room_idx * 2 + 2
}

fn abs_diff(a: usize, b: usize) -> usize {
    a.max(b) - a.min(b)
}