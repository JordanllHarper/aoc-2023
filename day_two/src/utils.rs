use crate::{models::CubeSet, Cube, Game};

pub fn get_cube_colour(word: &str, number: i32) -> Cube {
    match word {
        "blue" => Cube::Blue(number),
        "red" => Cube::Red(number),
        "green" => Cube::Green(number),
        _ => panic!(), //this won't happen
    }
}
// delimit colon : -> Game id
// delimit semi colons ;  -> sets
// delimit comma , -> cubes

/// assumes no comma
// e.g. 3 blue
pub fn get_cube(data: &str) -> Cube {
    let cube_split = data.trim().split(' ').collect::<Vec<&str>>();
    let number = cube_split.first().unwrap().trim().parse::<i32>().unwrap();
    let colour = cube_split.last().unwrap().trim();
    get_cube_colour(&colour, number)
}

// e.g. 3 blue, 4 red;
pub fn get_cube_set(data: &str) -> CubeSet {
    let split = data
        .split(',')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| get_cube(line))
        .collect::<Vec<Cube>>();

    CubeSet::new(split)
}

// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn get_game_sets(data: &str) -> Vec<CubeSet> {
    let split = data
        .split(';')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|set| get_cube_set(set))
        .collect::<Vec<CubeSet>>();

    split
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn get_game(data: &str) -> Game {
    let split = data.split(':').collect::<Vec<&str>>();

    let game_info = split.first().unwrap();
    let id = parse_game_id(game_info);
    let game_data = split.last().unwrap();
    let game_sets = get_game_sets(game_data);
    Game::new(id, game_sets)
}

fn parse_game_id(data: &str) -> i32 {
    let split = data.trim().split(' ').collect::<Vec<&str>>();
    let last = split.last().unwrap().trim();

    last.parse::<i32>().unwrap()
}
