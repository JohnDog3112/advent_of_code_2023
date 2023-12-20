use std::fs;

mod part1;
mod part2;
fn main() {
    let inp = parse_input();

    let sol1 = part1::solution(inp.clone());
    println!("part 1: {sol1}");

    let sol2 = part2::solution(inp);
    println!("part 2: {sol2}");
}

#[allow(unused)]
pub fn print_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid {
        let st = row.iter().map(|tile| 
            match tile {
                Tile::MoveRock => 'O',
                Tile::StillRock => '#',
                Tile::Empty => '.',
            }
        ).collect::<String>();

        println!("{st}");
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    MoveRock,
    StillRock,
    Empty
}
fn parse_input() -> Vec<Vec<Tile>>{
    let inp = fs::read_to_string("src/bin/day14/input.txt").expect("Error in reading the file");

    inp.split('\n').map(|line|
        line.chars().map(|ch|
            match ch {
                'O' => Tile::MoveRock,
                '#' => Tile::StillRock,
                '.' => Tile::Empty,
                _ => unreachable!()
            }
        ).collect()
    ).collect()
}
