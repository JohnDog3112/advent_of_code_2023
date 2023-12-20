use std::collections::HashSet;

use crate::{Tile, BeamDir};

pub fn solution(inp: &[Vec<Tile>]) -> usize {

    let mut max = 0;
    for i in 0..inp.len() {
        let left = calculate_tiles(inp, 0, i as i32, BeamDir::Right);
        let right = calculate_tiles(inp, inp[0].len() as i32 - 1, i as i32, BeamDir::Left);

        max = max.max(left.max(right));
    }

    for i in 0..inp[0].len() {
        let top = calculate_tiles(inp, i as i32, 0, BeamDir::Down);
        let bottom = calculate_tiles(inp, i as i32, inp.len() as i32 - 1, BeamDir::Up);

        max = max.max(top.max(bottom));
    }

    max
}

fn calculate_tiles(inp: &[Vec<Tile>], x: i32, y: i32, dir: BeamDir) -> usize {
    let mut visited_tiles = HashSet::new();
    let mut tile_dirs = HashSet::new();

    visited_tiles.insert((x, y));
    tile_dirs.insert((x, y, dir));

    let mut beams = vec![(x, y, dir)];

    while !beams.is_empty() {
        let mut new_beams = vec![];
        for (x, y, beam_dir) in beams {
            let next_dir = beam_dir.get_next(&inp[y as usize][x as usize]);
            
            for dir in next_dir {
                let offset = dir.get_offset();

                let next_x = x + offset.0;
                let next_y = y + offset.1;

                if next_x < 0 || next_y < 0 || next_x > inp[0].len() as i32 -1 || next_y > inp.len() as i32-1 {
                    continue;
                }
                visited_tiles.insert((next_x, next_y));

                let next_beam = (next_x, next_y, dir);
                if tile_dirs.contains(&next_beam) {
                    continue;
                }
                tile_dirs.insert(next_beam);
                new_beams.push(next_beam);
            }
        }
        beams = new_beams;
    }
    visited_tiles.len()
}