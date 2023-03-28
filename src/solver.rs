use crate::square::Square;
use std::cmp::Ordering;

pub fn verify_grid(grid: &[Vec<Square>]) -> bool {
    let mut row: u128 = 0;
    let mut col: u128 = 0;
    let mut boxes: u128 = 0;

    let mut cells = 0;
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j].value.is_empty() {
                continue;
            }

            cells += 1;

            let key = grid[i][j].value.chars().next().unwrap() as usize - '1' as usize;

            let key_row = 1 << (i * 9 + key);
            let key_col = 1 << (j * 9 + key);

            // i / 3 is integer division
            // We get the starting row index of the box (i / 3 * 3)
            // Then we get the starting column (j / 3)
            let key_boxes = 1 << ((i / 3 * 3 + j / 3) * 9 + key);

            // Check if corresponding bits are already set
            if row & key_row | col & key_col | boxes & key_boxes != 0 {
                return false;
            }

            // Set the corresponding bits
            row |= key_row;
            col |= key_col;
            boxes |= key_boxes;
        }
    }

    // The smallest amount of Sudoku "hints" is 17
    //https://phys.org/news/2012-01-mathematicians-minimum-sudoku-solution-problem.html
    cells >= 17
}

pub enum SolveResult {
    Unique,
    NotUnique,
    Invalid,
}

pub fn solve_grid(grid: &mut Vec<Vec<Square>>) -> SolveResult {
    // These are the bit fields that keep track of the numbers placed in each row, column, and box of the grid
    let mut row: u128 = 0;
    let mut col: u128 = 0;
    let mut boxes: u128 = 0;

    for r in 0..9 {
        for c in 0..9 {
            if !grid[r][c].value.is_empty() {
                // Calculated by left-shifting 1 by a value between 0 and 8, depending on the digit in the cell
                let key = 1 << (grid[r][c].value.chars().next().unwrap() as usize - '1' as usize);

                // The key value is then used to update the corresponding bit in the bit fields
                row |= key << (r * 9);
                col |= key << (c * 9);
                boxes |= key << ((r / 3 * 3 + c / 3) * 9);
            }
        }
    }

    
    let mut count = 0;
    let old_grid = grid.clone();

    // We keep a solved_grid because we make sure that there is not a 2nd solution to the puzzle
    // If another solution doesn't exits then we set the grid equal to the solved_grid
    let mut solved_grid: Vec<Vec<Square>> = Vec::new();

    // Call the solving method recursively
    solve(&mut solved_grid, grid, &mut row, &mut col, &mut boxes, 0, &mut count);

    match count.cmp(&1) {
        Ordering::Equal => {
            *grid = solved_grid;
            SolveResult::Unique
        },
        Ordering::Greater => {
            *grid = old_grid;
            SolveResult::NotUnique
        }
        Ordering::Less => {
            *grid = old_grid;
            SolveResult::Invalid
        }
    }
}

fn solve(
    solved_grid: &mut Vec<Vec<Square>>, 
    grid: &mut Vec<Vec<Square>>,
    row: &mut u128,
    col: &mut u128,
    boxes: &mut u128,
    i: usize,
    count: &mut i32,
) -> bool {
    // If there is multiple solutions then automatically return true
    if *count > 1 {
        return true;
    }

    // If reached the end
    if i == 81 {
        // We need to save the grid in the case that we do not find another solution to the puzzle
        if *count == 0 {
            *solved_grid = grid.clone();
        }

        *count += 1;
        return false;
    }

    
    // Get the index of the row and column
    let (r, c) = (i / 9, i % 9);

    // If the cell is not empty then move to the next cell
    if !grid[r][c].value.is_empty() {
        return solve(solved_grid, grid, row, col, boxes, i + 1, count);
    }

    // Box index
    let b = (r / 3) * 3 + (c / 3);

    // This is a bit mask that represents the numbers that are already present
    // We shift to the right to align each bits with the corresponding row, column, and box
    let mask = (*row >> (r * 9)) | (*col >> (c * 9)) | (*boxes >> (b * 9));

    for x in 0..9 {
        // Move the bit that 1 has to the xth bit and then check it
        // to make sure that the bit has not already been set 
        let xmask = 1 << x;
        if mask & xmask != 0 {
            continue;
        }

        // We update the bit at the current x value using xmask
        *row |= xmask << (r * 9);
        *col |= xmask << (c * 9);
        *boxes |= xmask << (b * 9);

        // Since its 0-8 then we do x+1
        grid[r][c].value = std::char::from_digit(x + 1, 10).unwrap().to_string();
        grid[r][c].solved_cell = true;
        // Recursively call itself with the next cell to check if the value works
        if solve(solved_grid, grid, row, col, boxes, i + 1, count) {
            return true;
        }

        // If it didnt work then we reset the changes we did to the bit fields
        *row ^= xmask << (r * 9);
        *col ^= xmask << (c * 9);
        *boxes ^= xmask << (b * 9);

        grid[r][c].value = String::new();
        grid[r][c].solved_cell = false;
    }
    
    false
}
