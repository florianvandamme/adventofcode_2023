use std::collections::HashMap;
use std::fs;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct PartNumber {
    line: usize,
    position: (usize, usize),
    value: i32,
}

#[derive(Debug)]
pub struct SymbolPosition {
    line: usize,
    position: usize,
    symbol: char,
}

pub fn solve_part_1() {
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    let parts = input.lines();

    let mut values: HashMap<usize, Vec<PartNumber>> = HashMap::new();
    let mut symbols: Vec<SymbolPosition> = vec![];

    let mut total_sum = 0;

    for (index, part) in parts.enumerate() {
        let (parsed_symbols, mut parsed_values) = parse_parts(index, part);
        for parsed_symbol in parsed_symbols {
            symbols.push(parsed_symbol);
        }

        values.insert(index, parsed_values);
    }

    // Find which values to count
    for symbol in symbols {

        // index - 1 || index || index + 1
        let collision: Vec<usize> = vec![symbol.position - 1, symbol.position, symbol.position + 1];

        // Previous Line
        for part in values.get(&(symbol.line - 1)).unwrap() {
            if is_tuple_colling(part.position, &collision) {
                println!("{:?} is colliding with {:?}", symbol, part);
                total_sum += part.value;
            }
        }

        // Same Line
        for part in values.get(&symbol.line).unwrap() {
            if is_tuple_colling(part.position, &collision) {
                println!("{:?} is colliding with {:?}", symbol, part);
                total_sum += part.value;
            }
        }

        // Next Line
        for part in values.get(&(symbol.line + 1)).unwrap() {
            if is_tuple_colling(part.position, &collision) {
                println!("{:?} is colliding with {:?}", symbol, part);
                total_sum += part.value;
            }
        }
    }

    println!("The sum of all the engine parts is: {}", total_sum);
}

pub fn solve_part_2() {
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    let parts = input.lines();

    let mut values: HashMap<usize, Vec<PartNumber>> = HashMap::new();
    let mut symbols: Vec<SymbolPosition> = vec![];

    let mut total_sum = 0;

    for (index, part) in parts.enumerate() {
        let (parsed_symbols, mut parsed_values) = parse_parts(index, part);
        for parsed_symbol in parsed_symbols {
            symbols.push(parsed_symbol);
        }

        values.insert(index, parsed_values);
    }

    // Find which values to count
    for symbol in symbols {
        if (symbol.symbol == '*') {
            let mut adjacent_numbers: Vec<&PartNumber> = vec![];
            let collision: Vec<usize> = vec![symbol.position - 1, symbol.position, symbol.position + 1];

            // Previous Line
            for part in values.get(&(symbol.line - 1)).unwrap() {
                if is_tuple_colling(part.position, &collision) {
                    println!("{:?} is colliding with {:?}", symbol, part);
                    adjacent_numbers.push(part);
                }
            }

            // Same Line
            for part in values.get(&symbol.line).unwrap() {
                if is_tuple_colling(part.position, &collision) {
                    println!("{:?} is colliding with {:?}", symbol, part);
                    adjacent_numbers.push(part);
                }
            }

            // Next Line
            for part in values.get(&(symbol.line + 1)).unwrap() {
                if is_tuple_colling(part.position, &collision) {
                    println!("{:?} is colliding with {:?}", symbol, part);
                    adjacent_numbers.push(part);
                }
            }

            if adjacent_numbers.len() == 2 {
                total_sum += adjacent_numbers[0].value * adjacent_numbers[1].value;
            }
        }
    }

    println!("The sum of all the engine parts is: {}", total_sum);
}

pub fn parse_parts(line: usize, part: &str) -> (Vec<SymbolPosition>, Vec<PartNumber>) {
    let mut value_vec: Vec<PartNumber> = vec![];
    let mut symbol_vec: Vec<SymbolPosition> = vec![];

    let mut string_number = String::new();
    let max_len = part.len();

    for (index, char) in part.chars().enumerate() {
        match char {
            '.' => {
                if string_number.len() > 0 {
                    value_vec.push(PartNumber {
                        line,
                        value: string_number.parse::<i32>().unwrap(),
                        position: (index - string_number.len(), index - 1),
                    });
                    string_number = String::new();
                }
            }
            c if c.is_digit(10) => {
                string_number.push(char);

                if index == max_len - 1 {
                    value_vec.push(PartNumber {
                        line,
                        value: string_number.parse::<i32>().unwrap(),
                        position: (index - string_number.len(), index - 1),
                    });
                    string_number = String::new();
                }
            }
            c if !c.is_digit(10) => {
                symbol_vec.push(SymbolPosition {
                    line,
                    position: index,
                    symbol: c,
                });

                if string_number.len() > 0 {
                    value_vec.push(PartNumber {
                        line,
                        value: string_number.parse::<i32>().unwrap(),
                        position: (index - string_number.len(), index - 1),
                    });
                    string_number = String::new();
                }
            }
            _ => panic!("YO WTF")
        }
    }

    (symbol_vec, value_vec)
}

pub fn is_tuple_colling(tuple: (usize, usize), collision: &[usize]) -> bool {
    collision.contains(&tuple.0) || collision.contains(&tuple.1)
}

