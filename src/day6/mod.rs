use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Match {
    duration: i128,
    distance: i128,
    number_of_winning_possibilities: i128,
    possibilities: Vec<MatchPossibility>,
}

#[derive(Debug)]
pub struct MatchPossibility {
    holding_time: i128,
    distance_traveled: i128,
}

pub fn solve_part_1() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day6/input.txt").unwrap();
    let mut matches = parse_matches(input);

    let mut total_sum: i128 = 1;

    for mut m in matches {
        for i in 1..m.duration {
            let distance = calculate_distance(i, m.duration);
            m.possibilities.push(MatchPossibility {
                distance_traveled: distance,
                holding_time: i,
            });
        }

        for possibility in m.possibilities {
            if possibility.distance_traveled > m.distance {
                m.number_of_winning_possibilities += 1;
            }
        }

        total_sum *= m.number_of_winning_possibilities;
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The number of ways you can beat the record multiplied is equal to {} found in  {:?}", total_sum, end - start);
}

pub fn solve_part_2() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day6/input.txt").unwrap();

    let mut matches = parse_matches_kerning_fixed(input);
    println!("{:?}", matches);

    let mut total_sum: i128 = 1;

    for mut m in matches {
        for i in 1..m.duration {
            let distance = calculate_distance(i, m.duration);
            m.possibilities.push(MatchPossibility {
                distance_traveled: distance,
                holding_time: i,
            });
        }

        for possibility in m.possibilities {
            if possibility.distance_traveled > m.distance {
                m.number_of_winning_possibilities += 1;
            }
        }

        total_sum *= m.number_of_winning_possibilities;
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The number of ways you can beat the record multiplied is equal to {} found in  {:?}", total_sum, end - start);
}

fn calculate_distance(button_press: i128, duration: i128) -> i128 {
    (duration - button_press) * button_press
}

fn parse_matches(text: String) -> Vec<Match> {
    let mut matches: Vec<Match> = vec![];

    let lines = text.lines();
    let mut numbers: Vec<i128> = vec![];

    for line in lines {
        let mut values = line.split_whitespace();
        values.next();

        for value in values {
            numbers.push(value.parse::<i128>().unwrap());
        }
    }

    let size = numbers.len();
    for index in 0..size / 2 {
        matches.push(Match {
            duration: numbers[index] as i128,
            distance: numbers[(index + size / 2)],
            number_of_winning_possibilities: 0,
            possibilities: vec![],
        })
    }

    matches
}

fn parse_matches_kerning_fixed(text: String) -> Vec<Match> {
    let mut matches: Vec<Match> = vec![];

    let lines = text.lines();
    let mut numbers: Vec<i128> = vec![];

    for line in lines {
        let mut values = line.split_whitespace();
        values.next();
        let mut value = String::new();

        for b in values {
            value = format!("{}{}", value, b)
        }

        numbers.push(value.parse::<i128>().unwrap());
    }

    let size = numbers.len();
    for index in 0..size / 2 {
        matches.push(Match {
            duration: numbers[index] as i128,
            distance: numbers[(index + size / 2)] as i128,
            number_of_winning_possibilities: 0,
            possibilities: vec![],
        })
    }

    matches
}