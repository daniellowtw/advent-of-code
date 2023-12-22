#![allow(warnings)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    process::exit,
    vec,
};

#[derive(Debug, Clone)]
struct PuzzleInput {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
    start: (usize, usize),
}

impl PuzzleInput {
    fn directional_neighbour(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < self.height - 1 {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < self.width - 1 {
            neighbours.push((x, y + 1));
        }
        return neighbours;
    }
}

fn parse(s: &str) -> PuzzleInput {
    let grid: Vec<Vec<char>> = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().chars().collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut start = (0, 0);
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'S' {
                start = (i, j);
            }
        }
    }
    return PuzzleInput {
        grid,
        start,
        height,
        width,
    };
}

fn succ_1(pi: &PuzzleInput, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    for pos in pi.directional_neighbour(x, y) {
        let c = pi.grid[pos.0][pos.1];
        if c == '#' {
            continue;
        }
        res.insert(pos);
    }
    return res;
}

fn part2(pi: &PuzzleInput, dist: i64) -> i64 {
    // My first intuition was to do a BFS, but on meta grid, where each coordinate is the given grid.
    // Each visit to a grid will reduce dist by approximately O(size of grid).
    // This is too slow as we still need to basically explore O(10^6 * 10^6) nodes.

    // Then my next intuition was to skip to the boundary, and then try to do a DFS there to determine
    // the meta grid boundary. A boundary is defined to be a partially filled grid where the max distance
    // left when entering the grid is not enough to visit all visitable points.

    // This seems doable so I started coding up a all_paths to all_paths function, which answers the following:
    // When entering the grid from a given position, what is the shortest dist to the adjacent grids,
    // and the position in that new grid.

    // I hadn't looked at the sample input at this point. When I finished coding, I ran it for the input and
    // it was still very slow. I reckon it is still exploring about O(10^7 - 10^8) nodes. When I printed out
    // what the vertical path up looks like between each grid. I noticed it's a straight line. So I looked at the input.
    // I realized that basically the input has a boundary of . and a vertical and horizontal lines of ".".
    // So I realized this can be solved by some maths.

    // I did a few calculations of the given input, and noticed that the number of steps reaches exactly the boundary
    // of a sub grid if we moved in only one direction. Given the symmetry of the input, I only needed to count
    // 12 different partial grid types and combine them in some linear transformation.

    let mid_length = pi.start.0 as i64;
    let block_idx = (dist - mid_length - 1) / pi.height as i64;
    let mut block_rem = (dist - mid_length - 1) % pi.height as i64;
    if block_rem < 0 {
        block_rem += pi.height as i64;
    }
    dbg!(block_idx, block_rem);
    assert!(block_idx * pi.height as i64 + block_rem + mid_length + 1 == dist);
    dbg!(pi.start);

    // Specific to puzzle input. These assertions makes sure the grid types are correct
    assert!(block_rem == pi.height as i64 - 1 as i64);
    assert!(mid_length * 2 == block_rem);
    assert!(block_idx % 2 != 0);

    let mid = pi.start.0;

    // Partial grid types. These are subgrids that are missing two corners.
    let top = part1(pi, (pi.height - 1, mid), block_rem);
    let left = part1(pi, (mid, pi.width - 1), block_rem);
    let right = part1(pi, (mid, 0), block_rem);
    let bottom = part1(pi, (0, mid), block_rem);

    // Partial grid types. These are subgrids that looks like corners.
    let top_left = part1(pi, (pi.height - 1, pi.width - 1), mid_length - 1);
    let top_right = part1(pi, (pi.height - 1, 0), mid_length - 1);
    let bottom_left = part1(pi, (0, pi.width - 1), mid_length - 1);
    let bottom_right = part1(pi, (0, 0), mid_length - 1);

    let mut ans = top + left + right + bottom;
    ans += (block_idx + 1) * (top_left + top_right + bottom_left + bottom_right);

    // Partial grids. These are subgrids that are missing one corners.
    let top_left = part1(pi, (pi.height - 1, pi.width - 1), mid_length + block_rem);
    let top_right = part1(pi, (pi.height - 1, 0), mid_length + block_rem);
    let bottom_left = part1(pi, (0, pi.width - 1), mid_length + block_rem);
    let bottom_right = part1(pi, (0, 0), mid_length + block_rem);
    ans += (block_idx) * (top_left + top_right + bottom_left + bottom_right);

    // At this point we have the boundary partial grids covered. We now need to count for the full grids.

    let (odd, even) = all_paths(pi, pi.start);

    // Specific to puzzle input. The parity of block_idx and dist determines how we should combine the full grids.
    // A subgrid is odd if and only if the meta world the coordinates sum to an odd number.

    // In the starting grid, we start in an even cell (i + j == even), given that we are moving an odd number of steps,
    // we can only reach the odd cells in this grid. It takes an even number of steps, and we start in an odd
    // cell (i + j == odd). So we can only reach the even cells in this adjacent grid.
    let total_odd_subgrids = (block_idx + 1) * (block_idx + 1);
    let total_even_subgrids = block_idx * block_idx;

    ans += total_odd_subgrids * even;
    ans += total_even_subgrids * odd;

    return ans;
}

fn all_paths(pi: &PuzzleInput, start: (usize, usize)) -> (i64, i64) {
    // This returns the possible positions that can be reached in the given grid in an odd and even number of steps.
    let mut hm = HashMap::new();
    let mut curr_pos: VecDeque<((usize, usize), i32)> = VecDeque::new();
    curr_pos.push_back((start, 0));
    // Explore all possible paths with bfs, so we get cheapest cost
    while !curr_pos.is_empty() {
        let (pos, step) = curr_pos.pop_front().unwrap();
        if hm.contains_key(&pos) {
            continue;
        }
        if step != 0 {
            hm.insert(pos, step as i64);
        }
        for i in succ_1(&pi, pos.0, pos.1) {
            curr_pos.push_back((i, step + 1));
        }
    }

    let mut odd = 0;
    let mut even = 0;

    for i in 0..pi.height {
        for j in 0..pi.width {
            if hm.contains_key(&(i, j)) {
                if (i + j) % 2 == 0 {
                    even += 1;
                } else {
                    odd += 1;
                }
            }
        }
    }
    return (odd, even);
}

fn part1(pi: &PuzzleInput, start: (usize, usize), n: i64) -> i64 {
    // Strategy: create next state, and do it for n times, then find the size of the final state.
    // state here tracks what can be reached from the current state.
    let init: HashSet<(usize, usize)> = vec![start].into_iter().collect();
    let res = (0..n).fold(init, |acc, _| {
        return acc
            .iter()
            .flat_map(|pos| succ_1(pi, pos.0, pos.1))
            .collect();
    });
    return res.len() as i64;
}

fn _display_grid(pi: &PuzzleInput, res: &HashSet<(usize, usize)>) {
    for i in 0..pi.height {
        for j in 0..pi.width {
            if res.contains(&(i, j)) {
                print!("O");
            } else if pi.grid[i][j] == '#' {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let s: String = fs::read_to_string("./src/input21.txt").unwrap();
    let pi: PuzzleInput = parse(s.trim());
    // let ans = part1(&pi, 64);
    let ans = part2(&pi, 26501365);
    // 9766433312
    // 12055236918 // Too low
    // 302628077045904 // Too Low
    // 605244149013955 // Too Low
    // 605247143458542 // ??
    // 605247143458542
    // 605247138198755
    println!("{}", ans);
    // let ans = part2(pi);
    // println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn test_part1_sample_input() {
        let pi = parse(SAMPLE_INPUT.trim());
        let ans = part1(&pi, pi.start, 6);
        println!("{}", ans);
        assert_eq!(ans, 16);
    }

    #[test]
    fn test_part2_sample_input() {
        let s: String = fs::read_to_string("./src/input21.txt").unwrap();
        let pi = parse(s.trim());
        // The sample input doesn't satisfy the assumptions I am making for my input.
        // Hence, use a smaller case and use part 1 to check my maths for part 2.

        // Create a 5 x 5 grid based on input
        let meta_world_size = 5;
        let number_of_sub_grids_north_before_hitting_boundary = 2;
        let mut new_map = vec![vec!['.'; meta_world_size * 131]; meta_world_size * 131];
        for i in 0..new_map.len() {
            for j in 0..new_map[0].len() {
                new_map[i][j] = pi.grid[i % pi.height][j % pi.width];
            }
        }

        let pi2 = PuzzleInput {
            grid: new_map,
            start: (
                number_of_sub_grids_north_before_hitting_boundary * 131 + 65,
                number_of_sub_grids_north_before_hitting_boundary * 131 + 65,
            ),
            height: meta_world_size * 131,
            width: meta_world_size * 131,
        };
        let distance = (65 + number_of_sub_grids_north_before_hitting_boundary * 131) as i64;
        // dbg!(part1(&pi2, pi2.start, distance));
        // cached value 92811

        let ans = part2(&pi, distance);
        assert_eq!(ans, 92811);
    }
}
