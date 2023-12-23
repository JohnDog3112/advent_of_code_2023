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



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Ground,
    Forest,
    DownSlope,
    UpSlope,
    RightSlop,
    LeftSlope
}
fn parse_input() -> Vec<Vec<Tile>> {
    let inp: String = fs::read_to_string("src/bin/day23/input.txt").expect("Error in reading the file");

    
    inp.split('\n').map(|line| {
        line.chars().map(|ch| {
            match ch {
                '.' => Tile::Ground,
                '#' => Tile::Forest,
                'v' => Tile::DownSlope,
                '^' => Tile::UpSlope,
                '>' => Tile::RightSlop,
                '<' => Tile::LeftSlope,
                _ => unreachable!(),
            }
        }).collect()
    }).collect()

}