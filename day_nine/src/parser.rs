pub fn parse_to_number_line(input: Vec<String>) -> Vec<Vec<i64>> {
    input
        .iter()
        .map(|each| {
            each.split_whitespace()
                .map(|each| each.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}
