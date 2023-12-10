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
        println!("myLine {}", file_line);
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
    let mut file_lines = get_file_contents(file_path);

    let lines = file_lines
        .iter_mut()
        .map(|x| parse_line_2(x))
        .collect::<Vec<_>>();
    let iter = lines.windows(2);
    let mut total_gear_ratio = 0;

    iter.for_each(|lines| {
        let [previous_line, current_line] = lines else {
            todo!("uh oh")
        };
        // find matches within the current line
        find_adjacent_symbols(current_line.part_numbers, &mut current_line.symbols);
        // find matches with current part numbers and previous lines symbols
        //total_gear_ratio += find_adjacent_symbols(current_line.part_numbers, previous_line.symbols);
        // find matches with current symbols numbers and previous part numbers
        //total_gear_ratio += find_adjacent_symbols(previous_line.part_numbers, current_line.symbols);
    });

    return total_gear_ratio;
}

fn parse_line_2(line: &str) -> Line2 {
    let number_regex = Regex::new(r"\d+").unwrap();
    let part_numbers = number_regex
        .find_iter(&line)
        .map(|x| {
            let value = x.as_str().parse::<i32>().unwrap();
            PartNumber {
                range: x.range(),
                value,
            }
        })
        .collect::<Vec<_>>();
    let symbol_regex = Regex::new(r"[^0-9.\s]").unwrap();
    let symbols = symbol_regex
        .find_iter(&line)
        .map(|x| Symbol {
            range: x.range(),
            value: x.as_str().to_string(),
            adjacent_part_numbers: Vec::new(),
        })
        .collect::<Vec<_>>();
    Line2 {
        part_numbers,
        symbols,
    }
}

fn find_adjacent_symbols(part_numbers: Vec<PartNumber>, symbols: &mut Vec<Symbol>) {
    for part in part_numbers {
        let part_start: usize = part.range.start;
        let part_end = part.range.end;
        // for each symbol
        for symbol_location in symbols {
            let symbol_start = symbol_location.range.start;
            /*
            467..114..
            ...*.....
            */
            if symbol_start <= part_end && symbol_start + 1 >= part_start {
                // match?
                let part_number = part.value;
                symbol_location.adjacent_part_numbers.push(part_number);
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Matrix {
    pub part_numbers: Vec<PartNumber>,
    pub symbols: Vec<Symbol>,
    pub lines: Vec<Line2>,
}
impl Matrix {
    pub fn new(&mut self, file_path: &str) -> Self {
        let mut file_lines = get_file_contents(file_path);

        let lines = file_lines
            .iter_mut()
            .map(|x| parse_line_2(x))
            .collect::<Vec<_>>();
        let part_numbers = lines
            .iter()
            .flat_map(|f| &f.part_numbers)
            .collect::<Vec<_>>();
        let symbols = lines.iter().flat_map(|f| &f.symbols).collect::<Vec<_>>();
        Self {
            lines,
            symbols,
            part_numbers,
        }
    }
}

#[derive(Default, Debug)]
pub struct PartNumber {
    pub range: Range<usize>,
    pub value: i32,
}

#[derive(Default, Debug)]
pub struct Symbol {
    pub range: Range<usize>,
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
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Debug)]
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
        let result = parse_line_2("467..114..");
        assert_debug_snapshot!(result, @r###"
        Line2 {
            part_numbers: [
                PartNumber {
                    range: 0..3,
                    value: 467,
                },
                PartNumber {
                    range: 5..8,
                    value: 114,
                },
            ],
            symbols: [],
        }
        "###);

        let result2 = parse_line_2("617*......");
        assert_debug_snapshot!(result2, @r###"
        Line2 {
            part_numbers: [
                PartNumber {
                    range: 0..3,
                    value: 617,
                },
            ],
            symbols: [
                Symbol {
                    range: 3..4,
                    value: "*",
                    adjacent_part_numbers: [],
                },
            ],
        }
        "###);
    }

    // #[test]
    // fn test_process_gears() {
    //     let part_numbers = vec![Location::new()];
    //     let mut symbols = vec![Location::new()];
    //     let result = process_gears(part_numbers, symbols.as_mut());
    //     assert_eq!(result, 0);
    // }
}
