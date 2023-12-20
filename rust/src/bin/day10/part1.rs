use std::collections::VecDeque;

use crate::{Tile, try_move, Cord, Dir};

pub fn solution(inp: &Vec<Vec<Tile>>) -> i32 {
    let weights = generate_weights(inp);

    weights.iter().map(|line| *line.iter().max().unwrap()).max().unwrap() - 1

}

pub fn generate_weights(inp: &Vec<Vec<Tile>>) -> Vec<Vec<i32>> {
    let mut weights = vec![vec![0; inp[0].len()]; inp.len()];

    let mut start = Cord { x: -1, y: -1};
    'outer: for (y, row) in inp.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if let Tile::Start = val {
                start = Cord {x: x as i128, y: y as i128};
                break 'outer;
            }
        }
    }
    assert!(start.x != -1, "Invalid input!");

    //set initial weight to 1
    weights[start.y as usize][start.x as usize] = 1;

    let starting_dirs = [Dir::North, Dir::South, Dir::East, Dir::West];

    let mut to_process = VecDeque::new();

    for dir in starting_dirs {
        let cord = start + dir.to_cord();
        if cord.y < 0 || cord.x < 0 || cord.y >= weights.len() as i128 || cord.x >= weights[0].len() as i128 {
            continue;
        }
        let tile = &inp[cord.y as usize][cord.x as usize];
        if let Some(next_dir) = try_move(tile, dir) {
            to_process.push_back((cord, next_dir, 2));
            weights[cord.y as usize][cord.x as usize] = 2;
        }
    }
    while let Some((cord, dir, weight)) = to_process.pop_front() {
        let next_cord = cord + dir.to_cord();
        let tile = &inp[next_cord.y as usize][next_cord.x as usize];

        let cur_weight = weights[next_cord.y as usize][next_cord.x as usize];
        if cur_weight != 0 && cur_weight < weight+1 {
            continue;
        }

        if let Some(new_dir) = try_move(tile, dir) {
            to_process.push_back((next_cord, new_dir, weight+1));
            weights[next_cord.y as usize][next_cord.x as usize] = weight+1;
        }
    }

    weights
}