use std::collections::{HashMap, HashSet};
use std::{fs, vec};
use std::hash::Hash;
use std::str::Lines;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn solve_part_1() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day8/input.txt").unwrap();

    let mut parts = input.split("\n\n");
    let instructions = parts.next().unwrap();

    let node_map = nodes_to_hashmap(parts.next().unwrap().lines());

    let mut total_steps = 0;
    let mut current_node: String = "AAA".to_string();

    while current_node != "ZZZ" {
        for char in instructions.chars() {
            println!("{}", char);
            let value = node_map.get(&current_node).unwrap();
            total_steps += 1;

            match char {
                'L' => current_node = value.clone().0,
                'R' => current_node = value.clone().1,
                _ => panic!("This should never happen"),
            };

            if current_node == "ZZZ" {
                break;
            }
        }
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total steps required to reach ZZZ is {}, found in {:?}", total_steps, end - start);
}

pub fn solve_part_2() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day8/input.txt").unwrap();

    let mut parts = input.split("\n\n");
    let instructions = parts.next().unwrap();

    let node_map = nodes_to_hashmap(parts.next().unwrap().lines());

    let mut nodes = find_starting_nodes(&node_map);
    let mut output_steps: Vec<u128> = vec![];

    for node in nodes {
        let mut current_node = node.clone();
        let mut steps: u128 = 0;

        while !current_node.ends_with("Z") {
            for char in instructions.chars() {
                let value = node_map.get(&current_node).unwrap();
                steps += 1;

                match char {
                    'L' => current_node = value.clone().0,
                    'R' => current_node = value.clone().1,
                    _ => panic!("This should never happen"),
                };

                if current_node.ends_with("Z") {
                    output_steps.push(steps);
                    break;
                }
            }
        }
    }

    let mut total_steps: u128 = 0;
    let mut prime_factor = highest_prime_factor(&prime_factors(output_steps[0]));
    println!("{}", prime_factor);

    total_steps = prime_factor * output_steps.iter().map(|value| value / prime_factor).product::<u128>();

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total steps required to reach the XXZ locations is {}, found in {:?}", total_steps, end - start);
}

pub fn prime_factors(mut num: u128) -> Vec<u128> {
    let mut factors = vec![];
    let mut div = 2;

    while num > 1 {
        while num % div == 0 {
            factors.push(div);
            num /= div
        }

        div += 1
    }

    factors
}

pub fn highest_prime_factor(input: &Vec<u128>) -> u128 {
    let highest_number = input.iter().max().unwrap();
    let power = input.iter().filter(|num| num == &highest_number).count();

    highest_number.pow(power as u32)
}


pub fn nodes_to_hashmap(nodes: Lines) -> HashMap<String, (String, String)> {
    let mut output: HashMap<String, (String, String)> = HashMap::new();

    for node in nodes {
        let mut split = node.split(" = ");
        let key = split.next().unwrap();

        let tuple_string = split.next().unwrap().replace("(", "").replace(")", "");
        let mut split_values = tuple_string.split(", ");

        output.insert(key.to_owned(), (split_values.next().unwrap().to_owned(), split_values.next().unwrap().to_owned()));
    }

    output
}

pub fn find_starting_nodes(map: &HashMap<String, (String, String)>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for key in map.keys() {
        if key.ends_with("A") {
            output.push(key.to_owned());
        }
    }

    output
}
