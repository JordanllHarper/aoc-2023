use std::fs::read_to_string;

use solve::{
    adjust_galaxy_positions_pt1, adjust_galaxy_positions_pt2, get_column_adjustments, get_galaxies,
    get_galaxy_pairs, get_row_adjustments, manhattan_distance,
};

use crate::{parser::parse_to_space_items, solve::expand_rows};

mod models;
mod parser;
mod solve;
fn main() {
    let input = read_lines("input.txt");
    let output = part_two(input);
    println!("{}", output);
}

fn part_one(input: Vec<String>) -> i64 {
    let space_items = parse_to_space_items(input);
    let column_offset = get_column_adjustments(&space_items);
    let row_offset = get_row_adjustments(&space_items);
    let galaxies = get_galaxies(space_items);
    let offset_galaxy_x_y = adjust_galaxy_positions_pt2(galaxies, column_offset, row_offset, 1);
    let pairs = get_galaxy_pairs(offset_galaxy_x_y);
    let distances: Vec<i64> = pairs
        .into_iter()
        .map(|pair| manhattan_distance(&pair.a, &pair.b))
        .collect();

    distances.into_iter().sum()
}

// TODO:
fn part_two(input: Vec<String>) -> i64 {
    let space_items = parse_to_space_items(input);
    let column_offset = get_column_adjustments(&space_items);
    let row_offset = get_row_adjustments(&space_items);
    let galaxies = get_galaxies(space_items);
    let offset_galaxy_x_y =
        adjust_galaxy_positions_pt2(galaxies, column_offset, row_offset, 1000000 - 1);
    let pairs = get_galaxy_pairs(offset_galaxy_x_y);
    let distances: Vec<i64> = pairs
        .into_iter()
        .map(|pair| manhattan_distance(&pair.a, &pair.b))
        .collect();

    distances.into_iter().sum()
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
        let data = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let expected = 374;
        let actual = part_one(data.lines().map(|s| s.to_string()).collect());
        assert_eq!(expected, actual);
    }
    // TODO:
    #[test]
    fn part_two_test() {
        let data = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let expected = 8410;
        let actual = part_two(data.lines().map(|s| s.to_string()).collect());
        assert_eq!(expected, actual);
    }
}
