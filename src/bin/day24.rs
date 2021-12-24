use std::fs;
use std::time::Instant;
use itertools::Itertools;

#[derive(Debug)]
// This is after interpretation of what the code block is doing and the minimal fields to capture
// what each block differs in. See algorithm description below for more details.
struct Instruction {
    pop_first: bool,

    // Compare previous
    compare: i8,

    // If <digit> matches (prev + compare), then push offset + <digit>
    offset: i8,
}


fn main() {
    // Idea: dfs/bfs. The crux about this input is that it's split into 14 chunks.
    // Each chunk has the same pattern and does the following algo:
    // 1. Read input and look at the state of z, extract remainder of z
    // 2. Maybe divide the state by 26
    // 3. Compare f(remainder) with input
    // 4. If it's the same, then go to step 1
    // 4b. If it is different, multiply z by 26, and then add f(input)
    let s: String = fs::read_to_string("./src/input24.txt").unwrap();
    let ss = s.split("\n")
        .filter(|x| !x.is_empty())
        .collect_vec(); // collect();
    let ss = ss.chunks(18).collect_vec();
    assert_eq!(ss.len(), 14); // Should have 14 instructions;

    let mut instructions = vec![];
    for block in ss {
        let divide_instruction = block[4].split(" ").collect_vec();
        assert_eq!(divide_instruction.first().unwrap(), &"div");
        let pop_first = divide_instruction.last().unwrap() == &"26";

        let compare_instruction = block[5].split(" ").collect_vec();
        assert_eq!(compare_instruction.first().unwrap(), &"add");
        let compare = compare_instruction.last().unwrap().parse::<i8>().unwrap();

        let offset_instruction = block[15].split(" ").collect_vec();
        assert_eq!(offset_instruction.first().unwrap(), &"add");
        let offset = offset_instruction.last().unwrap().parse::<i8>().unwrap();
        instructions.push(Instruction {
            pop_first,
            compare,
            offset,
        });
    }
    // dbg!(&instructions);

    // PS: Technically can solve by hand now because there are exactly 7 "pop" and 7 "push" instructions.
    // .e.g the last pop must be to pop the first "push"

    // Part 1
    let s = Instant::now();
    // We can represent z with a stack, where elements represents the base 26 representation of z
    // This makes it easy to manipulate division and remainder. The key assumption here is that
    // our compare and offset are never big enough to cause the value to exceed 26. Might be safer
    // to just use i32/i64 for general case.

    // Do DFS where the state is the stack and the digits chosen so far.
    let ans = aux_dfs(1, (vec![], vec![]), &instructions, &(1..=9).rev().collect_vec());
    dbg!(ans);
    // Pretty slow now < 3m.
    dbg!(s.elapsed());
    assert_eq!(simulate(vec![5, 3, 9, 9, 9, 9, 9, 5, 8, 2, 9, 3, 9, 9], &instructions), vec![]);

    // Part 2. We just need to reverse the order. Interestingly this is way quicker!
    let s = Instant::now();
    let ans2 = aux_dfs(1, (vec![], vec![]), &instructions, &(1..=9).collect_vec());
    dbg!(s.elapsed());
    dbg!(ans2);
}

// Step starts from 1.
fn aux_dfs(step: i8, state: (Vec<i8>, Vec<i8>), instructions: &Vec<Instruction>, search_order: &Vec<i8>) -> Option<Vec<i8>> {
    if step == 15 {
        if state.0.len() == 0 { Some(state.1) } else { None }
    } else {
        let (current_stack, steps_taken) = state;
        let instruction = instructions.get((step - 1) as usize).unwrap();

        let pop_left = instructions.iter().dropping((step - 1) as usize).filter(|x| x.pop_first).count();
        if current_stack.len() > pop_left {
            // Not possible even if you pop everything.
            return None;
        }

        for &digit in search_order {
            let mut new_stack = current_stack.clone();
            let mut new_step = steps_taken.clone();
            new_step.push(digit);
            let prev_val = if instruction.pop_first {
                new_stack.pop().unwrap()
            } else {
                *new_stack.last().unwrap_or(&0)
            };
            if prev_val + instruction.compare != digit {
                new_stack.push(instruction.offset + digit)
            }
            if let Some(ans) = aux_dfs(step + 1, (new_stack, new_step), instructions, search_order) {
                return Some(ans);
            }
        }
        return None;
    }
}

fn simulate(input: Vec<i8>, instructions: &Vec<Instruction>) -> Vec<i8> {
    let mut stack = vec![];
    for (i, &digit) in input.iter().enumerate() {
        let instruction = instructions.get(i).unwrap();
        let prev_val = if instruction.pop_first {
            stack.pop().unwrap()
        } else {
            *stack.last().unwrap_or(&0)
        };
        if prev_val + instruction.compare != digit {
            stack.push(instruction.offset + digit)
        }
    }
    stack
}