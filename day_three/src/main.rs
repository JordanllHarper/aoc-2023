use std::fs::read_to_string;

use util::{create_2d_grid, get_notable_characters_in_line, search_around_gears, Coordinate};

use crate::util::{parse_complete_characters, CompleteCharacters, NotableCharacters};

mod models;
mod util;
fn main() {
    let input = read_lines("input.txt");
    let output = part_two(input);
    println!("{}", output);
}

// fn part_one(input: Vec<String>) -> usize {
//     // let schematic = input.iter().map(|line| build_vector_schematic(line));
//     let grid = create_2d_grid(input);
//
//     let mut add_numbers = Vec::new();
//
//     for (y_coord, element) in grid.clone().into_iter().enumerate() {
//         let number_characters = get_numbers_in_line(&element, y_coord.try_into().unwrap());
//         let full_numbers = parse_numbers_in_line(number_characters);
//         match full_numbers {
//             Some(numbers) => {
//                 let to_add_numbers =
//                     get_added_numbers(numbers, grid.clone(), y_coord.try_into().unwrap());
//                 add_numbers.push(to_add_numbers);
//             }
//             None => continue,
//         }
//     }
//     add_numbers.iter().sum()
// }

fn part_two(input: Vec<String>) -> usize {
    let grid = create_2d_grid(input);
    let complete_characters = grid
        .iter()
        .enumerate()
        .map(|(y_coord, line)| get_notable_characters_in_line(line, y_coord))
        .map(|notable| parse_complete_characters(&notable))
        .collect::<Vec<Vec<CompleteCharacters>>>();
    let gears_to_search = complete_characters
        .clone()
        .into_iter()
        .flatten()
        .filter_map(|value| match value {
            CompleteCharacters::Number(_) => None,
            CompleteCharacters::Gear(c) => Some(c),
        })
        .collect::<Vec<Coordinate>>();

    let result = search_around_gears(gears_to_search, complete_characters.clone());

    result
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
    use crate::part_two;

    // #[test]
    // fn part_one_test() {
    //     let data= "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
    //     let expected = 1;
    //     let actual = part_one(
    //         data.lines()
    //             .map(|line| line.to_string())
    //             .collect::<Vec<String>>(),
    //     );
    //     assert_eq!(expected, actual);
    // }

    #[test]
    fn part_two_test() {
        let data= "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let expected = 467835;
        let actual = part_two(
            data.lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(expected, actual);
    }
}
