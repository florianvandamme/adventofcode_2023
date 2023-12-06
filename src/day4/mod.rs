use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
pub struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    card_numbers: HashSet<i32>,
}

pub fn solve_part_1() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();
    let cards = input.lines();

    let mut total_sum = 0;

    for card in cards {
        let parsed_card = parse_card(card);

        let score = calculate_card_score(&parsed_card);
        println!("Card {} score is {}", parsed_card.id, score);

        total_sum += score;
    }

    println!("The score of the cards is: {}", total_sum);
}

pub fn solve_part_2() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();
    let cards = input.lines();

    let mut card_map: HashMap<i32, i32> = HashMap::new();

    for card in cards {
        let parsed_card = parse_card(card);

        match card_map.get(&parsed_card.id) {
            Some(count) => card_map.insert(parsed_card.id, count + 1),
            None => card_map.insert(parsed_card.id, 1),
        };

        let matching_numbers = matching_numbers(&parsed_card);
        let card_copies = card_map.get(&parsed_card.id).unwrap().to_owned();

        for x in 1..matching_numbers + 1 {
            let increment = x as i32;
            let card_id = &parsed_card.id + increment;

            match card_map.get(&card_id) {
                Some(count) => card_map.insert(card_id, count + 1 * card_copies),
                None => card_map.insert(card_id, 1 * card_copies),
            };
        }
    }

    let total_sum: i32 = card_map.values().sum();

    println!("The sum of x is: {}", total_sum);
}

pub fn parse_card(card: &str) -> Card {
    let mut parts = card.split(": ");

    let id_string = parts.next().unwrap();

    let id_parts = id_string.split(" ");
    let id = id_parts.last().unwrap().parse::<i32>().unwrap();

    let mut winning_numbers_set: HashSet<i32> = HashSet::new();
    let mut card_numbers_set: HashSet<i32> = HashSet::new();

    let number_parts = parts.next().unwrap();
    let mut numbers = number_parts.split(" | ");

    let winning_numbers_section = numbers.next().unwrap();
    let winning_numbers = winning_numbers_section.split_whitespace();
    for winning_number in winning_numbers {
        winning_numbers_set.insert(winning_number.parse::<i32>().unwrap());
    }

    let card_numbers_section = numbers.next().unwrap();
    let card_numbers = card_numbers_section.split_whitespace();
    for card_number in card_numbers {
        card_numbers_set.insert(card_number.parse::<i32>().unwrap());
    }

    Card {
        id,
        card_numbers: card_numbers_set,
        winning_numbers: winning_numbers_set,
    }
}

pub fn calculate_card_score(card: &Card) -> usize {
    let count = matching_numbers(card);

    if count == 0 {
        return 0;
    }

    2usize.pow(count as u32 - 1)
}

pub fn matching_numbers(card: &Card) -> usize {
    let matching_numbers = card.card_numbers.intersection(&card.winning_numbers);
    let count = matching_numbers.count();

    count
}
