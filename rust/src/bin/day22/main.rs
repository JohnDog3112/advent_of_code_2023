use std::fs;

mod part1;
mod part2;
fn main() {
    let mut inp = parse_input();

    inp.sort();


    let sol1 = part1::solution(inp.clone());
    println!("part 1: {sol1}");
    
    let sol2 = part2::solution(inp);
    println!("part 2: {sol2}");
    
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Brick {
    pub x: Pos,
    pub y: Pos,
    pub z: Pos,
}

impl Brick {
    pub fn is_below(&self, oth: &Brick) -> bool {
        let z = self.z.higher() < oth.z.lower();

        let y = self.y.intersect(oth.y);

        let x = self.x.intersect(oth.x);
        
        x && y && z
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.z.partial_cmp(&other.z) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pos {
    Int(i64),
    Range(i64, i64)
}

impl Pos {
    pub fn new(a: i64, b: i64) -> Self{
        match a.cmp(&b) {
            std::cmp::Ordering::Less => Self::Range(a, b),
            std::cmp::Ordering::Equal => Self::Int(a),
            std::cmp::Ordering::Greater => Self::Range(b, a),
        }
    }
    pub fn lower(self) -> i64 {
        match self {
            Pos::Int(a) => a,
            Pos::Range(a, _) => a,
        }
    }
    pub fn higher(self) -> i64 {
        match self {
            Pos::Int(a) => a,
            Pos::Range(_, a) => a,
        }
    }
    pub fn intersect(self, oth: Self) -> bool {
        match self {
            Pos::Int(a) => {
                match oth {
                    Pos::Int(b) => a == b,
                    Pos::Range(b, c) => {
                        a >= b && a <= c
                    },
                }
            },
            Pos::Range(a, b) => {
                match oth {
                    Pos::Int(c) => {
                        c >= a && c <= b
                    },
                    Pos::Range(c,d) => {
                        c >= a && c <= b
                        || d >= a && d <= b
                        || a >= c && a <= d
                        || b >= c && b <= d
                    }
                }
            },
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.higher().partial_cmp(&other.higher())
    }
}
fn parse_input() -> Vec<Brick> {
    let inp: String = fs::read_to_string("src/bin/day22/input.txt").expect("Error in reading the file");

    inp.split('\n').map(|line| {
        let mut parts = line.split('~').map(|bricks| {
            bricks.split(',').map(|a| a.parse::<i64>().unwrap())
        });

        let poses: Vec<Pos> = parts.next().unwrap()
            .zip(parts.next().unwrap())
            .map(|(a,b)|Pos::new(a,b)).collect();

        Brick {
            x: poses[0],
            y: poses[1],
            z: poses[2]
        }
    }).collect()
}