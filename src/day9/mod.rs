use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn solve_part_1() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day9/input.txt").unwrap();

    let mut total_history = 0;

    for line in input.lines() {
        let values: Result<Vec<i64>, _> = line.split_whitespace().map(|value| value.parse::<i64>()).collect();
        let mut result = map_until_nil(values.unwrap());
        result.reverse();

        generate_forecast(&mut result);
        println!("{:?}", result);

        total_history += result.last().unwrap().last().unwrap();
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total history is {}, found in {:?}", total_history, end - start);
}

pub fn solve_part_2() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day9/input.txt").unwrap();

    let mut total_history = 0;

    for line in input.lines() {
        let values: Result<Vec<i64>, _> = line.split_whitespace().map(|value| value.parse::<i64>()).collect();
        let mut result = map_until_nil(values.unwrap());
        result.reverse();

        generate_history(&mut result);
        println!("{:?}", result);

        total_history += result.last().unwrap().first().unwrap();
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total history is {}, found in {:?}", total_history, end - start);
}

pub fn map_until_nil(input: Vec<i64>) -> Vec<Vec<i64>> {
    let mut extrapolation = input.clone();
    let mut output = vec![input];

    while !all_nil(&extrapolation) {
        extrapolation = extrapolate(&extrapolation);

        output.push(extrapolation.clone());
    }

    output
}

pub fn all_nil(input: &Vec<i64>) -> bool {
    input.iter().all(|&i| i == 0)
}

pub fn extrapolate(input: &Vec<i64>) -> Vec<i64> {
    input.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn generate_forecast(input: &mut Vec<Vec<i64>>) -> &mut Vec<Vec<i64>> {
    input[0].push(0);

    for i in 1..input.len() {
        let (left, right) = input.split_at_mut(i);
        right[0].push(right[0].last().unwrap() + left[i - 1].last().unwrap());
    }

    input
}

pub fn generate_history(input: &mut Vec<Vec<i64>>) -> &mut Vec<Vec<i64>> {
    input[0].push(0);

    for i in 1..input.len() {
        let (left, right) = input.split_at_mut(i);
        right[0].insert(0, right[0].first().unwrap() - left[i - 1].first().unwrap());
    }

    input
}
