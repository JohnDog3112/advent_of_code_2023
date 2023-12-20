use std::{fs, collections::HashMap};

mod part1;
mod part2;
fn main() {
    let inp = parse_input();

    let sol1 = part1::solution(&inp);
    println!("part 1: {sol1}");

    let sol2 = part2::solution(&inp);
    println!("part 2: {sol2}");
}

fn parse_input() -> Vec<Vec<i32>> {
    let inp = fs::read_to_string("src/bin/day17/input.txt").expect("Error in reading the file");
    
    inp.split('\n').map(|line| {
        line.chars().map(|ch| 
            ch.to_digit(10).unwrap() as i32
        ).collect()
    }).collect()
}

#[derive(Debug, Clone)]
pub struct Node {
    pub connections: HashMap<(Dir, i32), i32>
}

impl Node {
    pub fn get_min_weight(&self) -> i32{
        self.connections.values().copied().min().unwrap_or(0)
    }
}
fn print_grid(grid: &Vec<Vec<Node>>) {
    for line in grid {
        let str = line.iter().map(|node| {
            format!("{:0>3} ", node.get_min_weight())
        }).collect::<String>();
        println!("{str}");
    }

    let mut point = (grid[0].len() - 1, grid.len() - 1);

    let mut new_grid = vec![vec!['.'; grid[0].len()]; grid.len()];

    new_grid[grid.len() - 1][grid[0].len()-1] = '#';
    
    let mut prev_dir = Dir::None;

    while point != (0,0) && point != (1,0) && point != (0,1) {
        let mut dir = Dir::None;
        let mut min_weight = -1;
        let mut min_consecutive = 0;
        for ((next_dir,consecutive), weight) in grid[point.1][point.0].connections.iter() {
            if (min_weight == -1 || *weight <= min_weight) && *next_dir != prev_dir {
                min_consecutive = *consecutive;
                min_weight = *weight;
                dir = next_dir.get_opposite();
            }
        }

        prev_dir = dir.get_opposite();
        let offset = dir.get_offset();

        let ch = match prev_dir {
            Dir::None => unreachable!(),
            Dir::Right => '>',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Up => '^',
        };
        for i in 0..min_consecutive+1 {
            let x = point.0 as i32 + offset.0*(i+1);
            let y = point.1 as i32 + offset.1*(i+1);
            new_grid[y as usize][x as usize] = ch;
        }

        
        let x = point.0 as i32 + offset.0 * (min_consecutive + 1);
        let y = point.1 as i32 + offset.1 * (min_consecutive + 1);

        //println!("{}, {}", x ,y );
        point = (x as usize, y as usize);
    }

    for line in new_grid {
        let line = line.into_iter().collect::<String>();
        println!("{line}");
    }
}
#[derive(Debug, Clone)]
pub struct Process {
    pub x: i32,
    pub y: i32,
    pub dir: Dir,
    pub consecutive: i32,
    pub weight: i32,
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
impl Eq for Process {}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (-self.weight).partial_cmp(&(-other.weight))
    }
}
impl Ord for Process{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    None,
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    const DIRS: [Dir; 4] = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];
    
    pub fn get_opposite(self) -> Self {
        match self {
            Dir::None => Dir::None,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
        }
    }
    pub fn get_other_dirs(self) -> Vec<Self> {
        let ignore = self.get_opposite();
        Self::DIRS.into_iter().filter(|a| *a != ignore).collect()
    }

    pub fn get_offset(&self) -> (i32, i32) {
        match self {
            Dir::None => (0,0),
            Dir::Right => (1,0),
            Dir::Down => (0,1),
            Dir::Left => (-1,0),
            Dir::Up => (0,-1),
        }
    }
}