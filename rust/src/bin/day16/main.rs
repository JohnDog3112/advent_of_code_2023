use std::fs;

mod part1;
mod part2;
fn main() {
    let inp = parse_input();

    let sol1 = part1::solution(&inp);
    println!("part 1: {sol1}");

    let sol2 = part2::solution(&inp);
    println!("part 2: {sol2}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeamDir {
    Up,
    Right,
    Down,
    Left,
}

impl BeamDir {
    pub fn get_next(self, tile: &Tile) -> Vec<BeamDir> {
        match tile {
            Tile::Empty => vec![self],
            Tile::SplitUp => {
                match self {
                    BeamDir::Right
                    | BeamDir::Left => vec![BeamDir::Up, BeamDir::Down],
                    _ => vec![self],
                }
            },
            Tile::SplitSide => {
                match self {
                    BeamDir::Up
                    | BeamDir::Down => vec![BeamDir::Right, BeamDir::Left],
                    _ => vec![self],
                }
            },
            Tile::MirrorForward => {
                match self {
                    BeamDir::Up => vec![BeamDir::Right],
                    BeamDir::Right => vec![BeamDir::Up],
                    BeamDir::Down => vec![BeamDir::Left],
                    BeamDir::Left => vec![BeamDir::Down]
                }
            },
            Tile::MirrorBackward => {
                match self {
                    BeamDir::Up => vec![BeamDir::Left],
                    BeamDir::Right => vec![BeamDir::Down],
                    BeamDir::Down => vec![BeamDir::Right],
                    BeamDir::Left => vec![BeamDir::Up]
                }
            },
        }
    }
    pub fn get_offset(&self) -> (i32, i32) {
        match self {
            BeamDir::Up => (0,-1),
            BeamDir::Right => (1,0),
            BeamDir::Down => (0,1),
            BeamDir::Left => (-1,0),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    SplitUp,
    SplitSide,
    MirrorForward,
    MirrorBackward
}



fn parse_input() -> Vec<Vec<Tile>> {
    let inp = fs::read_to_string("src/bin/day16/input.txt").expect("Error in reading the file");

    inp.split('\n').map(|line| {
        line.chars().map(|ch| {
            match ch {
                '.' => Tile::Empty,
                '|' => Tile::SplitUp,
                '-' => Tile::SplitSide,
                '/' => Tile::MirrorForward,
                '\\' => Tile::MirrorBackward,
                _ => unreachable!()
            }
        }).collect()
    }).collect()
}