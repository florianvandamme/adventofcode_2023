use std::fs;

pub fn solve_part_1() {
    let input = fs::read_to_string("src/day5/briefing.txt").unwrap();
    let cards = input.lines();

    let mut total_sum = 0;

    println!("The score of the cards is: {}", total_sum);
}

pub fn solve_part_2() {
    let input = fs::read_to_string("src/day5/briefing.txt").unwrap();
    let cards = input.lines();

    let mut total_sum = 0;

    println!("The sum of x is: {}", total_sum);
}
