use std::fs;

fn main() {
    // Today's input can be hand written.
    // x=57..116, y=-198..-148

    // Part one is just maths. Maximizing height means to maximize y velocity.
    // Since we will always cross the y axis (a parabolic trajectory) again, we can choose a
    // value such that the next step it still reaches within the target box. This means we choose
    // |y_min| -1.

    let (x_min, x_max, y_min, y_max) = (57, 116, -198, -148);
    // Part 2: Input is very small, so just brute force.
    let ans = brute_force(x_min, x_max, y_min, y_max);
    dbg!(ans);
}

fn brute_force(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> i32 {
    let mut count = 0;
    for candidate_vy in y_min..y_min.abs() { // y velocity cannot be negative
        for candidate_vx in 0..=x_max {
            // Search for a t that satisfy req.
            let mut x = 0;
            let mut y = 0;
            let mut vx = candidate_vx;
            let mut vy = candidate_vy;
            loop {
                x = x + vx;
                y = y + vy;
                vx -= 1;
                if vx < 0 {
                    vx = 0;
                }
                vy -= 1;

                // Enumerate stopping conditions.
                if x_min <= x && x <= x_max && y_min <= y && y <= y_max {
                    count += 1;
                    break;
                }

                if x > x_max {
                    break;
                }
                if vx == 0 && x < x_min {
                    break;
                }
                if y <= y_min {
                    break;
                }
            }
        }
    }
    count
}
