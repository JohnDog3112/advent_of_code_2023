use std::collections::HashSet;

use crate::Tile;


pub fn solution(inp: &Vec<Vec<Tile>>) -> usize {

    search(inp, HashSet::new(), 0, (1, 0), (0,-1))
}


fn search(grid: &Vec<Vec<Tile>>, mut corners: HashSet<(usize, usize)>, tiles_traversed: usize, pos: (usize, usize), direction: (i32, i32)) -> usize {
    //println!("-------------------");
    let mut cur_pos = pos;
    let mut traversed = tiles_traversed;
    let mut last_dir = direction;
    loop {
        let mut found_tiles = vec![];
        let mut dir_count = 0;


        let around = [(1,0), (-1,0), (0,1), (0,-1)];
        for dir in around {
            if dir == last_dir {
                continue;
            }
            let x = cur_pos.0 as i32 + dir.0;
            let y = cur_pos.1 as i32 + dir.1;

            if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                continue;
            }

            if corners.contains(&(x as usize, y as usize)) {
                continue;
            }
            //println!("{x}, {y}, {:?}", grid[y as usize][x as usize]);

            let required_dir = match grid[y as usize][x as usize] {
                Tile::Ground => {
                    dir_count += 1;
                    found_tiles.push((x as usize, y as usize));
                    continue;
                },
                Tile::Forest => continue,
                Tile::DownSlope => (0, 1),
                Tile::UpSlope => (0, -1),
                Tile::RightSlop => (1, 0),
                Tile::LeftSlope => (-1, 0),
            };

            dir_count += 1;
            if dir != required_dir {
                continue;
            }

            found_tiles.push((x as usize, y as usize));

        }

        //println!("{:?}", found_tiles);
        if dir_count > 1 {
            corners.insert(cur_pos);
        }
        if found_tiles.is_empty() {
            if cur_pos == (grid[0].len()-2, grid.len()-1) {
                return traversed;
            } else {
                return 0;
            }
        }
        traversed += 1;
        if found_tiles.len() > 1 {
            let mut max = 0;
            for tile in found_tiles {
                let tmp = search(
                    grid, 
                    corners.clone(), 
                    traversed, 
                    tile, 
                    (cur_pos.0 as i32 - tile.0 as i32, cur_pos.1 as i32 - tile.1 as i32)
                );
                if tmp > max {
                    max = tmp;
                }
            }
            return max;
        }

        last_dir = (
            cur_pos.0 as i32 - found_tiles[0].0 as i32,
            cur_pos.1 as i32 - found_tiles[0].1 as i32,
        );

        cur_pos = found_tiles[0];

        
    }
}