use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn solve_part_1() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day10/input.txt").unwrap();

    let mut total_history = 0;

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total history is {}, found in {:?}", total_history, end - start);
}

pub fn solve_part_2() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day10/input.txt").unwrap();

    let mut total_history = 0;

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total history is {}, found in {:?}", total_history, end - start);
}
