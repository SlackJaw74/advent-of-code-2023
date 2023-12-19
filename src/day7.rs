use crate::file_helper::get_file_contents;
use phf::phf_map;
use std::{cmp::Ordering, collections::HashMap};

const CARD_RANK: phf::Map<&'static str, &'static i32> = phf_map! {
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

const CARD_RANK_WITH_JOKER: phf::Map<&'static str, &'static i32> = phf_map! {
    "A" => &13,
    "K" => &12,
    "Q" => &11,
    "T" => &10,
    "9" => &9,
    "8" => &8,
    "7" => &7,
    "6" => &6,
    "5" => &5,
    "4" => &4,
    "3" => &3,
    "2" => &2,
    "J" => &1
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

pub fn execute_day_7_a(file_path: &str) -> i32 {
    let lines = get_file_contents(file_path);
    let mut hands = get_hands(lines, CARD_RANK, &score_hand);

    hands.sort_by(|a, b| sort_hands(a, b));
    let total = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        let hand_score = hand.bid * ((index as i32) + 1);
        return acc + hand_score;
    });
    return total;
}

pub fn execute_day_7_b(file_path: &str) -> i32 {
    let lines = get_file_contents(file_path);
    let mut hands = get_hands(lines, CARD_RANK_WITH_JOKER, &score_hand_with_joker);

    hands.sort_by(|a, b| sort_hands_with_joker(a, b));
    let total = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        let hand_score = hand.bid * ((index as i32) + 1);
        return acc + hand_score;
    });
    return total;
}

pub fn get_hands(
    lines: Vec<String>,
    card_rank: phf::Map<&'static str, &'static i32>,
    score_fn: &dyn Fn(&str) -> i32,
) -> Vec<Hand> {
    return lines
        .iter()
        .map(|line| {
            let split_line = line.split(" ").collect::<Vec<_>>();
            let hand_value = split_line.get(0).expect("expected card value").to_string();
            let mut chars = hand_value.chars().collect::<Vec<_>>();
            chars.sort_by(|a, b| {
                let a_rank = card_rank
                    .get(&a.to_string())
                    .expect("a_rank should be found");
                let b_rank = card_rank
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
                score: score_fn(&clone_value),
            }
        })
        .collect::<Vec<_>>();
}

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
    let full_house = char_map.iter().find(|x| x.1.eq(&2)).is_some()
        && char_map.iter().find(|x| x.1.eq(&3)).is_some();
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

pub fn score_hand_with_joker(sorted_hand_value: &str) -> i32 {
    sorted_hand_value
        .chars()
        .collect::<Vec<_>>()
        .sort_by(|a, b| {
            let a_value = CARD_RANK_WITH_JOKER.get(&a.to_string());
            let b_value = CARD_RANK_WITH_JOKER.get(&b.to_string());
            return b_value.cmp(&a_value);
        });
    // run regex on the sorted hand value
    let char_count_map = sorted_hand_value
        .chars()
        .into_iter()
        .fold(vec![], |mut acc, char| {
            println!("char {}", char);
            if !acc.contains(&(char, 0)) {
                acc.push((char, 0));
            }
            let char_item = acc
                .iter_mut()
                .find(|x| x.0.eq(&char))
                .expect("Char should be found!");
            char_item.1 += 1;
            return acc;
        });
    //let char_map = char_count_map.iter_mut().map(|entry| entry).collect::<Vec<_>>();
    let joker_chars = char_count_map.iter().find(|x| x.0.eq(&'J'));
    let mut joker_distribution_count = 0;
    match joker_chars {
        Some((_, count)) => joker_distribution_count = *count,
        None => {}
    };
    if joker_distribution_count == 5 {
        return *HAND_RANK.get("FIVE_OF_A_KIND").expect("hand_rank value");
    }
    let updated_char_map = char_count_map
        .iter()
        .map(|(char, count)| {
            println!("char {}", char);
            println!("current count {}", count);
            if char.eq_ignore_ascii_case(&'J') || joker_distribution_count.eq(&0) {
                return (char, *count);
            }
            let mut updated_count = *count;
            while joker_distribution_count > 0 && updated_count < 5 {
                updated_count += 1;
                joker_distribution_count -= 1;
            }
            return (char, updated_count);
        })
        .collect::<Vec<_>>();

    println!("char_map {:?}", char_count_map);
    println!("updated_char_map {:?}", updated_char_map);
    let five_of_a_kind = updated_char_map.iter().find(|x| (x.1).eq(&5)).is_some();
    if five_of_a_kind {
        return *HAND_RANK.get("FIVE_OF_A_KIND").expect("hand_rank value");
    }
    let four_of_a_kind = updated_char_map.iter().find(|x| (x.1).eq(&4)).is_some();
    if four_of_a_kind {
        return *HAND_RANK.get("FOUR_OF_A_KIND").expect("hand_rank value");
    }
    let full_house = updated_char_map.iter().find(|x| (x.1).eq(&2)).is_some()
        && updated_char_map.iter().find(|x| (x.1).eq(&3)).is_some();
    if full_house {
        return *HAND_RANK.get("FULL_HOUSE").expect("hand_rank value");
    }
    let three_of_a_kind = !full_house && updated_char_map.iter().find(|x| (x.1).eq(&3)).is_some();
    if three_of_a_kind {
        return *HAND_RANK.get("THREE_OF_A_KIND").expect("hand_rank value");
    }
    let pairs = updated_char_map
        .iter()
        .filter(|x| (x.1).eq(&2) && (x.1).eq(&2))
        .collect::<Vec<_>>();
    if pairs.len() == 2 {
        return *HAND_RANK.get("TWO_PAIR").expect("hand_rank value");
    }
    if pairs.len() == 1 {
        return *HAND_RANK.get("ONE_PAIR").expect("hand_rank value");
    }
    return *HAND_RANK.get("HIGH_CARD").expect("hand rank value");
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

pub fn sort_hands_with_joker(a: &Hand, b: &Hand) -> Ordering {
    if a.score != b.score {
        return a.score.cmp(&b.score);
    }

    // same score, walk the cards to find the winner
    let a_chars = a.value.chars().collect::<Vec<_>>();
    let b_chars = b.value.chars().collect::<Vec<_>>();

    for (index, a_char) in a_chars.iter().enumerate() {
        let a_rank = CARD_RANK_WITH_JOKER
            .get(&a_char.to_string())
            .expect("some_value");

        let b_char = b_chars.get(index).expect("should be a char here");
        let b_rank = CARD_RANK_WITH_JOKER
            .get(&b_char.to_string())
            .expect("Some_other_value");
        println!("a: {} {}", a_char, a_rank);
        println!("b: {} {} ", b_char, b_rank);
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

mod tests {

    use super::*;

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

    #[test]
    fn execute_day_7_b_test() {
        let result = execute_day_7_b("./input/day-7-test.txt");
        assert_eq!(result, 6839);
    }

    #[test]
    fn score_hand_with_joker_test() {
        let full_house_result = score_hand_with_joker("2233J");
        assert_eq!(
            full_house_result,
            *HAND_RANK.get("FULL_HOUSE").expect("full house value")
        );

        let four_of_a_kind_result = score_hand_with_joker("555TJ");
        assert_eq!(
            four_of_a_kind_result,
            *HAND_RANK.get("FOUR_OF_A_KIND").expect("four of a kind")
        );
    }
    
    #[test]
    fn sort_hands_with_joker_test() {
        let a = Hand {
            value: "Q2Q2Q".to_string(),
            bid: 19,
            score: 0
        };
        let b = Hand {
            value: "T3T3J".to_string(),
            bid: 23,
            score: 0
        };

        let result = sort_hands_with_joker(&a, &b);
        assert_eq!(result, Ordering::Less);
    }
}
