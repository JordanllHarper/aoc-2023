use std::{collections::HashSet, fs::read_to_string};

use solve::{get_seed_destinations, parse_almanac};

use crate::{
    models::Range,
    solve::{get_final_ranges, parse_seed_range},
};

mod models;
mod solve;
fn main() {
    let input = read_stream("input.txt");
    let output = part_two(&input);
    println!("{}", output);
}

fn part_one(input: &str) -> i64 {
    let almanac = parse_almanac(input);
    println!("Almanac: {:?}", almanac);
    let final_transform = get_seed_destinations(almanac);
    *final_transform.iter().min().unwrap()
}

fn part_two(input: &str) -> i64 {
    let almanac = parse_almanac(input);
    let minimum = get_final_ranges(almanac)
        .iter()
        .map(|item| item.start)
        .min()
        .unwrap();

    println!("{:?}", minimum);
    minimum
}

fn read_stream(filename: &str) -> String {
    read_to_string(filename).unwrap()
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
        let data  = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n";
        let expected = 35;
        let actual = part_one(data);

        assert_eq!(expected, actual);
    }
    #[test]
    fn part_two_test() {
        let data  = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n";
        let expected = 46;
        let actual = part_two(data);
        assert_eq!(expected, actual);
    }
}
