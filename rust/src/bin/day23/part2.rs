use std::collections::{HashSet, HashMap};

use crate::Tile;


pub fn solution(inp: &Vec<Vec<Tile>>) -> usize {

    //search(inp, HashSet::new(), (1, 0), (0,-1)).0

    let mut tmp = HashMap::new();
    find_connections(inp, (1,0), &mut tmp, (1,0), Dir::South);

    /*println!("hi! {}", tmp.len());

    for (i, l) in inp.iter().enumerate() {
        let l = l.iter().enumerate().map(|(j, ch)| {
            if tmp.contains_key(&(j, i)) {
                'O'
            } else if *ch == Tile::Forest {
                '#'
            } else {
                '.'
            }
        }).collect::<String>();

        println!("{l}");
    }*/
    let mut traversed = HashSet::new();
    search_connections(&tmp, &mut traversed, (1,0), (inp[0].len()-2, inp.len()-1)).unwrap()
}


#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}
impl Dir {
    pub fn rev(self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
    pub fn get_offset(self) -> (i32, i32) {
        match self {
            Dir::North => (0, -1),
            Dir::South => (0, 1),
            Dir::East => (1, 0),
            Dir::West => (-1, 0)
        }
    }
    pub fn from_offset(a: (i32, i32)) -> Self {
        match a {
            (0, -1) => Dir::North,
            (0, 1) => Dir::South,
            (1, 0) => Dir::East,
            (-1, 0) => Dir::West,
            _ => unreachable!()
        }
    }
}

type Connections = HashMap<(usize, usize), HashMap<Dir, ((usize, usize), usize)>>;

fn search_connections(connections: &Connections, traversed: &mut HashSet<(usize, usize)>, cur: (usize, usize), end_pos: (usize, usize)) -> Option<usize> {
    if cur == end_pos {
        return Some(0);
    }

    traversed.insert(cur);
    let connection = connections.get(&cur).unwrap();

    let mut found_valid = false;
    let mut max = 0;
    for (pos, dist) in connection.values() {
        if traversed.contains(pos) {
            continue;
        }
        if let Some(a) = search_connections(connections, traversed, *pos, end_pos) {
            let tmp = dist + a;
            if tmp > max {
                max = tmp;
            }
            found_valid = true;
        }
    }

    traversed.remove(&cur);

    if found_valid {
        Some(max)
    } else {
        None
    }
}
fn find_connections(grid: &Vec<Vec<Tile>>, pos: (usize, usize), connections: &mut Connections, prev: (usize, usize), prev_direction: Dir) {
    let mut cur_pos = pos;
    let mut traversed = 0;
    let mut last_dir = prev_direction.rev().get_offset();
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
            //println!("{x}, {y}, {:?}", grid[y as usize][x as usize]);

            if grid[y as usize][x as usize] != Tile::Forest {
                dir_count += 1;
                found_tiles.push((x as usize, y as usize));
            }

        }

        traversed += 1;

        if dir_count > 1 {
            if let Some(node) = connections.get_mut(&prev) {
                node.insert(prev_direction, (cur_pos, traversed));
            } else {
                let mut tmp = HashMap::new();
                tmp.insert(prev_direction, (cur_pos, traversed));
                connections.insert(prev, tmp);
            }

            let from = Dir::from_offset(last_dir);
            if let Some(node) = connections.get_mut(&cur_pos) {
                node.insert(from, (prev, traversed));
            } else {
                let mut tmp = HashMap::new();
                tmp.insert(from, (prev, traversed));
                connections.insert(cur_pos, tmp);
            }

            let directions = [Dir::North, Dir::South, Dir::East, Dir::West];
            //println!("{:?}", cur_pos);
            for dir in directions {
                if dir == from {
                    continue;
                }
                if connections.get(&cur_pos).unwrap().contains_key(&dir) {
                    continue;
                }
                let off = dir.get_offset();
                let x = cur_pos.0 as i32 + off.0;
                let y = cur_pos.1 as i32 + off.1;
                if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                    continue;
                }
                let pos = (x as usize, y as usize);
                //println!("pos: {:?}", pos);

                if grid[pos.1][pos.0] == Tile::Forest {
                    continue;
                }
                find_connections(grid, pos, connections, cur_pos, dir);
                
            }
            return;
        }

        if found_tiles.is_empty() {
            traversed -= 1;
            if cur_pos == (grid[0].len()-2, grid.len()-1) {
                //println!("found! {:?} {:?}, {:?}", prev, prev_direction, cur_pos);
                if let Some(node) = connections.get_mut(&prev) {
                    node.insert(prev_direction, (cur_pos, traversed));
                } else {
                    let mut tmp = HashMap::new();
                    tmp.insert(prev_direction, (cur_pos, traversed));
                    connections.insert(prev, tmp);
                }
    
                let from = Dir::from_offset(last_dir);
                if let Some(node) = connections.get_mut(&cur_pos) {
                    node.insert(from, (prev, traversed));
                } else {
                    let mut tmp = HashMap::new();
                    tmp.insert(from, (prev, traversed));
                    connections.insert(cur_pos, tmp);
                }
                return;
            } else {
                return;
            }
        }

        last_dir = (
            cur_pos.0 as i32 - found_tiles[0].0 as i32,
            cur_pos.1 as i32 - found_tiles[0].1 as i32,
        );

        cur_pos = found_tiles[0];


    }
}
