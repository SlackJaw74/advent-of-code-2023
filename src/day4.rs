use std::collections::HashSet;

use regex::Regex;

use crate::file_helper::get_file_contents;

pub fn execute_day_4_a(file_path: &str) -> i32 {
    let lines = get_file_contents(file_path);
    let total_score = lines
        .iter()
        .map(|line| Card::new(line))
        .map(|mut x| x.calculate_card_score())
        .sum();
    return total_score;
}

pub fn execute_day_4_b(file_path: &str) -> usize {
    let lines = get_file_contents(file_path);

    // create the cards
    let mut cards = lines.iter().map(|line| Card::new(line)).collect::<Vec<_>>();

    // calculate the score for each card
    cards
        .iter_mut()
        .for_each(|x| x.calculate_card_winning_numbers());

    let card_count = cards.len();

    for index in 0..card_count {
        let card = cards.get_mut(index).expect("Missing card!");
        let card_copies = card.copies;

        for n in 0..card.count_winning_numbers {
            cards
                .get_mut(index + n + 1)
                .and_then(|next_card| Some(next_card.copies += card_copies));
        }
    }

    let card_count = cards.iter().fold(0, |acc, card| acc + card.copies);
    // get the total score taking in to account duplicate count
    return card_count;
}

#[derive(Default, Debug, Clone)]
pub struct Card {
    pub number: usize,
    pub winning_numbers: HashSet<i32>,
    pub ticket_numbers: HashSet<i32>,
    pub score: i32,
    pub count_winning_numbers: usize,
    pub copies: usize,
}
impl Card {
    pub fn new(line: &str) -> Self {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let lottery_card_regex = Regex::new(
            r"Card[\s]+(?<card_number>[\d]+): (?<winning_numbers>[\d\s]+)\|(?<ticket_numbers>[\d\s]+)",
        )
        .unwrap();
        let captures = lottery_card_regex.captures(line).unwrap();
        let number = captures.name("card_number").map_or(0, |capture| {
            capture.as_str().parse::<usize>().expect("bad card number")
        });
        let winning_numbers = captures
            .name("winning_numbers")
            .map(|capture| {
                capture
                    .as_str()
                    .trim()
                    .split(" ")
                    .flat_map(|value| value.to_string().parse::<i32>())
            })
            .expect("failed to get winning numbers")
            .collect();
        let ticket_numbers = captures
            .name("ticket_numbers")
            .map(|capture| {
                capture.as_str().trim().split(" ").flat_map(|value| {
                    let temp = value.to_string();
                    temp.parse::<i32>()
                })
            })
            .expect("failed to get ticket numbers")
            .collect();
        Card {
            number,
            winning_numbers,
            ticket_numbers,
            score: 0,
            count_winning_numbers: 0,
            copies: 1,
        }
    }

    pub fn calculate_card_winning_numbers(&mut self) {
        let ticket_numbers = &self.ticket_numbers;
        let winning_numbers = &self.winning_numbers;
        let card_score = ticket_numbers
            .intersection(winning_numbers)
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        self.count_winning_numbers = card_score;
    }

    pub fn calculate_card_score(&mut self) -> i32 {
        let ticket_numbers = &self.ticket_numbers;
        let winning_numbers = &self.winning_numbers;
        let card_score = ticket_numbers
            .intersection(winning_numbers)
            .fold(0, |acc, _| {
                if acc == 0 {
                    return 1;
                } else {
                    return acc + acc;
                }
            });
        self.score = card_score;
        return card_score;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn execute_day_4_a_mini() {
        let result = execute_day_4_a("./input/day-4-test.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn execute_day_4_b_mini() {
        let result = execute_day_4_b("./input/day-4-b-test.txt");
        assert_eq!(result, 30);
    }
}
