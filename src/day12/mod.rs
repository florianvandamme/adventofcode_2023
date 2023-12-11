use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn solve_part_1() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day11/input.txt").unwrap();
    let total_distance = 0;

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total distance of the shortest paths is {}, found in {:?}", total_distance, end - start);
}

pub fn solve_part_2() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day11/briefing.txt").unwrap();
    let total_distance = 0;

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total distance of the shortest paths is {}, found in {:?}", total_distance, end - start);
}