use std::ops::Range;

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
        let mut current_line = parse_line(&file_line);

        // find matches within the current line
        matched_part_numbers.append(&mut find_part_numbers(
            &current_line.part_numbers,
            &mut current_line.symbols,
        ));
        // find matches with current part numbers and previous lines symbols
        matched_part_numbers.append(&mut find_part_numbers(
            &current_line.part_numbers,
            &mut previous_line.symbols,
        ));
        // find matches with current symbols numbers and previous part numbers
        matched_part_numbers.append(&mut find_part_numbers(
            &previous_line.part_numbers,
            &mut current_line.symbols,
        ));

        previous_line = current_line;
    }

    return matched_part_numbers.iter().sum();
}

fn parse_line(line: &str) -> Line {
    let number_regex = Regex::new(r"\d+").unwrap();
    let part_numbers = number_regex
        .find_iter(&line)
        .map(|x| {
            let value = x.as_str().parse::<i32>().unwrap();
            Location {
                range: x.range(),
                value: x.as_str().to_string(),
                value_as_int: value,
                adjacent_locations: Vec::new(),
            }
        })
        .collect();
    let symbol_regex = Regex::new(r"[^0-9.\s]").unwrap();
    let symbols = symbol_regex
        .find_iter(&line)
        .map(|x| Location {
            range: x.range(),
            value: x.as_str().to_string(),
            value_as_int: 0,
            adjacent_locations: Vec::new(),
        })
        .collect();
    Line {
        part_numbers,
        symbols,
    }
}

fn find_part_numbers(part_numbers: &Vec<Location>, symbols: &mut Vec<Location>) -> Vec<i32> {
    let mut matched_parts = Vec::new();
    for part in part_numbers {
        let part_start = part.range.start;
        let part_end = part.range.end;
        // for each symbol
        for symbol_location in &mut *symbols {
            let symbol_start = symbol_location.range.start;
            /*
            467..114..
            ...*...... */
            if symbol_start <= part_end && symbol_start + 1 >= part_start {
                // match?
                let part_number = part.value_as_int;
                symbol_location.adjacent_locations.push(part_number);
                matched_parts.push(part_number);
            }
        }
    }

    return matched_parts;
}

pub fn execute_day_3_b(file_path: &str) -> i32 {
    let mut matrix = Matrix::new(file_path);
    return matrix.get_gear_ratio();
}

fn parse_line_2(line: &str, index: usize) -> Line2 {
    let number_regex = Regex::new(r"\d+").unwrap();

    let part_numbers = number_regex
        .find_iter(&line)
        .map(|x| {
            let value = x.as_str().parse::<i32>().unwrap();
            PartNumber {
                range: x.range(),
                row_number: index,
                value,
            }
        })
        .collect::<Vec<_>>();
    let symbol_regex = Regex::new(r"[^0-9.\s]").unwrap();
    let symbols = symbol_regex
        .find_iter(&line)
        .map(|x| Symbol {
            range: x.range(),
            row_number: index,
            value: x.as_str().to_string(),
            adjacent_part_numbers: Vec::new(),
        })
        .collect::<Vec<_>>();
    Line2 {
        part_numbers,
        symbols,
    }
}

#[derive(Default, Debug)]
pub struct Matrix {
    pub part_numbers: Vec<PartNumber>,
    pub symbols: Vec<Symbol>,
}
impl Matrix {
    pub fn new(file_path: &str) -> Self {
        let mut file_lines = get_file_contents(file_path);

        let lines = file_lines
            .iter_mut()
            .enumerate()
            .map(|(index, line)| parse_line_2(line, index))
            .collect::<Vec<_>>();
        let part_numbers = lines
            .iter()
            .cloned()
            .flat_map(|f| f.part_numbers)
            .collect::<Vec<_>>();
        let symbols = lines
            .iter()
            .cloned()
            .flat_map(|f| f.symbols)
            .collect::<Vec<_>>();
        Self {
            symbols,
            part_numbers,
        }
    }

    pub fn get_gear_ratio(&mut self) -> i32 {
        let part_numbers = &self.part_numbers;
        let symbols = &self.symbols;
        symbols.iter().map(|symbol| {
            let matching_part_numbers = part_numbers
                .iter()
                .filter(|part_number| {
                    let symbol_range = Range {
                        start: symbol.row_number - 1,
                        end: symbol.row_number + 1,
                    };
                    let row_in_range = part_number.row_number >= symbol_range.start
                        && part_number.row_number <= symbol_range.end;
                    return row_in_range;
                })
                .filter(|part_number| {
                    let is_adjacent = symbol.range.start <= part_number.range.end
                        && symbol.range.start + 1 >= part_number.range.start;
                    return is_adjacent;
                })
                .collect::<Vec<_>>();
            if matching_part_numbers.len() == 2 {
                let sum = matching_part_numbers
                    .iter()
                    .map(|part_number| part_number.value)
                    .fold(1, |acc, x| acc * x);
                return sum
            }
            return 0;
        }).sum()
    }
}

#[derive(Default, Debug, Clone)]
pub struct PartNumber {
    pub range: Range<usize>,
    pub row_number: usize,
    pub value: i32,
}

#[derive(Default, Debug, Clone)]
pub struct Symbol {
    pub range: Range<usize>,
    pub row_number: usize,
    pub value: String,
    pub adjacent_part_numbers: Vec<i32>,
}

#[derive(Default, Debug)]
pub struct Location {
    pub range: Range<usize>,
    pub value: String,
    pub value_as_int: i32,
    pub adjacent_locations: Vec<i32>,
}

#[derive(Default, Debug, Clone)]
pub struct Line2 {
    pub part_numbers: Vec<PartNumber>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug)]
pub struct Line {
    pub part_numbers: Vec<Location>,
    pub symbols: Vec<Location>,
}

fn create_line() -> Line {
    return Line {
        part_numbers: Vec::new(),
        symbols: Vec::new(),
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use insta::*;

    #[test]
    fn execute_day_3_b_mini() {
        let result = execute_day_3_b("./input/day-3-test.txt");
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_parse_line_2() {
        let result = parse_line_2("467..114..", 0);
        assert_debug_snapshot!(result, @r###"
        Line2 {
            part_numbers: [
                PartNumber {
                    range: 0..3,
                    row_number: 0,
                    value: 467,
                },
                PartNumber {
                    range: 5..8,
                    row_number: 0,
                    value: 114,
                },
            ],
            symbols: [],
        }
        "###);

        let result2 = parse_line_2("617*......", 0);
        assert_debug_snapshot!(result2, @r###"
        Line2 {
            part_numbers: [
                PartNumber {
                    range: 0..3,
                    row_number: 0,
                    value: 617,
                },
            ],
            symbols: [
                Symbol {
                    range: 3..4,
                    row_number: 0,
                    value: "*",
                    adjacent_part_numbers: [],
                },
            ],
        }
        "###);
    }
}
