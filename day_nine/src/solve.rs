use std::{collections::HashSet, usize};

use crate::models::ListStage;

pub fn work_out_differences_in_sequence(numbers: Vec<i64>) -> i64 {
    // TODO:
    // Recursively make lists
    // stop when the list contains ONLY duplicates -> this would result in the next list being 0's
    // once we hit the floor, take the last value and add to 0 ->

    let differences = differences_between_numbers(numbers);
    if contains_only_dups(&differences) {
        return *differences.first().unwrap();
    }
    return work_out_differences_in_sequence(differences.clone()) + differences.last().unwrap();
}
pub fn work_out_before_first(numbers: Vec<i64>) -> i64 {
    let final_stage = ListStage::new(
        *numbers.first().unwrap(),
        Box::new(Some(get_differences_first_value(numbers))),
    );

    let mut list_stages = collect_stages_into_list(final_stage, &mut Vec::new());

    list_stages.reverse();
    let mut accum = list_stages.first().unwrap().to_owned();
    list_stages.remove(0);
    println!("Accum before iteration {}", accum);
    println!("List: {:?}", list_stages);
    for number in list_stages {
        accum = number - accum;
        println!("{}", accum);
    }
    accum
}
pub fn collect_stages_into_list(start: ListStage, collected_nums: &mut Vec<i64>) -> Vec<i64> {
    let this_num = start.first_of_this_stage;
    collected_nums.push(this_num);

    let opt_prev_stage = *start.previous_step;
    match opt_prev_stage {
        Some(v) => return collect_stages_into_list(v, collected_nums),
        None => return collected_nums.to_vec(),
    }
}
pub fn get_differences_first_value(numbers: Vec<i64>) -> ListStage {
    let differences = differences_between_numbers(numbers);
    if contains_only_dups(&differences) {
        return ListStage::new(*differences.first().unwrap(), Box::new(None));
    }
    return ListStage::new(
        *differences.first().unwrap(),
        Box::new(Some(get_differences_first_value(differences))),
    );
}
pub fn differences_between_numbers(numbers: Vec<i64>) -> Vec<i64> {
    numbers
        .windows(2)
        .into_iter()
        .map(|pair| pair.last().unwrap() - pair.first().unwrap())
        .collect::<Vec<i64>>()
}
pub fn contains_only_dups(numbers: &Vec<i64>) -> bool {
    numbers.into_iter().collect::<HashSet<&i64>>().len() == 1
}
