use std::collections::HashMap;

use crate::file_helper::get_file_contents;

// https://adventofcode.com/2023/day/2
/* Determine which games would have been possible if the bag had been loaded with
only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs
of those games? */
// Game 30: 2 blue, 4 green; 7 green, 1 blue, 1 red; 1 blue, 8 green
pub fn execute_day_2() -> i32 {
    let lines = get_file_contents("./input/day-2.txt");
    const MAX_RED_COUNT: i32 = 12;
    const MAX_GREEN_COUNT: i32 = 13;
    const MAX_BLUE_COUNT: i32 = 14;
    let mut color_hash_map: HashMap<&str, i32> = HashMap::new();
    color_hash_map.insert("red", MAX_RED_COUNT);
    color_hash_map.insert("green", MAX_GREEN_COUNT);
    color_hash_map.insert("blue", MAX_BLUE_COUNT);

    let mut total = 0;

    for line in lines {
        let parts = line.split(":").collect::<Vec<_>>();
        let game_part = *parts.get(0).expect("expected game part");
        let game_number = game_part
            .split_whitespace()
            .collect::<Vec<_>>()
            .get(1)
            .expect("expected game number")
            .to_string();

        let games_part = *parts.get(1).expect("expected games");
        let games = games_part.split(";").collect::<Vec<_>>();
        let mut all_games_valid = true;
        for game in games {
            let colors = game.split(",").collect::<Vec<_>>();

            for color in colors {
                let color_set = color.trim().split_whitespace().collect::<Vec<_>>();
                let color_name = color_set.get(1).expect("expected color!");
                let color_count = color_set.get(0).expect("expected count for color!");
                let color_max_value = color_hash_map
                    .get(color_name)
                    .unwrap_or_else(|| panic!("color not found {}", color_name));
                if color_count.parse::<i32>().unwrap() > *color_max_value {
                    all_games_valid = false;
                    break;
                }
            }
            if !all_games_valid {
                break;
            }
        }

        if all_games_valid {
            let game_value = game_number.parse::<i32>().unwrap();
            total += game_value;
        }
    }

    return total;
}
//Game 1: 2 blue, 3 red; 3 green, 3 blue, 6 red; 4 blue, 6 red; 2 green, 2 blue, 9 red; 2 red, 4 blue
pub fn execute_day_2_b() -> i32 {
    let lines = get_file_contents("./input/day-2.txt");

    let mut total = 0;

    for line in lines {
        let mut color_hash_map: HashMap<String, i32> = HashMap::new();
        color_hash_map.insert("red".to_string(), 1);
        color_hash_map.insert("green".to_string(), 1);
        color_hash_map.insert("blue".to_string(), 1);
        let parts = line.split(":").collect::<Vec<_>>();
        let games_part = parts.get(1).expect("expected games");
        let games = games_part.split(";").collect::<Vec<_>>();
        for game in games {
            let colors = game.split(",").collect::<Vec<_>>();

            for color in colors {
                let color_set = color.trim().split_whitespace().collect::<Vec<_>>();
                let color_name = color_set.get(1).expect("expected color!");
                let color_count = color_set.get(0).expect("expected count for color!");
                let color_max_value = color_hash_map
                    .get(*color_name)
                    .unwrap_or_else(|| panic!("color not found {}", color_name));
                if color_count.parse::<i32>().unwrap() > *color_max_value {
                    color_hash_map
                        .insert(color_name.to_string(), color_count.parse::<i32>().unwrap());
                }
            }
        }
        // calculate power = multiple each color
        let red_value = color_hash_map["red"];
        let blue_value = color_hash_map["blue"];
        let green_value = color_hash_map["green"];
        let game_power = red_value * blue_value * green_value;
        total += game_power;
    }

    return total;
}
