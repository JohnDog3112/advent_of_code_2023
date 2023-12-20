use std::{fs, collections::HashMap};

mod part1;
mod part2;

fn main() {
    let inp = parse_input();

    let sol1 = part1::solution(&inp);
    println!("part1: {sol1}");

    let sol2 = part2::solution(&inp);
    println!("part2: {sol2}");
}

#[derive(Debug, Clone)]
pub struct Map {
    pub path: Vec<Direction>,
    pub nodes: HashMap<String, Node>,
}
#[derive(Debug, Clone)]
pub struct Node {
    pub left: String,
    pub right: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right
}
fn parse_input() -> Map{
    let inp = fs::read_to_string("src/bin/day8/input.txt").expect("Error in reading the file");

    let parts: Vec<&str> = inp.split("\n\n").collect();

    let path: Vec<Direction> = parts[0].chars().map(|dir| {
        match dir {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!()
        }
    }).collect();

    let nodes = parts[1].split('\n').map(|node| {
        let parts: Vec<&str> = node.split(" = ").collect();

        let node_id = parts[0].to_string();
        
        let directions: Vec<&str> = parts[1].split(", ").collect();
        let left = directions[0][1..].to_string();
        let right = directions[1][..3].to_string();

        (node_id, Node {left, right})
    });

    let nodes: HashMap<String, Node> = HashMap::from_iter(nodes);

    Map {
        path,
        nodes,
    }

}