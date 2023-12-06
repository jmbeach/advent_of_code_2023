mod pt2;

use std::collections::HashSet;
use std::fs;

fn main() {
    let total_score: i32 = fs::read_to_string("./input.txt").unwrap().lines().map(|line| ScratchCard::new(line).score()).sum();
    println!("Result: {total_score}")
}

struct ScratchCard {
    pub numbers: Vec<i32>,
    pub winning_numbers: HashSet<i32>,
}

impl ScratchCard {
    pub fn new(text: &str) -> Self {
        let without_id_split = text.split(":");
        let without_id = without_id_split.last().unwrap();
        let mut parts = without_id.split("|");
        let left = parts.next().unwrap().trim();
        let right = parts.next().unwrap().trim();
        let left_split = left.split_whitespace();
        let right_split = right.split_whitespace();
        let numbers = left_split.map(|x| x.parse::<i32>().unwrap()).collect();
        let winning_numbers: HashSet<i32> = right_split.map(|x| x.parse::<i32>().unwrap()).collect();
        Self {
            numbers,
            winning_numbers
        }
    }

    pub fn score(&self) -> i32 {
        let mut winning_numbers: i32 = -1;
        for number in self.numbers.clone() {
            if self.winning_numbers.contains(&number) {
                winning_numbers += 1;
            }
        }
        match winning_numbers {
            -1 => 0,
            _ => i32::pow(2, winning_numbers as u32)
        }
    }

    pub fn winning_count(&self) -> i32 {
        let mut winning_numbers: i32 = 0;
        for number in self.numbers.clone() {
            if self.winning_numbers.contains(&number) {
                winning_numbers += 1;
            }
        }
        winning_numbers
    }
}

#[cfg(test)]
mod tests4 {
    use std::collections::HashSet;
    use crate::ScratchCard;

    #[test]
    fn basic_test() {
        let card = ScratchCard::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card.numbers, vec!(41, 48, 83, 86, 17));
        let winning_set: HashSet<i32> = vec!(83, 86, 6, 31, 17, 9, 48, 53).into_iter().collect();
        assert_eq!(card.winning_numbers, winning_set);
        assert_eq!(card.score(), 8)
    }
}