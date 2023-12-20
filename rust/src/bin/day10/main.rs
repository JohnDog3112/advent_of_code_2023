use std::{fs, ops::Add};

mod part1;
mod part2;
fn main() {
    let inp = parse_input();
    //println!("{:?}", inp);

    let sol1 = part1::solution(&inp);
    println!("part1: {sol1}");

    let sol2 = part2::solution(&inp);
    println!("part2: {sol2}");
}

#[derive(Clone, Debug)]
pub enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}
fn parse_input() -> Vec<Vec<Tile>> {
    let inp = fs::read_to_string("src/bin/day10/input.txt").expect("Error in reading the file");

    inp.split('\n').map(|line| {
        line.chars().map(|ch| {
            match ch {
                '|' => Tile::NorthSouth,
                '-' => Tile::EastWest,
                'L' => Tile::NorthEast,
                'J' => Tile::NorthWest,
                '7' => Tile::SouthWest,
                'F' => Tile::SouthEast,
                '.' => Tile::Ground,
                'S' => Tile::Start,
                _ => unreachable!()
            }
        }).collect()
    }).collect()
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cord {
    x: i128,
    y: i128,
}
impl Add for Cord {
    type Output = Cord;

    fn add(self, rhs: Self) -> Self::Output {
        Cord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Dir {
    North,
    South,
    East,
    West,
}
impl Dir {
    pub fn to_cord(self) -> Cord {
        match self {
            Dir::North => Cord {x: 0, y: -1},
            Dir::South => Cord {x: 0, y: 1},
            Dir::East => Cord {x: 1, y: 0},
            Dir::West => Cord {x: -1, y: 0},
        }
    }
}

pub fn try_move(tile: &Tile, dir: Dir) -> Option<Dir> {
    match (tile, dir) {
        (Tile::EastWest, Dir::East) => Some(Dir::East),
        (Tile::EastWest, Dir::West) => Some(Dir::West),

        (Tile::NorthEast, Dir::South) => Some(Dir::East),
        (Tile::NorthEast, Dir::West) => Some(Dir::North),

        (Tile::NorthSouth, Dir::South) => Some(Dir::South),
        (Tile::NorthSouth, Dir::North) => Some(Dir::North),

        (Tile::NorthWest, Dir::South) => Some(Dir::West),
        (Tile::NorthWest, Dir::East) => Some(Dir::North),

        (Tile::SouthEast, Dir::North) => Some(Dir::East),
        (Tile::SouthEast, Dir::West) => Some(Dir::South),

        (Tile::SouthWest, Dir::North) => Some(Dir::West),
        (Tile::SouthWest, Dir::East) => Some(Dir::South),
        
        _ => None
    }
}