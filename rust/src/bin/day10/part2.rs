use crate::{Tile, try_move, Cord, Dir};

const ALL_DIRECTIONS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

pub fn solution(inp: &Vec<Vec<Tile>>) -> usize {
    let mut start = Cord { x: -1, y: -1};
    'outer: for (y, row) in inp.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if let Tile::Start = val {
                start = Cord {x: x as i128, y: y as i128};
                break 'outer;
            }
        }
    }

    let mut directions = vec![];

    for dir in ALL_DIRECTIONS {
        let cord = start + dir.to_cord();
        if cord.y < 0 || cord.x < 0 || cord.y >= inp.len() as i128 || cord.x >= inp[0].len() as i128 {
            continue;
        }
        let tile = &inp[cord.y as usize][cord.x as usize];
        if try_move(tile, dir).is_some() {
            directions.push(dir);
        }
    }


    //println!("{:?}, {:?}", directions, inside);
    
    let mut weights = crate::part1::generate_weights(inp);

    let mut cur_cord = start;
    let mut cur_dir = directions[0];
    let mut prev_dir = directions[0];

    loop {        

        let inside_dir = get_inside(cur_dir);

        let inside_tile = cur_cord + inside_dir.to_cord();
        
        get_connected_tiles(&mut weights, inside_tile);

        let inside_dir = get_inside(prev_dir);

        let inside_tile = cur_cord + inside_dir.to_cord();
        
        get_connected_tiles(&mut weights, inside_tile);

        cur_cord = cur_cord + cur_dir.to_cord();


        let tile = &inp[cur_cord.y as usize][cur_cord.x as usize];

        if cur_cord == start {
            break;
        }
        prev_dir = cur_dir;
        cur_dir = try_move(tile, cur_dir).unwrap();

    }

    let mut inside_num = -1;
    for line in &weights{
        if line[0] == -1 || line[line.len()-1] == -1 {
            inside_num = 0;
            break;
        }
    }
    for i in 0..weights[0].len() {
        if weights[0][i] == -1 || weights[weights.len()-1][i] == -1 {
            inside_num = 0;
            break;
        }
    }

    /*for (i, line) in weights.iter().enumerate() {
        let str = line.iter().enumerate().map(|(j,ch)| {
            if *ch == inside_num {
                'I'
            } else if *ch == 0 || *ch == -1 {
                'O'
            } else {
                match inp[i][j] {
                    Tile::NorthSouth => '|',
                    Tile::EastWest => '─',
                    Tile::NorthEast => '└',
                    Tile::NorthWest => '┘',
                    Tile::SouthWest => '┐',
                    Tile::SouthEast => '┌',
                    Tile::Start => 'S',
                    _ => unreachable!()
                }
            }
        }).collect::<String>();
        println!("{str}");
    }*/
    
    weights.into_iter().map(|line| line.into_iter().filter(|a| *a == inside_num).count()).sum()
}
fn get_connected_tiles(weights: &mut Vec<Vec<i32>>, tile: Cord) {
    if tile.x < 0 || tile.y < 0 || tile.y >= weights.len() as i128 || tile.x >= weights[0].len() as i128 {
        return;
    }
    if weights[tile.y as usize][tile.x as usize] != 0 {
        return;
    }
    weights[tile.y as usize][tile.x as usize] = -1;
    
    let mut to_process = vec![tile];

    while let Some(next) = to_process.pop() {
        for dir in ALL_DIRECTIONS {
            let new_cord = next + dir.to_cord();
            if new_cord.x < 0 || new_cord.y < 0 || new_cord.y >= weights.len() as i128 || new_cord.x >= weights[0].len() as i128 {
                continue;
            }
            if 0 == weights[new_cord.y as usize][new_cord.x as usize] {
                weights[new_cord.y as usize][new_cord.x as usize] = -1;
                to_process.push(new_cord);
            }
        }
    }
}


fn get_inside(dir: Dir) -> Dir {
    match dir {
        Dir::North => Dir::East,
        Dir::East => Dir::South,
        Dir::South => Dir::West,
        Dir::West => Dir::North,
    }
}

