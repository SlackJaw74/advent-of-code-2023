use crate::file_helper::get_file_contents;
use phf::phf_map;
use std::{cmp::Ordering, collections::HashMap};

static CARD_RANK: phf::Map<&'static str, &'static i32> = phf_map! {
    "A" => &13,
    "K" => &12,
    "Q" => &11,
    "J" => &10,
    "T" => &9,
    "9" => &8,
    "8" => &7,
    "7" => &6,
    "6" => &5,
    "5" => &4,
    "4" => &3,
    "3" => &2,
    "2" => &1
};

static HAND_RANK: phf::Map<&'static str, i32> = phf_map! {
    "FIVE_OF_A_KIND" => 7,
    "FOUR_OF_A_KIND" => 6,
    "FULL_HOUSE" => 5,
    "THREE_OF_A_KIND" => 4,
    "TWO_PAIR" => 3,
    "ONE_PAIR" => 2,
    "HIGH_CARD" => 1
};

// sort order A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
/*
Five of a kind, where all five cards have the same label: AAAAA
Four of a kind, where four cards have the same label and one card has a different label: AA8AA
Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
High card, where all cards' labels are distinct: 23456
 */

pub fn score_hand(sorted_hand_value: &str) -> i32 {
    // run regex on the sorted hand value
    let char_map = sorted_hand_value
        .chars()
        .into_iter()
        .fold(HashMap::new(), |mut acc, char| {
            let entry = acc.entry(char).or_insert(0);
            *entry += 1;
            return acc;
        });
    let five_of_a_kind = char_map.iter().find(|x| x.1.eq(&5)).is_some();
    if five_of_a_kind {
        return *HAND_RANK.get("FIVE_OF_A_KIND").expect("hand_rank value");
    }
    let four_of_a_kind = char_map.iter().find(|x| x.1.eq(&4)).is_some();
    if four_of_a_kind {
        return *HAND_RANK.get("FOUR_OF_A_KIND").expect("hand_rank value");
    }
    let full_house = char_map.iter().find(|x| x.1.eq(&2)).is_some() && char_map.iter().find(|x| x.1.eq(&3)).is_some();
    if full_house {
        return *HAND_RANK.get("FULL_HOUSE").expect("hand_rank value");
    }
    let three_of_a_kind = !full_house && char_map.iter().find(|x| x.1.eq(&3)).is_some();
    if three_of_a_kind {
        return *HAND_RANK.get("THREE_OF_A_KIND").expect("hand_rank value");
    }
    let pairs = char_map
        .iter()
        .filter(|x| x.1.eq(&2) && x.1.eq(&2))
        .collect::<Vec<_>>();
    if pairs.len() == 2 {
        return *HAND_RANK.get("TWO_PAIR").expect("hand_rank value");
    }
    if pairs.len() == 1 {
        return *HAND_RANK.get("ONE_PAIR").expect("hand_rank value");
    }
    return *HAND_RANK.get("HIGH_CARD").expect("hand rank value");
}

pub fn execute_day_7_a(file_path: &str) -> i32 {
    let lines = get_file_contents(file_path);
    // do something with lines
    let mut hands = lines
        .iter()
        .map(|line| {
            let split_line = line.split(" ").collect::<Vec<_>>();
            let hand_value = split_line.get(0).expect("expected card value").to_string();
            let mut chars = hand_value.chars().collect::<Vec<_>>();
            chars.sort_by(|a, b| {
                let a_rank = CARD_RANK
                    .get(&a.to_string())
                    .expect("a_rank should be found");
                let b_rank = CARD_RANK
                    .get(&b.to_string())
                    .expect("b_rank should be found");
                return a_rank.cmp(b_rank);
            });
            let sorted_hand_value = String::from_iter(chars);
            let clone_value = sorted_hand_value.clone();
            let bid = split_line
                .get(1)
                .expect("expeced bid")
                .replace("\r", "")
                .parse::<i32>()
                .expect("failed to parse bid");
            Hand {
                value: hand_value,
                bid,
                score: score_hand(&clone_value),
            }
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| sort_hands(a, b));
    let total = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        let hand_score = hand.bid * ((index as i32) + 1);
        return acc + hand_score;
    });
    // score each hand
    // sort by score:value
    // multiply bid by rank
    // sum it up
    return total;
}

pub fn sort_hands(a: &Hand, b: &Hand) -> Ordering {
    if a.score != b.score {
        return a.score.cmp(&b.score);
    }

    // same score, walk the cards to find the winner
    let a_chars = a.value.chars().collect::<Vec<_>>();
    let b_chars = &b.value.chars().collect::<Vec<_>>();

    for (index, a_char) in a_chars.iter().enumerate() {
        let a_rank = CARD_RANK.get(&a_char.to_string()).expect("some balue");

        let b_char = b_chars.get(index).expect("should be a char here");
        let b_rank = CARD_RANK
            .get(&b_char.to_string())
            .expect("Some_other_value");
        if a_rank != b_rank {
            return a_rank.cmp(&b_rank);
        }
    }

    Ordering::Equal
}

pub struct Hand {
    pub value: String,
    pub bid: i32,
    pub score: i32,
}
impl Hand {}
/*
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
 */

mod tests {

    use super::*;

    #[test]
    fn sort_hands_test() {
        let a = Hand {
            value: "QQQJA".to_string(),
            bid: 1,
            score: 0,
        };
        let b = Hand {
            value: "Q2Q2Q".to_string(),
            bid: 1,
            score: 0,
        };
        let result = sort_hands(&a, &b);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn execute_day_7_a_test() {
        let result = execute_day_7_a("./input/day-7-test.txt");
        assert_eq!(result, 6592);
    }

    #[test]
    fn execute_day_7_a_test_2() {
        let result = execute_day_7_a("./input/day-7.txt");
        assert_ne!(result, 251973945);
    }
}
