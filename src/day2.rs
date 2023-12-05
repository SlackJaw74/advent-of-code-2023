use std::env;
use std::env::current_dir;
use std::fs;

pub fn execute_day2() -> i32 {

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


fn get_number(value: &str) -> i32 {

    let mut first_number_set = vec![];
    let mut first_number_found = false;
    for character in value.chars() {
        if character.is_ascii_digit() {
            if (!first_number_found) {
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
