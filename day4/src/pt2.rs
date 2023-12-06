use crate::ScratchCard;

fn process_copy(cards: Vec<i32>, start:  i32, count: i32) -> i32 {
    let mut result = 0;
    for i in start..count {
        let winners = cards[i];
        if winners < 1 {
            continue;
        }
        result += 1;
        result += process_copy(cards.clone())
    }
    result
}

pub fn process_cards(text: &str) -> i32 {
    let cards: Vec<i32> = text.lines().map(|line| ScratchCard::new(line).winning_count()).collect();
    let mut result = 0;
    for i in 0..cards.len() {
        let winner_count = cards[i];
        if winner_count < 1 {
            continue
        }
        result += 1;
        for j in 1..=winner_count {
            let copy_winner =
        }
    }
    result
}