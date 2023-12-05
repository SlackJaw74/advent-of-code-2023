use std::env;
use std::env::current_dir;
use std::fs;

pub fn execute_day_1() -> i32 {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let root = current_dir().expect("failed to get current dir");
    let full_path = root.join(file_path);

    println!("In file {}", file_path);
    println!("In file {}", full_path.display());

    let contents = fs::read_to_string(full_path)
        .expect("Should have read the file, dog!");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut total = 0;
    for line in lines {
        total = total + get_number(line);
    }
    return total;
}


pub fn execute_day_1_b() -> i32 {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let root = current_dir().expect("failed to get current dir");
    let full_path = root.join(file_path);

    println!("In file {}", file_path);
    println!("In file {}", full_path.display());

    let contents = fs::read_to_string(full_path)
        .expect("Should have read the file, dog!");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut total = 0;
    for line in lines {
        total = total + get_number_2(line);
    }
    return total;
}

fn get_number_2(value: &str) -> i32 {

    let english_numbers: [[&str; 2]; 9] = [["one", "1"], ["two", "2"], ["three", "3"], ["four", "4"], ["five", "5"], ["six", "6"], ["seven", "7"], ["eight", "8"], ["nine", "9"]];

    let mut first_number_set: Vec<&str> = vec!["0", "0"];
    let mut number_found = false;
    // let mut found_number = ["0", "0"];
    let mut first_number_index: usize = 1000;
    let mut second_number_index: usize = 0;

    for number in english_numbers {
        let english_number = number[0];
        let char_number = number[1];

        // let mut english_number_index: i32;
        let english_matches: Vec<_> = value.match_indices(english_number).map(|(i, _)|i).collect();
        let char_matches: Vec<_> = value.match_indices(char_number).map(|(i, _)|i).collect();
        
       if english_matches.len() > 0 {
         let english_match_first_index = english_matches.get(0).unwrap();
         let english_match_second_index = english_matches.get(english_matches.len() - 1).unwrap();
         
         if english_match_first_index.lt(&first_number_index) {
            first_number_index = *english_match_first_index;
            first_number_set[0] = char_number;
            if !number_found {
                number_found = true;
                first_number_set[1] = char_number;
            }
         }

         if english_match_second_index.gt(&second_number_index) {
            second_number_index = *english_match_second_index;
            first_number_set[1] = char_number;
         }
       } 

       if char_matches.len() > 0 {
        let char_match_first_index = char_matches.get(0).unwrap();
        let char_match_second_index = char_matches.get(char_matches.len() - 1).unwrap();
        if char_match_first_index.lt(&first_number_index) {
            first_number_index = *char_match_first_index;
            first_number_set[0] = char_number;
            if !number_found {
                number_found = true;
                first_number_set[1] = char_number;
            }
        }

        if char_match_second_index.gt(&second_number_index) {
            second_number_index = *char_match_second_index;
            first_number_set[1] = char_number;
         }
       }
       
        println!("english_matches{:?}", english_matches);
        println!("char_matches{:?}", char_matches);
    }

    let number_string = first_number_set.iter().cloned().collect::<String>();
    return number_string.parse::<i32>().unwrap();
}

fn get_number(value: &str) -> i32 {

    let mut first_number_set = vec![];
    let mut first_number_found = false;
    for character in value.chars() {
        if character.is_ascii_digit() {
            if !first_number_found {
                first_number_found = true;
                // push first digit twice in case there isn't a second digit
                first_number_set.push(character);
                first_number_set.push(character);
                continue;
            }
            if first_number_set.len().eq(&2) {
                first_number_set.remove(1);
            }
            first_number_set.push(character);
        }
    }

    let number_string = first_number_set.iter().cloned().collect::<String>();
    return number_string.parse::<i32>().unwrap();
}
