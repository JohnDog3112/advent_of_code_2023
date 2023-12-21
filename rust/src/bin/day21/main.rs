use std::fs;

mod part1;
mod part2;
fn main() {
    let (start, grid) = parse_input();
    
    let sol1 = part1::solution(start, &grid);
    println!("part 1: {sol1}");

    let sol2 = part2::solution(start, &grid);
    println!("part 2: {sol2}");
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Rock,
    Garden,
}
fn parse_input() -> ((usize, usize), Vec<Vec<Tile>>) {
    let inp = fs::read_to_string("src/bin/day21/input.txt").expect("Error in reading the file");

    let mut start = (0,0);
    let grid = inp.split('\n').enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, ch)| {
            match ch {
                '#' => Tile::Rock,
                '.' => Tile::Garden,
                'S' => {
                    start = (x,y);
                    Tile::Garden
                }
                _ => unreachable!()
            }
        }).collect()
    }).collect();

    (start, grid)
}