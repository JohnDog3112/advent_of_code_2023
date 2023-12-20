use crate::{IncompleteHand, HandType, Hand};

pub fn solution(inp: Vec<IncompleteHand>) -> i32 {

    let mut hands = inp.into_iter().map(|mut hand| {
        let (&max_card, &max_count) = hand.hashed_cards.iter().reduce(|card_a, card_b| {
            if *card_a.0 == 10 {
                return card_b;
            } else if *card_b.0 == 10 {
                return card_a;
            }
            match card_a.1.cmp(card_b.1) {
                std::cmp::Ordering::Less => card_b,
                std::cmp::Ordering::Greater => card_a,
                std::cmp::Ordering::Equal => card_a,
            }
        }).unwrap();
        if max_card != 10 {
            if let Some(jokers) = hand.hashed_cards.remove(&10) {
                hand.hashed_cards.insert(max_card, max_count+jokers);
            }
        }
        let hand_grabber: Vec<i32> = hand.hashed_cards.into_values().collect();
        let hand_type = HandType::calculate(hand_grabber);
        let cards: [u8; 5] = hand.cards.into_iter().map(|card| {
            if card == 10 {
                0
            } else {
                card
            }
        }).collect::<Vec<u8>>()[0..5].try_into().unwrap();
        Hand {
            bid: hand.bid,
            cards,
            hand_type
        }
    }).collect::<Vec<Hand>>();
    
    hands.sort();
    
    hands.into_iter().enumerate()
        .map(|(i, hand)| hand.bid * (i as i32+1))
        .sum()
}
