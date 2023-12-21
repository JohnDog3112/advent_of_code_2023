use std::collections::HashSet;

use crate::Tile;

//kinda slow, might optimize later
//https://www.desmos.com/calculator/ougazmlgmz
pub fn solution(start: (usize, usize), grid: &[Vec<Tile>]) -> i128 {



    let mut prev = vec![(start.0 as i64, start.1 as i64)];
    for _i in 0..(131*2 + 65) {
        let mut new = HashSet::new();
        recurse_tiles(&prev, &mut new, grid);

        prev = new.into_iter().collect();

        /*if i % 100 == 0 {
            println!("{i}");
        }*/
    }
    let mut sections = vec![vec![]; 5*5];

    for point in &prev {
        let a = (point.0 + 2*131)/131;
        let b = (point.1 + 2*131)/131;

        let x = point.0.rem_euclid(grid[0].len() as i64);
        let y = point.1.rem_euclid(grid.len() as i64);

        sections[a as usize + (b*5) as usize].push((x,y));
    }
    let steps: i128 = 26501365;
    let tile_length = steps/131;
    let small_sections = tile_length;
    let large_sections = small_sections-1;

    let even_sections = tile_length*tile_length;
    let odd_sections = (tile_length-1)*(tile_length-1);
    
    let end_count = {
        sections[2].len()
        + sections[10].len()
        + sections[14].len()
        + sections[22].len()
    } as i128;

    let small_count = {
        sections[1].len()
        + sections[3].len()
        + sections[15].len()
        + sections[19].len()
    } as i128;

    let large_count = {
        sections[6].len()
        + sections[8].len()
        + sections[16].len()
        + sections[18].len()
    } as i128;

    let even_tile_count = {
        sections[7].len()
    } as i128;

    let odd_tile_count = {
        sections[12].len()
    } as i128;


    end_count 
    + small_count*small_sections 
    + large_count*large_sections 
    + even_tile_count*even_sections
    + odd_tile_count*odd_sections
}

#[allow(unused)]
fn print_grid(grid: &[Vec<Tile>], tiles: &[(usize, usize)]) {
    let mut tmp_grid: Vec<Vec<char>> = grid.iter().cloned()
        .map(|a| {
            a.into_iter().map(|ch| {
                match ch {
                    Tile::Rock => '#',
                    Tile::Garden => '.',
                }
            }).collect()
        }).collect();
    
    for tile in tiles {
        tmp_grid[tile.1][tile.0] = 'O'
    }

    for line in tmp_grid {
        let line: String = line.into_iter().collect();
        println!("{}", line);
    }
}
fn recurse_tiles(starting: &[(i64, i64)], visited: &mut HashSet<(i64, i64)>, grid: &[Vec<Tile>]) {
    for pos in starting {
        let around = [(0,1), (0,-1), (1,0),(-1,0)];
        for dir in around {
            let x = pos.0 + dir.0;
            let y = pos.1 + dir.1;

            
            let a = x.rem_euclid(grid[0].len() as i64);
            let b = y.rem_euclid(grid.len() as i64);
            if grid[b as usize][a as usize] == Tile::Rock {
                continue;
            }

            visited.insert((x,y));
        }
    }
}