use std::collections::{HashSet};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn solve_part_1() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day11/input.txt").unwrap();

    expand_universe_to_file(input, 1usize);

    let input = fs::read_to_string("src/day11/intermediary_input.txt").unwrap();
    let galaxies = find_galaxies(input);
    let galaxy_pairs = find_galaxy_pairs(galaxies);

    let mut total_distance = 0;
    for pair in galaxy_pairs {
        total_distance += find_distance_between_galaxies(pair.0, pair.1);
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total distance of the shortest paths is {}, found in {:?}", total_distance, end - start);
}

pub fn solve_part_2() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day11/briefing.txt").unwrap();

    let galaxies = expand_universe_to_galaxy_coords(input, 2usize);
    let galaxy_pairs = find_galaxy_pairs(galaxies);

    let mut total_distance: u128 = 0;
    for pair in galaxy_pairs {
        total_distance += find_distance_between_galaxies(pair.0, pair.1);
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The total distance of the shortest paths is {}, found in {:?}", total_distance, end - start);
}

pub fn find_distance_between_galaxies(galaxy_a: (usize, usize), galaxy_b: (usize, usize)) -> u128 {
    let x_distance = if galaxy_a.0 > galaxy_b.0 {
        galaxy_a.0 - galaxy_b.0
    } else {
        galaxy_b.0 - galaxy_a.0
    };

    let y_distance = if galaxy_a.1 > galaxy_b.1 {
        galaxy_a.1 - galaxy_b.1
    } else {
        galaxy_b.1 - galaxy_a.1
    };

    (x_distance + y_distance) as u128
}

pub fn find_galaxies(input: String) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => galaxies.push((x, y)),
                _ => {}
            }
        }
    }

    galaxies
}

pub fn find_galaxy_pairs(input: Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut output = vec![];

    for (index, galaxy) in input.iter().enumerate() {
        for pair_index in index + 1..input.len() {
            output.push((*galaxy, input[pair_index]));
        }
    }

    output
}

pub fn expand_universe_to_galaxy_coords(input: String, factor: usize) -> Vec<(usize, usize)> {
    // Original, Projected
    let mut expanded_universe: Vec<Vec<((usize, usize), (usize, usize))>> = vec![];
    let line_length = input.lines().next().unwrap().len();
    let mut galaxy_x_positions: HashSet<usize> = HashSet::new();
    let mut y = 0;

    // Loop the lines
    for line in input.lines() {
        let mut found_galaxy_on_line = false;
        let mut new_line: Vec<((usize, usize), (usize, usize))> = vec![];

        for (index, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    found_galaxy_on_line = true;
                    galaxy_x_positions.insert(index);
                    new_line.push(((index, y), (index, y)));
                }
                _ => {}
            }
        }

        match found_galaxy_on_line {
            true => expanded_universe.push(new_line),
            false => y += factor - 1,
        }

        y += 1;
    }

    let mut col_indexes: Vec<usize> = (0..line_length).collect();
    col_indexes.retain(|&x| !galaxy_x_positions.contains(&x));
    col_indexes.sort();

    for row in expanded_universe.iter_mut() {
        for (i, col_i) in col_indexes.iter().enumerate() {
            for mut galaxy in &mut *row {
                if galaxy.0.0 > *col_i {
                    galaxy.1.0 = (factor - 1) + galaxy.1.0;
                }
            }
        }
    }

    let mut output: Vec<(usize, usize)> = vec![];
    for row in expanded_universe {
        for galaxy_set in row {
            output.push(galaxy_set.1)
        }
    }

    output
}

pub fn expand_universe_to_file(input: String, factor: usize) {
    let mut expanded_universe: Vec<Vec<char>> = vec![];
    let line_length = input.lines().next().unwrap().len();
    let mut galaxy_x_positions: HashSet<usize> = HashSet::new();

    // Loop the lines
    for line in input.lines() {
        let mut found_galaxy_on_line = false;
        let mut new_line: Vec<char> = vec![];

        for (index, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    found_galaxy_on_line = true;
                    galaxy_x_positions.insert(index);
                    new_line.push('#');
                }
                _ => {
                    new_line.push('.');
                }
            }
        }

        expanded_universe.push(new_line);

        if !found_galaxy_on_line {
            // Insert empty line
            expanded_universe.extend(vec![vec!['.'; line.chars().count()]; factor])
        }
    }

    let mut row_indexes: Vec<usize> = (0..line_length).collect();
    row_indexes.retain(|&x| !galaxy_x_positions.contains(&x));

    for row in expanded_universe.iter_mut() {
        for (i, index) in row_indexes.iter().enumerate() {
            let expansion_mansion = vec!['.'; factor];
            let index_to_insert_at = (i * factor) + *index;

            row.splice(index_to_insert_at..index_to_insert_at, expansion_mansion);
        }
    }

    let mut expanded_universe_string = String::new();
    for (i, row) in expanded_universe.iter().enumerate() {
        for c in row {
            expanded_universe_string.push(*c);
        }

        if i != expanded_universe.len() - 1 {
            expanded_universe_string.push('\n');
        }
    }

    let mut file = File::create("src/day11/intermediary_input.txt").expect("Could not create file");
    file.write_all(expanded_universe_string.as_bytes()).expect("Could not write to file");
}