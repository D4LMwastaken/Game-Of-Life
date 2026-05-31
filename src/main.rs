// Imports
use std::thread::sleep;
use std::time::Duration;

// Defined how big it is
const ROWS: usize = 67;
const COLS: usize = 67;

fn main() {
    let mut grid = vec![vec![false;COLS];ROWS];
    /* 3 dots on a row
    grid[10][9] = true;
    grid[10][10] = true;
    grid[10][11] = true;
    */
    /* Blinking eye
    grid[10][10] = true; grid[10][11] = true; grid[10][12] = true;
    grid[11][9] = true; grid[11][10] = true; grid[11][11] = true;
    */
    grid[10][11] = true;
    grid[11][12] = true;
    grid[12][10] = true; grid [12][11] = true; grid[12][12] = true;

    print_grid(&grid);
    loop {
        print!("{}[2J{}[1;1H", 27 as char, 27 as char); // Cool print function that clears the terminal
        print_grid(&grid);
        grid = update_grid(&grid);
        sleep(Duration::from_millis(200));
    }
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for &cell in row {
            if cell == true {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn count_neighbors(grid: &Vec<Vec<bool>>, r: usize, c:usize) -> i32 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1{
            if i == 0 && j == 0 {
                continue;
            }

            let neighbor_row = (r as isize) + i;
            let neighbor_col = (c as isize) + j;

            if neighbor_row >= 0 && neighbor_row < (ROWS as isize) && neighbor_col >= 0 && neighbor_col < (COLS as isize) {
                if grid[neighbor_row as usize][neighbor_col as usize] == true {
                    count += 1;
                }
            }
        }
    }

    count
}

fn update_grid(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut next_grid = vec![vec![false; COLS]; ROWS];

    for r in 0..ROWS {
        for c in 0..COLS {
            let is_alive = grid[r][c];
            let neighbors = count_neighbors(grid, r, c);
            if is_alive {
                if neighbors == 2 || neighbors == 3 {
                    next_grid[r][c] = true;
                } else {
                    next_grid[r][c] = false;
                }
            } else { // Not alive
                if neighbors == 3 { // Revive by the power of 3
                    next_grid[r][c] = true;
                } else {
                    next_grid[r][c] = false;
                }
            }
        }
    }
    next_grid
}