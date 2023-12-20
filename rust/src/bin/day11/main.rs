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

fn parse_input() -> Vec<Vec<char>> {
    let inp = fs::read_to_string("src/bin/day11/input.txt").expect("Error in reading the file");
    
    inp.split('\n').map(|line| line.chars().collect()).collect()
}