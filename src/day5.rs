use std::ops::Range;

use crate::file_helper::get_file_contents;

pub fn execute_day_5_a(file_path: &str) -> i64 {
    let lines = get_file_contents(file_path);
    let almanac = Almanac::new(lines, false);
    return almanac.get_lowest_location();
}

pub fn execute_day_5_b(file_path: &str) -> i64 {
    let lines = get_file_contents(file_path);
    let almanac = Almanac::new(lines, true);
    return almanac.get_lowest_location();
}

pub fn get_seeds(seeds_split: Vec<&str>, use_seed_ranges: bool) -> Vec<i64> {
    if use_seed_ranges {
        return seeds_split
            .get(1)
            .expect("Should be a seed list!")
            .trim()
            .split_whitespace()
            .flat_map(|x| x.parse::<i64>())
            .collect::<Vec<i64>>()
            .chunks(2)
            .flat_map(|chunk| {
                let start_value = chunk[0];
                let length = chunk[1];
                return (start_value..start_value + length).collect::<Vec<i64>>();
            })
            .collect::<Vec<i64>>();
    }
    return seeds_split
        .get(1)
        .expect("Should be a seed list!")
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<i64>())
        .collect();
}

pub struct Almanac {
    pub seeds: Vec<i64>,
    pub type_maps: Vec<MapType>,
}
impl Almanac {
    pub fn new(lines: Vec<String>, use_seed_ranges: bool) -> Self {
        let mut iter = lines.iter();
        // first line is always seeds
        // seeds: 79 14 55 13
        let seeds_split = iter
            .next()
            .expect("expected line 1!")
            .as_str()
            .split(":")
            .collect::<Vec<_>>();

        let seeds = get_seeds(seeds_split, use_seed_ranges);

        return iter.fold(
            Self {
                seeds,
                type_maps: vec![],
            },
            |mut acc, line| {
                if line.is_empty() {
                    return acc;
                }
                let split_line = line.split(" ").collect::<Vec<_>>();
                if split_line.len() == 2 {
                    // map title
                    let title = split_line[0];
                    let map_type = MapType {
                        name: title.to_owned(),
                        categories: vec![],
                    };

                    acc.type_maps.append(&mut vec![map_type]);
                } else if split_line.len() == 3 {
                    // category
                    let iter_mut = acc.type_maps.iter_mut();
                    let map_type = iter_mut.last().expect("Should be an entry!");
                    let values = split_line
                        .iter()
                        .flat_map(|x| x.replace("\r", "").parse::<i64>())
                        .collect::<Vec<_>>();
                    let destination_start = *values.get(0).expect("expected destination");
                    let source_start = *values.get(1).expect("Expected source");
                    let range_length = *values.get(2).expect("Expected range length");
                    let category = Category {
                        destination_category: Range {
                            start: destination_start,
                            end: destination_start + range_length,
                        },
                        source_category: Range {
                            start: source_start,
                            end: source_start + range_length,
                        },
                    };
                    map_type.add_category(category)
                }

                return acc;
            },
        );
    }

    pub fn get_lowest_location(&self) -> i64 {
        // for each seed
        //
        let seed_locations = self.seeds.iter().map(|seed| {
            let mut current_index = *seed;
            for type_map in &self.type_maps {
                for category in &type_map.categories {
                    if category.source_category.contains(&current_index) {
                        let offset = current_index - category.source_category.start;
                        current_index = category.destination_category.start + offset;
                        break;
                    }
                }
            }

            return current_index.to_owned();
        });
        return *seed_locations
            .collect::<Vec<i64>>()
            .iter()
            .min()
            .expect("expected minimum");
    }
}

pub struct Category {
    pub source_category: Range<i64>,
    pub destination_category: Range<i64>,
}
pub struct MapType {
    pub name: String,
    pub categories: Vec<Category>,
}
impl MapType {
    pub fn add_category(&mut self, category: Category) {
        self.categories.push(category);
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn execute_day_5_a_test() {
        let result = execute_day_5_a("./input/day-5-test.txt");
        assert_eq!(result, 35);
    }

    #[test]
    fn execute_day_5_b_test() {
        let result = execute_day_5_b("./input/day-5-test.txt");
        assert_eq!(result, 46);
    }
}
