use std::collections::vec_deque::Iter;
use std::collections::HashMap;
use std::ops::Range;

use crate::file_helper::get_file_contents;

pub fn execute_day_5_a(file_path: &str) -> usize {
    let lines = get_file_contents(file_path);
    let almanac = Almanac::new(lines);
    return almanac.get_lowest_location();
}

pub struct Almanac {
    pub seeds: Vec<usize>,
    pub type_maps: HashMap<String, MapType>,
}
impl Almanac {
    pub fn new(lines: Vec<String>) -> Self {
        let mut iter = lines.iter();
        // first line is always seeds
        // seeds: 79 14 55 13
        let seeds_split = iter
            .next()
            .expect("expected line 1!")
            .as_str()
            .split(":")
            .collect::<Vec<_>>();
        let seeds = seeds_split
            .get(1)
            .expect("Should be a seed list!")
            .trim()
            .split_whitespace()
            .flat_map(|x| x.parse::<usize>())
            .collect();
        let mut type_maps: HashMap<String, MapType> = HashMap::new();

        //let mut current_title = "Empty";
        let filtered_iter = iter.filter(|line| !line.is_empty());
        filtered_iter.for_each(|line| {
            let pieces = line.split(" ").map(|x| x.to_string()).collect::<Vec<_>>();
            let mut map_type = &mut MapType {
                name: "Empty".to_owned(),
                categories: vec![],
            };
            if pieces.len() == 2 {
                // 2 parts = title
                let current_title = pieces.get(0).expect("Map title expected");
                map_type = type_maps
                    .entry(current_title.to_string())
                    .or_insert(MapType {
                        name: current_title.to_string(),
                        categories: vec![],
                    });
            } else {
                let values = pieces
                    .iter()
                    .flat_map(|x| x.parse::<usize>())
                    .collect::<Vec<_>>();
                assert_eq!(
                    values.len(),
                    3,
                    "Expected 3 values for category got: {}",
                    values.len()
                );
                let destination_start = *values.get(0).expect("expected destination");
                let source_start = *values.get(1).expect("Expected source");
                let range_length = *values.get(2).expect("Expected range length");
                let category = Category {
                    source_category: Range {
                        start: destination_start,
                        end: destination_start + range_length,
                    },
                    destination_category: Range {
                        start: source_start,
                        end: source_start + range_length,
                    },
                };
                map_type.add_category(category);
            }
        });
        Almanac { seeds, type_maps }
    }

    pub fn get_lowest_location(&self) -> usize {
        return self.seeds.len();
    }
}

pub struct Category {
    pub source_category: Range<usize>,
    pub destination_category: Range<usize>,
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
}
