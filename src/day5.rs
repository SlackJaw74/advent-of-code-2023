use std::ops::Range;

use crate::file_helper::get_file_contents;

pub fn execute_day_5_a(file_path: &str) -> usize {
    let lines = get_file_contents(file_path);
    let almanac = Almanac::new(lines);
    return almanac.get_lowest_location();
}

pub struct Almanac {
    pub seeds: Vec<usize>,
    pub type_maps: Vec<MapType>,
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
        
        //let mut current_title = "Empty";
        //let filtered_iter = iter.filter(|line| !line.is_empty());
        return iter.fold(
            Self {
                seeds: seeds,
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
                } else {
                    // category
                    let map_type = acc.type_maps.iter_mut().last().expect("Should be an entry!");
                    let values = split_line
                        .iter()
                        .flat_map(|x| x.parse::<usize>())
                        .collect::<Vec<_>>();
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
                    map_type.add_category(category)
                }

                return acc;
            },
        );
    }

    pub fn get_lowest_location(&self) -> usize {
        self.seeds.iter().fold(0, |acc, seed| {
            let source_category = self.type_maps
                .iter()
                .find(|x| x.name.eq("seed-to-soil"));

            let category_match = source_category.expect("expected map not found")
                .categories
                .iter()
                .find(|category| category.source_category.contains(seed))
                .a;
            category_match.

            return acc;
        });
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
