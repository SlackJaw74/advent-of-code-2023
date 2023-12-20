use std::collections::HashMap;

use crate::file_helper::get_file_contents;

pub struct Instructions {
    steps: Vec<char>,
    node_map: HashMap<String, (String, String)>
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
        let indexed_steps = &self.steps;
        let node_map = &self.node_map;
    
        let step_count = indexed_steps.len();
        let mut current_step = 0;
        let mut current_node_name = "AAA";
        let mut current_node =  node_map.get(current_node_name).expect("expected starting node");
        while !current_node_name.eq("ZZZ") {
            let step_index = current_step % step_count;
            let tuple_side = indexed_steps.get(step_index).expect("Step expected");
            if tuple_side.eq_ignore_ascii_case(&'L') {
                current_node_name = &current_node.0;
            } else {
                current_node_name = &current_node.1;
            }
            current_step += 1;
            current_node = node_map.get(current_node_name).expect("node not found!");
            if (current_step % 100000) == 1 {
                print!(".");
            }
        }
    
        return current_step;
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
    return instructions.traverse_path();    
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
        assert_eq!(result, 2);
    }
}
