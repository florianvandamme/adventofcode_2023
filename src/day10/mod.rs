use std::cmp::max;
use std::collections::{HashMap};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::raw::time_t;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::day10::PipeType::{EastWest, Ground, NorthSouth, NorthEast, NorthWest, SouthWest, SouthEast, Start, Space};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum PipeType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
    Space,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Pipe {
    coordinates: (usize, usize),
    distance_from_start: u32,
    is_on_main_loop: bool,
    is_in_loop: bool,
    pipe_type: PipeType,
}

pub fn solve_part_1() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = fs::read_to_string("src/day10/briefing.txt").unwrap();

    let (map, start_pos, _) = parse_pipes(&input);

    let mut main_loop: HashMap<(usize, usize), Pipe> = HashMap::new();
    main_loop.insert(start_pos, map.get(&start_pos).unwrap().clone());

    let mut current_pos = start_pos.clone();
    let mut distance = 0;

    loop {
        let (left, right) = get_allowed_neighbours(map.get(&current_pos).unwrap());
        let mut neighbour = left.clone();

        if !main_loop.contains_key(&(right.0.unwrap(), right.1.unwrap())) {
            neighbour = right.clone();
        }

        let pipe = map.get(&(neighbour.0.unwrap(), neighbour.1.unwrap())).unwrap().clone();
        distance += 1;

        main_loop.insert(pipe.coordinates, Pipe {
            is_in_loop: false,
            distance_from_start: distance,
            pipe_type: pipe.pipe_type,
            coordinates: pipe.coordinates,
            is_on_main_loop: true,
        });

        current_pos = pipe.coordinates.clone();

        if current_pos == start_pos {
            break;
        }
    }

    println!("Reached position {:?} in {} pipes, max distance is {} and identified the following main loop {:?}", current_pos, main_loop.len(), main_loop.len() / 2usize, main_loop);

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The max distance is {}, found in {:?}", main_loop.len() / 2, end - start);
}

pub fn solve_part_2() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let input = fs::read_to_string("src/day10/input.txt").unwrap();
    let (map, start_pos, max_size) = parse_pipes(&input);

    let mut main_loop: HashMap<(usize, usize), Pipe> = HashMap::new();
    main_loop.insert(start_pos, map.get(&start_pos).unwrap().clone());

    let mut current_pos = start_pos.clone();

    loop {
        let (left, right) = get_allowed_neighbours(map.get(&current_pos).unwrap());
        let mut neighbour = left.clone();

        if !main_loop.contains_key(&(right.0.unwrap(), right.1.unwrap())) {
            neighbour = right.clone();
        }

        let mut pipe = map.get(&(neighbour.0.unwrap(), neighbour.1.unwrap())).unwrap().clone();
        let coordinates = pipe.coordinates;

        main_loop.insert(coordinates, Pipe {
            is_in_loop: false,
            distance_from_start: 0,
            pipe_type: pipe.pipe_type,
            coordinates,
            is_on_main_loop: true,
        });

        current_pos = coordinates.clone();

        if current_pos == start_pos {
            break;
        }
    }

    // Write to intermediate file
    let mut res = String::new();
    let line_length = max_size.0;

    for (_, c) in input.chars().enumerate() {
        // If x,y is not on main loop -> push ground
        let current_len = res.len();

        // +1 for the \n
        let y = current_len / (line_length + 1);
        let x = current_len - (y * (line_length + 1));

        // If it's not a new line and its not on the loop, replace with .
        let is_on_the_main_loop = main_loop.contains_key(&(x, y));

        match c {
            '\n' => res.push('\n'),
            _ => {
                if is_on_the_main_loop {
                    res.push(c);
                } else {
                    res.push('.');
                }
            }
        }
    }

    let mut file = File::create("src/day10/intermediary_input.txt").expect("Could not create file");
    file.write_all(res.as_bytes()).expect("Could not write to file");

    let input = fs::read_to_string("src/day10/intermediary_input.txt").unwrap();
    add_space_to_map(input, &max_size);

    let input = fs::read_to_string("src/day10/padded_input.txt").unwrap();
    let (map, _, max_size) = parse_pipes(&input);

    let cloned_map = map.clone();
    let mut ground_tiles = cloned_map.into_iter()
        .filter(|(_, value)| value.pipe_type == Ground)
        .collect::<HashMap<_, _>>();

    let mut result: Vec<Vec<Pipe>> = vec![]; // Collection of all connected ground zones
    let mut visited_non_ground_tiles: HashMap<(usize, usize), Pipe> = HashMap::new();

    // As long as there are ground tiles left to parse
    while !ground_tiles.is_empty() {
        // Grab a tile, initiate a new zone and add it to the tiles to visit
        let ground_tile = ground_tiles.iter().next().unwrap();
        let mut tiles_to_visit: Vec<Pipe> = vec![ground_tile.1.to_owned()];
        let mut tiles_in_zone: Vec<Pipe> = vec![];

        // As long as there are tiles to visit in the current zone
        while !tiles_to_visit.is_empty() {
            // Grab and delete the first tile that has yet to be visited
            let tile = tiles_to_visit.remove(0);

            // Check if we've seen the tile before, if not grab it
            let tile_has_to_be_visited;
            {
                tile_has_to_be_visited = ground_tiles.get(&tile.coordinates).is_some();
            }

            // If it was a non-ground neighbour
            if tile.pipe_type != Ground {
                // Check if we've already visited the tile
                let non_ground_tile = visited_non_ground_tiles.get(&tile.coordinates);
                match non_ground_tile {
                    // Do nothing if so
                    Some(_) => {}
                    // If not, calculate the neighbours and add
                    None => {
                        tiles_to_visit.extend(find_ground_like_neighbours(&tile.coordinates, &map));
                        visited_non_ground_tiles.insert(tile.coordinates, tile.clone());
                        tiles_in_zone.push(tile);
                    }
                }
            }

            // If the ground tile has yet to be visited, or the tile is not a ground tile
            if tile_has_to_be_visited {
                let tile = ground_tiles.remove(&tile.coordinates).unwrap();
                tiles_to_visit.extend(find_ground_like_neighbours(&tile.coordinates, &map));
                tiles_in_zone.push(tile);
            }
        }

        result.push(tiles_in_zone);
    }

    let mut closed_in_tiles = 0;
    for (i, res) in result.iter().enumerate() {
        let is_closed_in = is_closed_in_by_main_loop(res, max_size);
        for pipe in res {
            if pipe.pipe_type == Ground {
                if is_closed_in {
                    closed_in_tiles += 1;
                }
            }
        }
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("The amount of closed in tiles is {}, found in {:?}", closed_in_tiles, end - start);
}

pub fn is_closed_in_by_main_loop(zone: &Vec<Pipe>, max_size: (usize, usize)) -> bool {
    for tile in zone {
        if tile.coordinates.0 == 0 || tile.coordinates.0 == max_size.0 || tile.coordinates.1 == 0 || tile.coordinates.1 == max_size.1 {
            return false;
        }
    }

    true
}

pub fn find_ground_like_neighbours(active_tile: &(usize, usize), map: &HashMap<(usize, usize), Pipe>) -> Vec<Pipe> {
    let mut neighbours: Vec<Pipe> = vec![];

    let get_valid_neighbour = |coordinate: (Option<usize>, Option<usize>), i: usize| {
        if let (Some(x), Some(y)) = coordinate {
            if let Some(pipe) = map.get(&(x, y)) {
                if pipe.pipe_type == Ground || pipe.pipe_type == Space {
                    return Some(pipe.clone());
                }
            }
        }
        None
    };

    let directions = [
        (Some(active_tile.0), active_tile.1.checked_sub(1)), // North: 0
        (Some(active_tile.0 + 1), Some(active_tile.1)), // East: 1
        (Some(active_tile.0), Some(active_tile.1 + 1)), // South: 2
        (active_tile.0.checked_sub(1), Some(active_tile.1)), // West: 3
    ];

    for (i, direction) in directions.iter().enumerate() {
        if let Some(pipe) = get_valid_neighbour(*direction, i) {
            neighbours.push(pipe);
        }
    }

    neighbours
}

pub fn parse_pipes(input: &String) -> (HashMap<(usize, usize), Pipe>, (usize, usize), (usize, usize)) {
    let mut output: HashMap<(usize, usize), Pipe> = HashMap::new();
    let mut start_coords: (usize, usize) = (0usize, 0usize);
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, pipe) in line.chars().enumerate() {
            let is_start = pipe == 'S';

            if is_start {
                start_coords = (x, y);
            }

            output.insert((x, y), Pipe {
                coordinates: (x, y),
                pipe_type: map_char_to_pipe_type(pipe),
                is_on_main_loop: is_start,
                is_in_loop: false,
                distance_from_start: 0,
            });
            max_x += 1;
        }
        max_y += 1;
    }

    (output, start_coords, ((max_x / max_y), max_y))
}

pub fn add_space_to_map(input: String, initial_size: &(usize, usize)) {
    let mut res = String::new();
    let line_length = (initial_size.0 * 2) + 1;

    for (i, c) in input.chars().enumerate() {
        if i != 0 && i % (initial_size.0 + 1) == 0 {
            for i in 0..line_length - 1 {
                let current_set: Vec<char> = res.clone().chars().collect();
                match current_set[res.len() - line_length] {
                    'F' | '7' | '|' | 'S' => res.push('|'),
                    _ => res.push('X'),
                }
            }

            res.push('\n');
        }
        res.push(c);

        match c {
            '\n' => {}
            'F' => res.push('-'),
            'L' => res.push('-'),
            '.' => res.push('X'),
            '-' => res.push('-'),
            _ => res.push('X'),
        }
    }

    let mut file = File::create("src/day10/padded_input.txt").expect("Could not create file");
    file.write_all(res.as_bytes()).expect("Could not write to file");
}

pub fn map_char_to_pipe_type(c: char) -> PipeType {
    match c {
        '|' => PipeType::NorthSouth,
        '-' => PipeType::EastWest,
        'L' => PipeType::NorthEast,
        'J' => PipeType::NorthWest,
        '7' => PipeType::SouthWest,
        'F' => PipeType::SouthEast,
        '.' => PipeType::Ground,
        'X' => PipeType::Space,
        'S' => PipeType::Start,
        _ => panic!("Should never happen.")
    }
}

pub fn get_allowed_neighbours(pipe: &Pipe) -> ((Option<usize>, Option<usize>), (Option<usize>, Option<usize>)) {
    let north = (Some(pipe.coordinates.0), pipe.coordinates.1.checked_sub(1));
    let east = (Some(pipe.coordinates.0 + 1), Some(pipe.coordinates.1));
    let south = (Some(pipe.coordinates.0), Some(pipe.coordinates.1 + 1));
    let west = (pipe.coordinates.0.checked_sub(1), Some(pipe.coordinates.1));

    match pipe.pipe_type.clone() {
        PipeType::NorthSouth | PipeType::Start => (north, south),
        PipeType::EastWest => (east, west),
        PipeType::NorthEast => (north, east),
        PipeType::NorthWest => (north, west),
        PipeType::SouthWest => (south, west),
        PipeType::SouthEast => (south, east),
        value => panic!("Should never happen, fired on {:?}", value)
    }
}