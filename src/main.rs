mod day1;
mod day2;
mod day3;
mod day4;
mod file_helper;

use crate::day1::execute_day_1;
use crate::day1::execute_day_1_b;
use crate::day2::execute_day_2;
use crate::day2::execute_day_2_b;
use crate::day3::execute_day_3;
use crate::day3::execute_day_3_b;
use crate::day4::execute_day_4_a;

fn main() {
    println!("Day 1a: {}", execute_day_1());
    println!("Day 1b: {}", execute_day_1_b());

    println!("Day 2a: {}", execute_day_2());
    println!("Day 2b: {}", execute_day_2_b());

    println!("Day 3a: {}", execute_day_3());
    println!("Day 3b: {}", execute_day_3_b("./input/day-3.txt"));
    
    println!("Day 4a: {}", execute_day_4_a("./input/day-4.txt"));
}
