use crate::Card;

pub fn solution(inp: &[Card]) -> i32 {
    inp.iter().map(|card| {
        let amt = card.get_winning_amt();
        if amt != 0 {
            2_i32.pow((amt-1) as u32)
        } else {
            0
        }
    }).sum()
}