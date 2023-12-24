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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HailStone {
    pub pos: (i64, i64, i64),
    pub vel: (i64, i64, i64),
}
fn parse_input() -> Vec<HailStone> {
    let inp: String = fs::read_to_string("src/bin/day24/input.txt").expect("Error in reading the file");

    inp.split('\n').map(|line| {
        let parts: Vec<(i64,i64,i64)> = line.split(" @ ").map(|part| {
            let nums: Vec<i64> = part.split(", ").map(|num| num.trim().parse().unwrap()).collect();

            (nums[0], nums[1], nums[2])
        }).collect();

        HailStone {
            pos: parts[0],
            vel: parts[1],
        }

    }).collect()

}