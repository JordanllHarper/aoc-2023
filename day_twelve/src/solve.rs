use crate::{
    models::{KnownSpringSpan, KnownSpringType, Record, SpringType},
    spring_iterator::SpringIterator,
};
pub fn get_num_variations(record: &Record) -> i32 {
    let nums_to_satisfy = record.numbers.clone();

    let mut num_variations = 0;
    let variations = generate_variations(record.springs.clone(), 0);
    for variation in variations {
        let span = parse_into_spans(variation.clone());
        if satisfies_nums(nums_to_satisfy.clone(), span) {
            num_variations += 1;
        }
    }
    num_variations
}
pub fn generate_variations(
    springs: Vec<SpringType>,
    index_to_test: i32,
) -> Vec<Vec<KnownSpringType>> {
    let variations: Vec<Vec<KnownSpringType>> = Vec::new();

    variations
}
pub fn parse_into_spans(variation: Vec<KnownSpringType>) -> Vec<KnownSpringSpan> {
    SpringIterator::new(variation).collect()
}
pub fn satisfies_nums(nums: Vec<i32>, springs: Vec<KnownSpringSpan>) -> bool {
    let damaged_spans: Vec<KnownSpringSpan> = springs
        .into_iter()
        .filter(|s| s.spring_type == KnownSpringType::Damaged)
        .collect();
    nums.iter()
        .zip(damaged_spans)
        .map(|(num, damaged)| !damaged.matches_num(*num))
        .filter(|b| *b == false)
        // we want all of them to be true so count any that are false  -> this should be zero
        .count()
        == 0
}
