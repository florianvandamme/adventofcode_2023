use std::fs;

#[derive(Debug)]
pub struct Game {
    id: i32,
    cube_reveals: Vec<CubeReveal>,
}

#[derive(Debug)]
pub struct CubeReveal {
    red: i32,
    green: i32,
    blue: i32,
}

pub fn solve_part_1() {
    let input = fs::read_to_string("src/day2/input.txt").unwrap();
    let games = input.lines();

    let mut total_sum = 0;
    for game in games {
        let parsed_game = parse_game(game);
        println!("{:?}", parsed_game);

        if is_game_possible(&parsed_game, 12, 13, 14) {
            total_sum += parsed_game.id
        }
    }

    println!("The sum of all the possible games is: {}", total_sum);
}

pub fn solve_part_2() {
    let input = fs::read_to_string("src/day2/input.txt").unwrap();
    let games = input.lines();

    let mut total_sum = 0;
    for game in games {
        let parsed_game = parse_game(game);
        let game_cube_power = get_game_cube_power(&parsed_game);

        println!("{}", game_cube_power);
        total_sum += game_cube_power;
    }

    println!("The sum of all the possible games is: {}", total_sum);
}

pub fn is_game_possible(game: &Game, max_red: i32, max_green: i32, max_blue: i32) -> bool {
    for reveal in game.cube_reveals.iter().clone() {
        if (max_red < reveal.red || max_green < reveal.green || max_blue < reveal.blue) {
            return false;
        }
    }

    true
}

pub fn get_game_cube_power(game: &Game) -> i32 {
    let mut highest_reveal = CubeReveal {
        red: 0,
        green: 0,
        blue: 0,
    };

    for reveal in &game.cube_reveals {
        if reveal.red > highest_reveal.red {
            highest_reveal.red = reveal.red;
        }

        if reveal.green > highest_reveal.green {
            highest_reveal.green = reveal.green;
        }

        if reveal.blue > highest_reveal.blue {
            highest_reveal.blue = reveal.blue;
        }
    }

    highest_reveal.red * highest_reveal.green * highest_reveal.blue
}

pub fn parse_game(game: &str) -> Game {
    let mut parts = game.split(": ");

    let id_string = parts.next().unwrap();
    let id_parts = id_string.split(" ");
    let id = id_parts.last().unwrap().parse::<i32>().unwrap();

    let unparsed_moves = parts.next().unwrap();

    Game {
        id,
        cube_reveals: parse_moves(unparsed_moves),
    }
}

pub fn parse_moves(unparsed_moves: &str) -> Vec<CubeReveal> {
    let mut moves: Vec<CubeReveal> = vec![];
    let reveals = unparsed_moves.split("; ");

    for reveal in reveals {
        let colors = reveal.split(", ");
        let mut reveal = CubeReveal {
            green: 0,
            red: 0,
            blue: 0,
        };

        for color in colors {
            let parts: Vec<&str> = color.split(" ").collect();

            match parts.last().unwrap() {
                &"green" => reveal.green = parts.first().unwrap().parse::<i32>().unwrap(),
                &"red" => reveal.red = parts.first().unwrap().parse::<i32>().unwrap(),
                &"blue" => reveal.blue = parts.first().unwrap().parse::<i32>().unwrap(),
                _ => continue,
            }
        }

        moves.push(reveal);
    }

    moves
}
