use std::{iter, collections::HashSet};

use regex::Regex;

use crate::file_helper::get_file_contents;

pub fn execute_day_4_a(file_path: &str) -> i32 {
    let lines = get_file_contents(file_path);
    let total_score = lines
        .iter()
        .map(|line| Card::new(line))
        .map(|x| x.get_card_score())
        .sum();
    return total_score;
}

#[derive(Default, Debug)]
pub struct Card {
    pub number: usize,
    pub winning_numbers: HashSet<i32>,
    pub ticket_numbers: HashSet<i32>,
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
                capture.as_str().trim().split(" ").flat_map(|value| {
                    value
                        .to_string()
                        .parse::<i32>()
                })
            })
            .expect("failed to get winning numbers")
            .collect();
        let ticket_numbers = captures
            .name("ticket_numbers")
            .map(|capture| {
                capture.as_str().trim().split(" ").flat_map(|value| {
                    let temp = value.to_string();
                    println!("value {}", temp);
                    temp.parse::<i32>()
                })
            })
            .expect("failed to get ticket numbers")
            .collect();
        Card {
            number,
            winning_numbers,
            ticket_numbers,
        }
    }
    pub fn get_card_score(&self) -> i32 {
        let ticket_numbers = &self.ticket_numbers;
        let winning_numbers = &self.winning_numbers;
        let card_score = ticket_numbers.intersection(winning_numbers).fold(0, |acc, _| {
            if acc == 0 {
                return 1;
            } else {
                return acc + acc;
            }
         });
        println!("card_score={}", card_score);
        return card_score;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use insta::*;

    #[test]
    fn execute_day_4_a_mini() {
        let result = execute_day_4_a("./input/day-4-test.txt");
        assert_eq!(result, 13);
    }
}
