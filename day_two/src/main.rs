use std::fs::read_to_string;

use models::{Cube, Game};
use utils::get_game;
mod models;
mod utils;

fn main() {
    let input = read_lines("input.txt");
    let output = part_two(input);
    println!("{}", output);
}

fn find_max_of_colours(game: Game) -> (i32, i32, i32) {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    let sets = game.cube_sets;

    for set in sets {
        let cubes = set.cubes;

        for cube in cubes {
            match cube {
                Cube::Red(v) => {
                    if v > max_red {
                        max_red = v;
                    }
                }
                Cube::Green(v) => {
                    if v > max_green {
                        max_green = v;
                    }
                }
                Cube::Blue(v) => {
                    if v > max_blue {
                        max_blue = v;
                    }
                }
            }
        }
    }

    (max_red, max_green, max_blue)
}

pub fn part_two(input: Vec<String>) -> i32 {
    let sum: i32 = get_games(input)
        .into_iter()
        .map(|game| find_max_of_colours(game))
        .map(|max| max.0 * max.1 * max.2)
        .sum();

    sum
}

// red, green, blue
fn is_valid_game(game: Game) -> bool {
    let upper_red = 12;
    let upper_green = 13;
    let upper_blue = 14;
    let cube_sets = game.cube_sets;

    for set in cube_sets {
        for cube in set.cubes {
            match cube {
                Cube::Red(value) => {
                    if value > upper_red {
                        return false;
                    }
                }
                Cube::Green(value) => {
                    if value > upper_green {
                        return false;
                    }
                }
                Cube::Blue(value) => {
                    if value > upper_blue {
                        return false;
                    }
                }
            }
        }
    }
    true
}

struct GameValid {
    id: i32,
    is_valid: bool,
}

impl GameValid {
    fn new(id: i32, is_valid: bool) -> Self {
        Self { id, is_valid }
    }
}
pub fn get_games(input: Vec<String>) -> Vec<Game> {
    let games = input
        .into_iter()
        .map(|line| get_game(&line))
        .collect::<Vec<Game>>();
    games
}

pub fn part_one(input: Vec<String>) -> i32 {
    let games = get_games(input);
    let count: i32 = games
        .into_iter()
        .map(|game| {
            let id = game.id;
            let is_valid = is_valid_game(game);
            GameValid::new(id, is_valid)
        })
        .filter(|game_with_validation| game_with_validation.is_valid)
        .map(|valid_games| valid_games.id)
        .sum();

    count
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn part_one_test() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected: i32 = 8;
        let actual = part_one(
            data.lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>(),
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_test() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected: i32 = 2286;
        let actual = part_two(
            data.lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>(),
        );

        assert_eq!(expected, actual);
    }
}
