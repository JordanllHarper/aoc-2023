use std::fs::read_to_string;

use parser::parse_to_tokens;

use crate::solve::{
    get_inner_shape_squares, get_start, get_start_pipe, get_steps_furthest_away, parse_pipe_shape,
};

mod models;
mod parser;
mod solve;
mod token_iterator;
fn main() {
    let input = read_lines("input.txt");
    let output = part_two(input);
    println!("{}", output);
}

fn part_one(input: Vec<String>) -> i32 {
    let chars = input
        .into_iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let pipe_grid = parse_to_tokens(chars);
    let minimum = get_steps_furthest_away(pipe_grid);
    minimum / 2
}

// TODO:
fn part_two(input: Vec<String>) -> i32 {
    let chars = input
        .into_iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let full_grid = parse_to_tokens(chars);
    let pipe_shape = parse_pipe_shape(full_grid.clone());
    let start_item = get_start(full_grid.clone());
    let start_pipe = get_start_pipe(&full_grid, &start_item);
    let inner_shape_squares = get_inner_shape_squares(full_grid, pipe_shape, start_pipe)
        .into_iter()
        .count();

    inner_shape_squares.try_into().unwrap()
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
        let data = r".....
.S-7.
.|.|.
.L-J.
.....";
        let expected = 4;
        let actual = part_one(data.lines().map(|line| line.to_string()).collect());
        assert_eq!(expected, actual);

        let data = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let expected = 8;
        let actual = part_one(data.lines().map(|line| line.to_string()).collect());
        assert_eq!(expected, actual);
    }
    // TODO:
    #[test]
    fn part_two_test() {
        let data = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        let expected = 4;
        let actual = part_two(data.lines().map(|line| line.to_string()).collect());
        assert_eq!(expected, actual);
        let data = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let expected = 10;
        let actual = part_two(data.lines().map(|line| line.to_string()).collect());
        assert_eq!(expected, actual);
    }
}
