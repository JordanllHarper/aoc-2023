use std::fs::read_to_string;

use crate::solve::get_num_variations;

mod models;
mod parser;
mod solve;
mod spring_iterator;
fn main() {
    let input = read_lines("input.txt");
    let output = part_one(input);
    // println!("{}", output);
}

fn part_one(input: Vec<String>) -> i32 {
    parser::parse_to_models(input)
        .into_iter()
        .map(|record| get_num_variations(&record))
        .sum()
}

// TODO:
fn part_two(input: Vec<String>) {
    todo!()
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
        let data = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let expected = 21;
        let actual = part_one(data.lines().map(|s| s.to_string()).collect());
        assert_eq!(expected, actual);
    }
    // TODO:
    #[test]
    fn part_two_test() {}
}
