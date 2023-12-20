use lcmx::lcmx;
use std::{collections::HashMap, num};

use crate::file_helper::get_file_contents;

pub struct Instructions {
    steps: Vec<char>,
    node_map: HashMap<String, (String, String)>,
}
impl Instructions {
    pub fn new(lines: Vec<String>) -> Self {
        let mut line_iter = lines.iter();
        let steps = line_iter
            .next()
            .expect("Missing steps line")
            .replace("\r", "")
            .chars()
            .collect::<Vec<_>>();
        line_iter.next().expect("expected empty line");
        let mut node_map: HashMap<String, (String, String)> = HashMap::new();
        println!("building instructions map");
        line_iter.for_each(|line| {
            let split_line = line.split(" = ").collect::<Vec<_>>();
            let node_name = split_line.get(0).expect("node name expected").to_string();
            let split_tuple = split_line
                .get(1)
                .expect("left expected")
                .split(", ")
                .collect::<Vec<_>>();
            let left = split_tuple
                .get(0)
                .expect("left expected")
                .to_string()
                .replace("(", "");
            let right = split_tuple
                .get(1)
                .expect("right expected")
                .to_string()
                .replace(")", "")
                .replace("\r", "");
            node_map.insert(node_name, (left, right));
        });
        Self { steps, node_map }
    }
    pub fn traverse_path(&self) -> usize {
        println!("traversing path");

        return self.find_steps_to_z("AAA");
    }
    pub fn find_steps_to_z(&self, start_node_name: &str) -> usize {
        let indexed_steps = &self.steps;
        let node_map = &self.node_map;
        let step_count = indexed_steps.len();
        let mut current_step = 0;
        let mut current_node_name = start_node_name;
        let mut current_node = node_map
            .get(current_node_name)
            .expect("expected starting node");
        while !current_node_name.ends_with("Z") {
            let step_index = current_step % step_count;
            let tuple_side = indexed_steps.get(step_index).expect("Step expected");
            if tuple_side.eq_ignore_ascii_case(&'L') {
                current_node_name = &current_node.0;
            } else {
                current_node_name = &current_node.1;
            }
            current_step += 1;
            current_node = node_map.get(current_node_name).expect("node not found!");
        }

        return current_step;
    }
    pub fn traverse_paths(&self) -> usize {
        println!("traversing paths");
        let node_map = &self.node_map;
        let steps_per_path = node_map
            .keys()
            .filter(|key| key.ends_with("A"))
            .map(|node_name| self.find_steps_to_z(node_name))
            .collect::<Vec<_>>();
        return lcmx(&steps_per_path).unwrap();
    }
}

pub fn execute_day_8_a(file_path: &str) -> usize {
    let lines = get_file_contents(file_path);
    let instructions = Instructions::new(lines);
    return instructions.traverse_path();
}

pub fn execute_day_8_b(file_path: &str) -> usize {
    let lines = get_file_contents(file_path);
    let instructions = Instructions::new(lines);
    return instructions.traverse_paths();
}

mod tests {

    use super::*;

    #[test]
    fn execute_day_8_a_full_test() {
        let result = execute_day_8_a("./input/day-8.txt");
        assert_eq!(result, 16897);
    }

    #[test]
    fn execute_day_8_b_test() {
        let result = execute_day_8_b("./input/day-8-b-test.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn execute_day_8_b_full_test() {
        let result = execute_day_8_b("./input/day-8-b.txt");
        assert_eq!(result, 16563603485021);
    }
}
