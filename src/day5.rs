use crate::file_helper::get_file_contents;

pub fn execute_day_5_a(file_path: &str) -> usize {
    let lines = get_file_contents(file_path);
    return lines.len();
}
