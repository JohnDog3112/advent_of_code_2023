use std::collections::HashSet;

use crate::{Tile, BeamDir};

pub fn solution(inp: &[Vec<Tile>]) -> usize {

    let mut visited_tiles = HashSet::new();
    let mut tile_dirs = HashSet::new();

    visited_tiles.insert((0,0));
    tile_dirs.insert((0,0, BeamDir::Right));

    let mut beams = vec![(0, 0, BeamDir::Right)];

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
