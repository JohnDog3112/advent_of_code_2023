use std::collections::HashSet;

use crate::Tile;

pub fn solution(start: (usize, usize), grid: &[Vec<Tile>]) -> usize{

    let mut prev = vec![start];
    for _ in 0..64 {
        let mut new = HashSet::new();
        recurse_tiles(&prev, &mut new, grid);

        prev = new.into_iter().collect();

    }

    prev.len()
}

fn recurse_tiles(starting: &[(usize, usize)], visited: &mut HashSet<(usize, usize)>, grid: &[Vec<Tile>]) {
    for pos in starting {
        let around = [(0,1), (0,-1), (1,0),(-1,0)];
        for dir in around {
            let x = pos.0 as i32 + dir.0;
            let y = pos.1 as i32 + dir.1;
            if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                continue;
            }

            let (x,y) = (x as usize, y as usize);

            if grid[y][x] == Tile::Rock {
                continue;
            }

            visited.insert((x,y));
        }
    }
}