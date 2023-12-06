use std::collections::HashMap;
use std::{fs, thread};
use std::hash::Hash;
use std::str::{Split, SplitWhitespace};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct SeedMap {
    range: (i128, i128),
    input_modifier: i128,
    output_modifier: i128,
}

#[derive(Debug)]
pub struct SeedMapMap {
    name: String,
    maps: Vec<SeedMap>,
}

// pub fn solve_part_1() {
//     let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
//     let input = fs::read_to_string("src/day5/input.txt").unwrap();
//     let mut maps = input.split("\n\n");
//
//     let seed_line = maps.next().unwrap();
//
//     let mut seed_iterator = seed_line.split_whitespace();
//     let mut seeds: Vec<i128> = parse_seeds(&mut seed_iterator);
//     let seed_maps: Vec<SeedMapMap> = parse_maps(maps);
//
//     let mut seed_mapping: HashMap<i128, i128> = HashMap::new();
//
//     for seed in seeds {
//         seed_mapping.insert(seed, seed);
//
//         for maps in seed_maps.iter().clone() {
//             let input = seed_mapping.get(&seed).unwrap();
//             // println!("Checking mapping for {} in map {}", input, maps.name);
//
//             for map in &maps.maps {
//                 let input = seed_mapping.get(&seed).unwrap();
//
//                 if is_number_in_range(map.range, input) {
//                     let output = map_seed(input, &map);
//                     // println!("Input {} collided with range {:?}, new output is {}", input, map.range, output);
//                     seed_mapping.insert(seed, output);
//                     break;
//                 }
//             }
//
//             // println!("Mapped seed {} to {:?} in mapping {}", seed, seed_mapping, maps.name);
//         }
//     }
//
//     let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
//     println!("The lowest location number is {} found in {:?}", seed_mapping.values().min().unwrap(), end - start);
// }

pub fn solve_part_2() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day5/input.txt").unwrap();
    let mut maps = input.split("\n\n");

    let seed_line = maps.next().unwrap();

    let mut seed_iterator = seed_line.split_whitespace();
    let seed_ranges: Vec<(i128, i128)> = parse_seeds(&mut seed_iterator);

    let mut seed_maps: Vec<SeedMapMap> = parse_maps(maps);
    seed_maps.reverse();

    let lowest_range = (0, 100000000000);

    for location in lowest_range.0..lowest_range.0 + lowest_range.1 + 1 {
        let mut value: i128 = location;

        for maps in &seed_maps {
            for map in &maps.maps {
                if is_number_in_range(&map.range, &value) {
                    value = map_seed(&value, &map);
                    break;
                }
            }
        }

        for seed_range in seed_ranges.iter() {
            if is_number_in_range(seed_range, &value) {
                let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                panic!("The lowest location number is {} found in {:?}", location, end - start);
            }
        }
    }
}

pub fn parse_seeds(iter: &mut SplitWhitespace) -> Vec<(i128, i128)> {
    let mut result: Vec<(i128, i128)> = vec![];
    iter.next();

    let mut iter = iter.map(|x| x.parse::<i128>().unwrap());

    while let Some(elem1) = iter.next() {
        if let Some(elem2) = iter.next() {
            result.push((elem1, elem1 + elem2))
        } else {
            panic!("Iterator has an odd length");
        }
    }

    result
}

pub fn parse_maps(maps: Split<&str>) -> Vec<SeedMapMap> {
    let mut all_maps: Vec<SeedMapMap> = vec![];

    for map in maps {
        let mut iter = map.lines();
        let name = iter.next().unwrap();

        let mut vector_map: Vec<SeedMap> = vec![];

        for x in iter {
            let mut numbers = x.split_whitespace();
            let output_modifier: i128 = numbers.next().unwrap().parse::<i128>().unwrap();
            let input_modifier: i128 = numbers.next().unwrap().parse::<i128>().unwrap();
            let upper_bound: i128 = numbers.next().unwrap().parse::<i128>().unwrap();

            vector_map.push(SeedMap {
                range: (output_modifier, output_modifier + upper_bound - 1),
                input_modifier: output_modifier,
                output_modifier: input_modifier,
            });
        }

        all_maps.push({
            SeedMapMap {
                name: name.to_owned(),
                maps: vector_map,
            }
        });
    }

    all_maps
}

pub fn is_number_in_range(range: &(i128, i128), number: &i128) -> bool {
    number >= &range.0 && number <= &range.1
}

pub fn map_seed(input: &i128, map: &SeedMap) -> i128 {
    input - map.input_modifier + map.output_modifier
}