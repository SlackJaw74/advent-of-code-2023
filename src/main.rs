mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod file_helper;

use crate::day1::execute_day_1;
use crate::day1::execute_day_1_b;
use crate::day2::execute_day_2;
use crate::day2::execute_day_2_b;
use crate::day3::execute_day_3;
use crate::day3::execute_day_3_b;
use crate::day4::execute_day_4_a;
use crate::day4::execute_day_4_b;
use crate::day5::execute_day_5_a;
//use crate::day5::execute_day_5_b;
use crate::day6::execute_day_6_a;
use crate::day6::execute_day_6_b;
use crate::day7::execute_day_7_a;
use crate::day7::execute_day_7_b;
use crate::day8::execute_day_8_a;
use crate::day8::execute_day_8_b;

fn main() {
    println!("Day 1a: {}", execute_day_1());
    println!("Day 1b: {}", execute_day_1_b());

    println!("Day 2a: {}", execute_day_2());
    println!("Day 2b: {}", execute_day_2_b());

    println!("Day 3a: {}", execute_day_3());
    println!("Day 3b: {}", execute_day_3_b("./input/day-3.txt"));
    
    println!("Day 4a: {}", execute_day_4_a("./input/day-4.txt"));
    println!("Day 4b: {}", execute_day_4_b("./input/day-4.txt"));

    println!("Day 5a: {}", execute_day_5_a("./input/day-5.txt"));
    //println!("Day 5b: {}", execute_day_5_b("./input/day-5.txt"));

    println!("Day 6a: {}", execute_day_6_a());
    println!("Day 6b: {}", execute_day_6_b());

    println!("Day 7a: {}", execute_day_7_a("./input/day-7.txt"));
    println!("Day 7b: {}", execute_day_7_b("./input/day-7.txt"));

    println!("Day 8a: {}", execute_day_8_a("./input/day-8.txt"));
    println!("Day 8b: {}", execute_day_8_b("./input/day-8-b.txt"));
}
