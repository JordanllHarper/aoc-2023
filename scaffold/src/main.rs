use std::fs::read_to_string;

mod models;
mod parser;
mod solve;
fn main() {
    let input = read_lines("input.txt");
    let output = part_one(input);
    // println!("{}", output);
}

fn part_one(input: Vec<String>) {
    todo!()
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
    fn part_one_test() {}
    // TODO:
    #[test]
    fn part_two_test() {}
}
