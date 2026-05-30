// Defined how big it is
const ROWS: usize = 67;
const COLS: usize = 67;

fn main() {
    let mut grid = vec![vec![false;COLS];ROWS];
    grid[10][9] = true;
    grid[10][10] = true;
    grid[10][11] = true;
    print_grid(&grid);
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for &cell in row {
            if cell == true {
                print!("#");
            } else {
                print!(" ");
            }
            println!();
        }
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