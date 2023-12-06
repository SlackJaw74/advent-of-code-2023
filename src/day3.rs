use std::{ops::Range, collections::HashMap};

use crate::file_helper::get_file_contents;
use regex::Regex;

// https://adventofcode.com/2023/day/3
/*
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
*/
pub fn execute_day_3() -> i32 {
    let lines = get_file_contents("./input/day-3.txt");

    let mut previous_line: Line = create_line();
    let mut matched_part_numbers: Vec<i32> = Vec::new();
    for file_line in lines {
        println!("myLine {}", file_line);
        let current_line = parse_line(file_line);

        // find matches within the current line
        matched_part_numbers.append(&mut find_part_numbers(
            &current_line.part_numbers,
            &current_line.symbols,
        ));
        // find matches with current part numbers and previous lines symbols
        matched_part_numbers.append(&mut find_part_numbers(
            &current_line.part_numbers,
            &previous_line.symbols,
        ));
        // find matches with current symbols numbers and previous part numbers
        matched_part_numbers.append(&mut find_part_numbers(
            &previous_line.part_numbers,
            &current_line.symbols,
        ));

        previous_line = current_line;
    }

    return matched_part_numbers.iter().sum();
}


pub fn execute_day_3_b(file_path: &str) -> i32 {
    let lines = get_file_contents(file_path);
    let mut previous_line: Line = create_line();
    let mut matched_part_numbers: Vec<i32> = Vec::new();
    for file_line in lines {
        println!("myLine {}", file_line);
        let current_line = parse_line(file_line);

        // find matches within the current line
        matched_part_numbers.append(&mut find_part_numbers(
            &current_line.part_numbers,
            &current_line.symbols,
        ));
        // find matches with current part numbers and previous lines symbols
        matched_part_numbers.append(&mut find_part_numbers(
            &current_line.part_numbers,
            &previous_line.symbols,
        ));
        // find matches with current symbols numbers and previous part numbers
        matched_part_numbers.append(&mut find_part_numbers(
            &previous_line.part_numbers,
            &current_line.symbols,
        ));

        previous_line = current_line;
    }

    return matched_part_numbers.iter().sum();
}

struct Location {
    range: Range<usize>,
    value_as_int: i32,
    adjacent_locations: HashMap<i32, i32>
}

struct Line {
    part_numbers: Vec<Location>,
    symbols: Vec<Location>,
}

fn find_part_numbers(part_numbers: &Vec<Location>, symbols: &Vec<Location>) -> Vec<i32> {
    let mut matched_parts = Vec::new();
    for part in part_numbers {
        let part_start = part.range.start;
        let part_end = part.range.end;
        // for each symbol
        for symbol in symbols {
            let symbol_start = symbol.range.start;
            /*
            467..114..
            ...*...... */
            if symbol_start <= part_end && symbol_start + 1 >= part_start  {
                // match?
                let part_number = part.value_as_int;
                symbol.adjacent_locations.
                matched_parts.push(part_number);
            }
        }
    }

    return matched_parts;
}

fn create_line() -> Line {
    return Line {
        part_numbers: Vec::new(),
        symbols: Vec::new(),
    };
}

fn parse_line(line: String) -> Line {
    let number_regex = Regex::new(r"\d+").unwrap();
    let part_numbers = number_regex
        .find_iter(&line)
        .map(|x| {
            let value = x.as_str().parse::<i32>().unwrap();
            return Location {
                range: x.range(),
                value_as_int: value,
                adjacent_locations: Vec::new()
            };
        })
        .collect();
    let symbol_regex = Regex::new(r"[^0-9.\s]").unwrap();
    let symbols = symbol_regex
        .find_iter(&line)
        .map(|x| {
            return Location {
                range: x.range(),
                value_as_int: 0,
                adjacent_locations: Vec::new()
            };
        })
        .collect();
    return Line {
        part_numbers: part_numbers,
        symbols: symbols,
    };
}


#[cfg(test)]
mod tests {
    use super::execute_day_3_b;

    #[test]
    fn execute_day_3_b_mini() {

        let result = execute_day_3_b("./input/day-3-test.txt");
        assert_eq!(result, 467835);
    }
}