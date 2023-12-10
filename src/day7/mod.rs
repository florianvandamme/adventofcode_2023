use std::char::from_u32;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::mem::swap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum WinCondition {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
pub struct Hand {
    bid: u64,
    cards: Vec<char>,
    win_condition: WinCondition,
}

#[derive(Debug)]
pub struct Game {
    high_card: Vec<Hand>,
    one_pair: Vec<Hand>,
    two_pair: Vec<Hand>,
    three_of_a_kind: Vec<Hand>,
    full_house: Vec<Hand>,
    four_of_a_kind: Vec<Hand>,
    five_of_a_kind: Vec<Hand>,
}

pub fn solve_part_1() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day7/input.txt").unwrap();
    let hands = input.lines();

    let mut game_state = Game {
        high_card: vec![],
        one_pair: vec![],
        two_pair: vec![],
        three_of_a_kind: vec![],
        full_house: vec![],
        four_of_a_kind: vec![],
        five_of_a_kind: vec![],
    };

    for hand in hands {
        let new_hand = parse_hand(hand);

        match new_hand.win_condition {
            WinCondition::HighCard => sort_hands(new_hand, &mut game_state.high_card),
            WinCondition::OnePair => sort_hands(new_hand, &mut game_state.one_pair),
            WinCondition::TwoPair => sort_hands(new_hand, &mut game_state.two_pair),
            WinCondition::ThreeOfAKind => sort_hands(new_hand, &mut game_state.three_of_a_kind),
            WinCondition::FullHouse => sort_hands(new_hand, &mut game_state.full_house),
            WinCondition::FourOfAKind => sort_hands(new_hand, &mut game_state.four_of_a_kind),
            WinCondition::FiveOfAKind => sort_hands(new_hand, &mut game_state.five_of_a_kind),
        };
    }

    println!("{:?}", game_state);
    let mut total_sum = 0;

    total_sum += calculate_win_condition_bids(0, &game_state.high_card);
    println!("High card score: {}", total_sum);

    let high_card_offset = game_state.high_card.len();
    total_sum += calculate_win_condition_bids(high_card_offset, &game_state.one_pair);
    println!("One pair score: {}", total_sum);

    let one_pair_offset = game_state.one_pair.len() + high_card_offset;
    total_sum += calculate_win_condition_bids(one_pair_offset, &game_state.two_pair);
    println!("Two pair score: {}", total_sum);

    let two_pair_offset = game_state.two_pair.len() + one_pair_offset;
    total_sum += calculate_win_condition_bids(two_pair_offset, &game_state.three_of_a_kind);
    println!("Three of a kind score: {}", total_sum);

    let three_of_a_kind_offset = game_state.three_of_a_kind.len() + two_pair_offset;
    total_sum += calculate_win_condition_bids(three_of_a_kind_offset, &game_state.full_house);
    println!("Full house score: {}", total_sum);

    let full_house_offset = game_state.full_house.len() + three_of_a_kind_offset;
    total_sum += calculate_win_condition_bids(full_house_offset, &game_state.four_of_a_kind);
    println!("Four of a kind score: {}", total_sum);

    let four_of_a_kind_offset = game_state.four_of_a_kind.len() + full_house_offset;
    total_sum += calculate_win_condition_bids(four_of_a_kind_offset, &game_state.five_of_a_kind);
    println!("Five of a kind score: {}", total_sum);

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total winnings for every hand are {}, found in {:?}", total_sum, end - start);
}

pub fn solve_part_2() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day7/input.txt").unwrap();

    let hands = input.lines();

    let mut game_state = Game {
        high_card: vec![],
        one_pair: vec![],
        two_pair: vec![],
        three_of_a_kind: vec![],
        full_house: vec![],
        four_of_a_kind: vec![],
        five_of_a_kind: vec![],
    };

    for hand in hands {
        let new_hand = parse_hand(hand);

        match new_hand.win_condition {
            WinCondition::HighCard => sort_hands(new_hand, &mut game_state.high_card),
            WinCondition::OnePair => sort_hands(new_hand, &mut game_state.one_pair),
            WinCondition::TwoPair => sort_hands(new_hand, &mut game_state.two_pair),
            WinCondition::ThreeOfAKind => sort_hands(new_hand, &mut game_state.three_of_a_kind),
            WinCondition::FullHouse => sort_hands(new_hand, &mut game_state.full_house),
            WinCondition::FourOfAKind => sort_hands(new_hand, &mut game_state.four_of_a_kind),
            WinCondition::FiveOfAKind => sort_hands(new_hand, &mut game_state.five_of_a_kind),
        };
    }

    println!("{:?}", game_state);
    let mut total_sum = 0;

    total_sum += calculate_win_condition_bids(0, &game_state.high_card);
    println!("High card score: {}", total_sum);

    let high_card_offset = game_state.high_card.len();
    total_sum += calculate_win_condition_bids(high_card_offset, &game_state.one_pair);
    println!("One pair score: {}", total_sum);

    let one_pair_offset = game_state.one_pair.len() + high_card_offset;
    total_sum += calculate_win_condition_bids(one_pair_offset, &game_state.two_pair);
    println!("Two pair score: {}", total_sum);

    let two_pair_offset = game_state.two_pair.len() + one_pair_offset;
    total_sum += calculate_win_condition_bids(two_pair_offset, &game_state.three_of_a_kind);
    println!("Three of a kind score: {}", total_sum);

    let three_of_a_kind_offset = game_state.three_of_a_kind.len() + two_pair_offset;
    total_sum += calculate_win_condition_bids(three_of_a_kind_offset, &game_state.full_house);
    println!("Full house score: {}", total_sum);

    let full_house_offset = game_state.full_house.len() + three_of_a_kind_offset;
    total_sum += calculate_win_condition_bids(full_house_offset, &game_state.four_of_a_kind);
    println!("Four of a kind score: {}", total_sum);

    let four_of_a_kind_offset = game_state.four_of_a_kind.len() + full_house_offset;
    total_sum += calculate_win_condition_bids(four_of_a_kind_offset, &game_state.five_of_a_kind);
    println!("Five of a kind score: {}", total_sum);

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total winnings for every hand are {}, found in {:?}", total_sum, end - start);
}

pub fn parse_hand(hand: &str) -> Hand {
    let mut split = hand.split_whitespace();

    let cards: Vec<char> = split.next().unwrap().chars().collect();
    let bid = split.next().unwrap().parse::<u64>().unwrap();

    Hand {
        bid,
        win_condition: calculate_win_condition_part_2(cards.clone()),
        cards,
    }
}

pub fn calculate_win_condition(cards: Vec<char>) -> WinCondition {
    let mut card_set: HashMap<char, i8> = HashMap::new();

    for card in cards {
        *card_set.entry(card).or_insert(0) += 1;
    }

    match card_set.len() {
        1 => WinCondition::FiveOfAKind,
        2 => {
            if *card_set.values().max().unwrap() == 4 {
                return WinCondition::FourOfAKind;
            }

            WinCondition::FullHouse
        }
        3 => {
            if *card_set.values().max().unwrap() == 3 {
                return WinCondition::ThreeOfAKind;
            }

            WinCondition::TwoPair
        }
        4 => WinCondition::OnePair,
        _ => WinCondition::HighCard,
    }
}

pub fn calculate_win_condition_part_2(cards: Vec<char>) -> WinCondition {
    let mut card_set: HashMap<char, i8> = HashMap::new();

    for card in cards.clone() {
        *card_set.entry(card).or_insert(0) += 1;
    }

    let joker_key = "J".parse::<char>().unwrap();
    let jokers = card_set.get(&joker_key).unwrap_or(&0).to_owned();
    card_set.remove(&joker_key);

    let mut max_value = None;
    let mut keys: Vec<char> = vec![];
    let card_set_clone = card_set.clone();
    for (key, value) in card_set_clone {
        match max_value {
            None => {
                max_value = Some(value);
                keys.push(key);
            }
            Some(v) if value > v => {
                max_value = Some(value);
                keys = vec![key];
            }
            Some(v) if value == v => {
                keys.push(key);
            }
            _ => {}
        }
    }

    if keys.len() == 0 {
        keys.push("J".parse::<char>().unwrap());
    }

    let highest_card_key = get_highest_card(keys);
    *card_set.entry(highest_card_key).or_insert(0) += jokers;

    match card_set.len() {
        1 => WinCondition::FiveOfAKind,
        2 => {
            if *card_set.values().max().unwrap() == 4 {
                return WinCondition::FourOfAKind;
            }

            WinCondition::FullHouse
        }
        3 => {
            if *card_set.values().max().unwrap() == 3 {
                return WinCondition::ThreeOfAKind;
            }

            WinCondition::TwoPair
        }
        4 => WinCondition::OnePair,
        _ => WinCondition::HighCard,
    }
}

pub fn sort_hands(new_hand: Hand, hands: &mut Vec<Hand>) -> &mut Vec<Hand> {
    let mut inserted = false;
    let new_hand_clone = new_hand.clone();
    for (index, hand) in hands.iter().enumerate() {
        if is_hand_smaller(&new_hand, hand) {
            hands.insert(index, new_hand);
            inserted = true;
            break;
        }
    }

    if !inserted {
        hands.push(new_hand_clone);
    }

    hands
}

pub fn is_hand_smaller(left_hand: &Hand, right_hand: &Hand) -> bool {
    for (index, card) in left_hand.cards.iter().enumerate() {
        if card.to_string() != right_hand.cards[index].to_string() {
            return is_card_smaller(&card.to_string(), &right_hand.cards[index].to_string());
        }
    }

    false
}

pub fn get_highest_card(cards: Vec<char>) -> char {
    let mut sorted_cards: Vec<char> = cards.clone();

    for (index, card) in cards.iter().enumerate() {
        let mut swapped = false;

        for j in 0..sorted_cards.len() - index - 1 {
            if is_card_smaller(&sorted_cards[j + 1].to_string(), &card.to_string()) {
                sorted_cards.swap(j + 1, j);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    sorted_cards[cards.len() - 1]
}

pub fn is_card_smaller(left: &str, right: &str) -> bool {
    match (left.parse::<u8>(), right.parse::<u8>()) {
        (Ok(left_val), Ok(right_val)) => left_val < right_val,
        (Ok(_), Err(_)) => {
            if right == "J" {
                return false;
            }

            return true;
        }
        (Err(_), Ok(_)) => {
            if left == "J" {
                return true;
            }

            return false;
        }
        _ => {
            match left.to_string().as_str() {
                "A" => false,
                "J" => true,
                "K" => right == "A",
                "Q" => right == "A" || right == "K",
                "T" => right != "J",
                _ => panic!("This should never happen"),
            }
        }
    }
}

pub fn calculate_win_condition_bids(offset: usize, hands: &Vec<Hand>) -> u64 {
    let mut total_bids = 0;

    for (index, hand) in hands.iter().enumerate() {
        total_bids += hand.bid * (index + offset + 1) as u64
    }

    total_bids
}