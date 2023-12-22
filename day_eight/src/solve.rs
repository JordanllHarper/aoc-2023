use std::collections::HashMap;

use num::Integer;

use crate::models::{Direction, Map};

pub fn count_steps(map: Map) -> u32 {
    let mut current_destination = "AAA";
    let mut steps = 0;

    let mut cycling_iterator = map.directions.iter().cycle();
    while current_destination != "ZZZ" {
        let next_direction = cycling_iterator.next().unwrap();

        let current = map.nodes.get(current_destination).unwrap();
        match next_direction {
            crate::models::Direction::Left => current_destination = &current.left,
            crate::models::Direction::Right => current_destination = &current.right,
        }
        steps += 1;
    }

    steps
}

pub fn count_simul_steps(map: Map) -> u64 {
    let starting_dest = map
        .nodes
        .iter()
        .filter(|value| value.0.chars().last().unwrap() == 'A') // init with only a values
        .map(|each| each.0.to_string())
        .collect::<Vec<String>>();

    println!("{:?}", starting_dest);
    let mut steps_across_nodes = Vec::new();

    let mut cycling_iterator = map.directions.iter().cycle();
    for starting in starting_dest {
        let mut current_dest = starting.clone();
        let mut node_steps = 0;
        while current_dest.chars().last().unwrap() != 'Z' {
            let next_direction = cycling_iterator.next().unwrap();
            node_steps += 1;
            current_dest = to_next_destination(&map, next_direction, &current_dest);
        }
        steps_across_nodes.push(node_steps);
    }

    println!("{:?}", steps_across_nodes);

    steps_across_nodes
        .into_iter()
        .reduce(|accum, b| find_lowest_common_multiple(accum, b))
        .unwrap()
}
pub fn find_lowest_common_multiple(num_one: u64, num_two: u64) -> u64 {
    let (lcm, gcd) = num_one.gcd_lcm(&num_two);
    gcd
}
pub fn get_prime_factors(limit: u64) -> Vec<u64> {
    let mut numbers_between_2_and_limit = (2..=limit)
        .map(|each| (each, true))
        .collect::<HashMap<u64, bool>>();
    for i in 2..=limit {
        if *numbers_between_2_and_limit.get(&i).unwrap() {
            let mut j = u64::pow(i, 2);
            while j <= limit {
                numbers_between_2_and_limit
                    .entry(j)
                    .and_modify(|value| *value = false);
                j = j + i;
            }
        }
    }
    numbers_between_2_and_limit
        .into_iter()
        .filter_map(|each| if each.1 { Some(each.0) } else { None })
        .collect::<Vec<u64>>()
}
fn to_next_destination(map: &Map, next_direction: &Direction, current_destination: &str) -> String {
    let current = map.nodes.get(current_destination).unwrap();
    match next_direction {
        crate::models::Direction::Left => current.left.clone(),
        crate::models::Direction::Right => current.right.clone(),
    }
}
