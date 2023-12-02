use std::collections::HashMap;
use std::fs;

pub fn solve_part_1() {
    let input = fs::read_to_string("src/day1/input.txt").unwrap();
    let instructions = input.lines();

    let mut total_sum = 0;

    for instruction in instructions {
        let mut value = String::new();

        if let Some(index) = first_number_index(instruction) {
            value.push(instruction.chars().nth(index).unwrap());
        }

        if let Some(index) = last_number_index(instruction) {
            value.push(instruction.chars().nth(index).unwrap());
        }

        total_sum += value.parse::<i32>().unwrap();
    }

    println!("The sum of all the calibration values is: {}", total_sum);
}

pub fn solve_part_2() {
    let input = fs::read_to_string("src/day1/input.txt").unwrap();
    let instructions = input.lines();

    let mut total_sum = 0;

    for instruction in instructions {
        let mut output: HashMap<usize, i32> = HashMap::new();

        if let Some(index) = first_number_index(instruction) {
            output.insert(index, parse_char_to_i32(instruction.chars().nth(index).unwrap()));
        }

        if let Some(index) = last_number_index(instruction) {
            output.insert(index, parse_char_to_i32(instruction.chars().nth(index).unwrap()));
        }

        let res = find_number_words(instruction, output);

        let min = res.keys().min();
        let max = res.keys().max();

        let result: String = format!("{}{}", res.get(min.unwrap()).unwrap(), res.get(max.unwrap()).unwrap());
        let result_as_number = result.parse::<i32>().unwrap();

        total_sum += result_as_number;
    }

    println!("The sum of all the calibration values is: {}", total_sum);
}

pub fn first_number_index(instruction: &str) -> Option<usize> {
    instruction.find(|c: char| c.is_digit(10))
}

pub fn last_number_index(instruction: &str) -> Option<usize> {
    instruction.rfind(|c: char| c.is_digit(10))
}

pub fn parse_char_to_i32(char: char) -> i32 {
    char.to_string().parse::<i32>().unwrap()
}

pub fn find_number_words(text: &str, mut output: HashMap<usize, i32>) -> HashMap<usize, i32> {
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for number in numbers.iter() {
        for (index, _) in text.match_indices(number) {
            output.insert(index, parse_number_word(number));
        }
    }

    output
}

pub fn parse_number_word(word: &str) -> i32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Something went wrong!"),
    }
}
