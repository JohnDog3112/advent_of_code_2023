use crate::Card;

pub fn solution(inp: &[Card]) -> i32 {
    let mut card_amt = vec![1; inp.len()];

    for (i, card) in inp.iter().enumerate() {
        let amt = card.get_winning_amt() as usize;
        for j in i+1..(i+1+amt).min(card_amt.len()) {
            card_amt[j] += card_amt[i];
        }
    }
    card_amt.into_iter().sum()
}