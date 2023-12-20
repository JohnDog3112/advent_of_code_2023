use std::collections::HashMap;

#[allow(unused_imports)]
use crate::{Tile, print_grid};

pub fn solution(mut inp: Vec<Vec<Tile>>) -> usize {

    let cycles = 1000000000;

    let mut grids = HashMap::new();

    let mut loop_start = 0;
    let mut loop_end = 0;
    for i in 0..cycles {
        move_north(&mut inp);
        move_west(&mut inp);
        move_south(&mut inp);
        move_east(&mut inp);

        if let Some(iter) = grids.get(&inp) {
            //println!("grid repeated! {} - {}", iter, i);
            loop_start = *iter;
            loop_end = i;
            break;
        } else {
            grids.insert(inp.clone(), i);
        }
    }

    let loop_length = loop_end - loop_start;

    //number of cycles left after ignoring all the loops
    let remaining_cycles = (cycles-loop_start-1)%loop_length;

    for _ in 0..remaining_cycles {
        move_north(&mut inp);
        move_west(&mut inp);
        move_south(&mut inp);
        move_east(&mut inp);
    }
    //print_grid(&inp);
    
    inp.iter().enumerate().map(|(i, row)| {
        let move_rocks = row.iter().filter(|tile| **tile == Tile::MoveRock).count();
        move_rocks * (inp.len()-i)
    }).sum()
}

fn move_north(grid: &mut Vec<Vec<Tile>>) {
    for row in 0..grid.len() {
        'a: for col in 0..grid[0].len() {
            if grid[row][col] != Tile::MoveRock {
                continue;
            }
            for up_amt in 0..row {
                if grid[row-up_amt-1][col] != Tile::Empty {
                    grid[row][col] = Tile::Empty;
                    grid[row-up_amt][col] = Tile::MoveRock;
                    continue 'a;
                }
            }
            //didn't hit anything
            grid[row][col] = Tile::Empty;
            grid[0][col] = Tile::MoveRock;
        }
    }
}
fn move_west(grid: &mut Vec<Vec<Tile>>) {
    for col in 0..grid[0].len() {
        #[allow(clippy::needless_range_loop)]
        'a: for row in 0..grid.len() {
            if grid[row][col] != Tile::MoveRock {
                continue;
            }
            for left_amt in 0..col {
                if grid[row][col-left_amt-1] != Tile::Empty {
                    grid[row][col] = Tile::Empty;
                    grid[row][col-left_amt] = Tile::MoveRock;
                    continue 'a;
                }
            }
            //didn't hit anything
            grid[row][col] = Tile::Empty;
            grid[row][0] = Tile::MoveRock;
        }
    }
}
fn move_south(grid: &mut Vec<Vec<Tile>>) {
    for row in (0..grid.len()).rev() {
        'a: for col in 0..grid[0].len(){
            if grid[row][col] != Tile::MoveRock {
                continue;
            }
            for down_amt in 0..(grid.len()-row-1){
                if grid[row+down_amt+1][col] != Tile::Empty {
                    grid[row][col] = Tile::Empty;
                    grid[row+down_amt][col] = Tile::MoveRock;
                    continue 'a;
                }
            }
            let grid_len = grid.len();
            //didn't hit anything
            grid[row][col] = Tile::Empty;
            grid[grid_len-1][col] = Tile::MoveRock;
        }
    }
}

fn move_east(grid: &mut Vec<Vec<Tile>>) {
    for col in (0..grid[0].len()).rev() {
        'a: for row in 0..grid.len() {
            if grid[row][col] != Tile::MoveRock {
                continue;
            }
            for right_amt in 0..(grid[0].len()-col-1) {
                if grid[row][col+right_amt+1] != Tile::Empty {
                    grid[row][col] = Tile::Empty;
                    grid[row][col+right_amt] = Tile::MoveRock;
                    continue 'a;
                }
            }
            //didn't hit anything
            let grid_len = grid[0].len();
            grid[row][col] = Tile::Empty;
            grid[row][grid_len-1] = Tile::MoveRock;
        }
    }
}