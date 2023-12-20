use crate::{Hand,IncompleteHand, HandType};

pub fn solution(inp: Vec<IncompleteHand>) -> i32 {

    let mut hands = inp.into_iter().map(|hand| {
        let hand_grabber: Vec<i32> = hand.hashed_cards.into_values().collect();
        let hand_type = HandType::calculate(hand_grabber);
        Hand {
            bid: hand.bid,
            cards: hand.cards,
            hand_type
        }
    }).collect::<Vec<Hand>>();
    

    hands.sort();
    
    hands.into_iter().enumerate()
        .map(|(i, hand)| hand.bid * (i as i32+1))
        .sum()
}