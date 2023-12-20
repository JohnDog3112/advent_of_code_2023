use std::{fs, collections::HashMap};
mod part1;
mod part2;


fn main() {
    let inp = parse_input();

    let sol1 = part1::solution(inp.clone());

    println!("part1: {sol1}");

    let sol2 = part2::solution(inp);
    println!("part2: {sol2}");
    //println!("{:?}", inp);
}


#[derive(Debug, Clone)]
pub struct Hand {
    hand_type: HandType,
    cards: [u8; 5],
    pub bid: i32,
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        for i in 0..self.cards.len() {
            match self.cards[i].partial_cmp(&other.cards[i]) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
        }
        
        Some(core::cmp::Ordering::Equal)
    }
}
impl Eq for Hand {

}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}
impl HandType {
    pub fn calculate(hand_grabber: Vec<i32>) -> Self {
        match (hand_grabber.len(), hand_grabber.into_iter().max().unwrap()) {
            (5, _) => HandType::HighCard,
            (4, _) => HandType::OnePair,
            (3, 2) => HandType::TwoPair,
            (3, 3) => HandType::ThreeOfKind,
            (2, 3) => HandType::FullHouse,
            (2, 4) => HandType::FourOfKind,
            (1, _) => HandType::FiveOfKind,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
pub struct IncompleteHand {
    pub cards: [u8; 5],
    pub bid: i32,
    pub hashed_cards: HashMap<u8, i32>,
}
fn parse_input() -> Vec<IncompleteHand> {
    let inp = fs::read_to_string("src/bin/day7/input.txt").expect("Error in reading the file");

    inp.split('\n').map(|line| {
        let [cards, bid]: [&str; 2] = line.split(' ').collect::<Vec<&str>>()[0..2].try_into().unwrap();
        let cards: Vec<u8> = cards.chars().map(|card| {
            match card {
                '2'..='9' => card as u8 - b'2' + 1,
                'T' => 9,
                'J' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => unreachable!()
            }
        }).collect();

        let bid = bid.parse().unwrap();

        let hashed_cards = {
            let mut tmp = HashMap::new();
            for card in &cards {
                if let Some(i) = tmp.get_mut(card) {
                    *i += 1;
                } else {
                    tmp.insert(*card, 1);
                }
            }
            tmp
        };
        
        //println!("{:?}", hand_type);
        IncompleteHand {
            cards: cards[0..5].try_into().unwrap(),
            bid,
            hashed_cards
        }
    }).collect()
}