use std::cmp::min;
use std::collections::{BinaryHeap};
use std::fs;
use itertools::Itertools;

fn main() {
    // Strategy: dikjstra.
    let s: String = fs::read_to_string("./src/input15.txt").unwrap();

    #[allow(unused_mut)] // So that we can reuse the same code for part 2
    let mut grid: Vec<Vec<i32>> = s.split("\n")
        .filter(|x| { !x.is_empty() })
        .map(|x| x.split("")
            .filter(|x| { !x.is_empty() })
            .map(|y| y.trim().parse::<i32>().unwrap()).collect_vec()
        )
        .collect_vec();
    let num_rows = grid.len();
    let num_cols = grid.first().unwrap().len();
    dbg!(num_rows, num_cols);


    // Part 2: We need to build a new grid.
    // For part one, just comment this block out.
    // First we repeat then we apply offset.
    // Repeat cols.
    let mut grid = grid.iter().map(|x2| {x2.repeat(5)}).collect_vec();
    // Repeat rows.
    for _ in 1..5 {
        for i in 0..num_rows {
            grid.push(grid.get(i).unwrap().clone());
        }
    }
    // Apply offset.
    grid.iter_mut()
        .enumerate()
        .for_each(|(row_i, row)| {
            row.iter_mut()
                .enumerate()
                .for_each(|(col_i, col)| {
                    let offset = (row_i / num_rows + col_i / num_cols) % 9;
                    let i1 = *col + offset as i32;
                    *col = if i1 > 9 {
                        i1 - 9 // wrap around
                    } else {
                        i1
                    }
                })
        });
    let num_rows = grid.len();
    let num_cols = grid.first().unwrap().len();
    dbg!(num_rows, num_cols);

    // Create cost matrix from grid.
    let mut cost_grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..num_rows {
        let mut row: Vec<i32> = Vec::new();
        row.resize(num_cols, i32::MAX);
        cost_grid.push(row);
    }

    // Solution is literally on the example page of heap lol.
    // https://doc.rust-lang.org/std/collections/binary_heap/
    // This is by default a max heap.
    // So we have to make sure to negate value when putting in so we get the min out first.
    let mut queue = BinaryHeap::new();

    queue.push((0, (0, 0)));
    // Set top left to be 0 cost.
    *cost_grid.get_mut(0).unwrap().get_mut(0).unwrap() = 0;

    while !queue.is_empty() {
        let (current_cell_cost, (i, j)) = queue.pop().unwrap();
        let current_cell_cost = - current_cell_cost; // Out heap is a max heap.
        let grid_cell = grid.get(i).unwrap().get(j).unwrap();
        if *grid_cell == -1 {
            // Visited
            continue;
        }
        // let current_cell_cost = cost_grid.get(i).unwrap().get(j).unwrap().clone();
        // We can re use from day 9!
        let neighbours = neighbours(num_rows, num_cols, i, j);
        for (nx, ny) in neighbours {
            // let neighbour_cost = (&cost_grid).get(nx).unwrap().get(ny).unwrap().clone();
            let neighbour_cell = grid.get_mut(nx).unwrap().get_mut(ny).unwrap();
            if *neighbour_cell == -1 {
                // visited
                continue;
            }
            let x1 = cost_grid.get_mut(nx).unwrap().get_mut(ny).unwrap();
            *x1 = min(*x1, current_cell_cost + *neighbour_cell);
            queue.push((- *x1, (nx, ny)))
        }
        let grid_cell = grid.get_mut(i).unwrap().get_mut(j).unwrap();
        *grid_cell = -1; // mark as visited
    }
    let x1 = (&cost_grid).last().unwrap().last().unwrap().clone();

    dbg!(x1);
}

fn neighbours(num_rows: usize, num_cols: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let vec1: Vec<i32> = vec![-1, 1];
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for k in vec1.clone() {
        let newi = i as i32 + k;
        if newi >= 0 && newi < num_rows as i32 {
            candidates.push((newi as usize, j))
        }
        let newj = j as i32 + k;
        if newj >= 0 && newj < num_cols as i32 {
            candidates.push((i, newj as usize))
        }
    }
    candidates
}

