use std::{collections::HashSet, fs};

mod part1;
mod part2;
fn main() {
    //let parse_input = Instant::now();
    let inp = get_input();
    //let parse_input = parse_input.elapsed();

    //println!("{:?}", inp);
    //let sol1_time = Instant::now();
    let sol1 = part1::solution(&inp);
    //let sol1_time = sol1_time.elapsed();
    
    println!("part1: {sol1}");

    //let sol2_time = Instant::now();
    let sol2 = part2::solution(&inp);
    //let sol2_time = sol2_time.elapsed();

    println!("part2: {sol2}");

    //println!("parse_time: {:?}", parse_input);
    //println!("part1_time: {:?}", sol1_time);
    //println!("part2_time: {:?}", sol2_time);

    //println!("total part1_time: {:?}", parse_input+sol1_time);
    //println!("total part2_time: {:?}", parse_input+sol2_time);



}

#[derive(Clone, Debug)]
pub struct Card {
    pub winning_numbers: Vec<i32>,
    pub selected_numbers: Vec<i32>
}

impl Card {
    pub fn get_winning_amt(&self) -> i32 {
        let winning_numbers: HashSet<&i32> = HashSet::from_iter(self.winning_numbers.iter());
        
        let amt = self.selected_numbers.iter().filter(move |&num| winning_numbers.contains(num)).count();

        amt as i32
    }
}

pub fn get_input() -> Vec<Card> {
    let inp = fs::read_to_string("src/bin/day4/input.txt").expect("Error in reading the file");

    let cards = inp.split('\n').map(|line| {
        let numbers = line.split(": ").last().unwrap();

        let mut group = numbers.split(" | ").map(|part| {
            part.split_ascii_whitespace().map(|num| num.parse::<i32>().unwrap()).collect()
        }).collect::<Vec<Vec<i32>>>();

        Card {
            winning_numbers: group.remove(0),
            selected_numbers: group.remove(0)
        }
    });

    cards.collect()
}