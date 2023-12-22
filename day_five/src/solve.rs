use crate::models::{Almanac, ItemToItemSpec, ItemToItemType, MapRange, Range};

pub fn parse_almanac(full_stream: &str) -> Almanac {
    let split: Vec<&str> = full_stream.split("\n\n").collect();
    let first_and_ranges = split.split_first().unwrap();
    let start_seed_collection = parse_seeds(&first_and_ranges.0);
    let maps = first_and_ranges
        .1
        .into_iter()
        .map(|range_str| parse_item_to_item(&range_str))
        .collect::<Vec<ItemToItemSpec>>();

    Almanac::new(start_seed_collection, maps)
}

pub fn parse_seeds(line: &str) -> Vec<i64> {
    line.trim()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|num| num.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
pub fn parse_item_to_item_type(line: &str) -> ItemToItemType {
    let title = line.split_whitespace().next().unwrap();
    ItemToItemType::from_str(title).unwrap()
}

pub fn parse_item_to_item_number_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|each| each.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn parse_item_to_item_lines(lines: &[&str]) -> Vec<Vec<i64>> {
    lines
        .iter()
        .map(|line| parse_item_to_item_number_line(line))
        .collect::<Vec<Vec<i64>>>()
}

pub fn get_seed_destinations(almanac: Almanac) -> Vec<i64> {
    let seeds = almanac.original_seed_list;
    let maps = almanac.maps;
    println!("START SEEDS => {:?}", seeds);

    let mut seed_transform = seeds; //init with og seeds
                                    //

    for map in maps {
        println!("||| Step {:?} |||\n", map.item_to_item_type);
        seed_transform = transform_line_through_range(&seed_transform, map);
    }

    seed_transform
}

pub fn get_final_ranges(almanac: Almanac) -> Vec<Range> {
    let seeds = almanac.original_seed_list;
    let seed_range = parse_seed_range(&seeds);
    let section_maps = almanac
        .maps
        .iter()
        .map(|each| each.ranges())
        .collect::<Vec<Vec<MapRange>>>();

    let mut transformed_range = seed_range;
    for (index, section) in section_maps.into_iter().enumerate() {
        let section_number = index + 1;
        println!("Section: {section_number}\n {:#?}", section);

        transformed_range = run_seeds_through_ranges(section, transformed_range);
        println!("Transformed seed range:{:#?}", transformed_range);
    }
    transformed_range
}
pub fn run_seeds_through_ranges(
    section_ranges: Vec<MapRange>,
    seed_ranges: Vec<Range>,
) -> Vec<Range> {
    let mut untransformed_seeds = seed_ranges;

    let mut transformed_seeds: Vec<Range> = Vec::new();

    for section in section_ranges {
        let transformed_to_non = untransformed_seeds
            .iter()
            .map(|seed_range| transform_seed_range(*seed_range, section))
            .map(|transformed| get_transformed_to_non_transformed(transformed))
            .collect::<Vec<(Vec<Range>, Vec<Range>)>>();

        for transformed in transformed_to_non.clone() {
            transformed_seeds.extend(transformed.0)
        }

        untransformed_seeds = Vec::new();
        for transformed in transformed_to_non {
            untransformed_seeds.extend(transformed.1)
        }
    }
    transformed_seeds.extend(untransformed_seeds);

    transformed_seeds
}

pub fn get_transformed_to_non_transformed(
    transforms: Vec<(Range, bool)>,
) -> (Vec<Range>, Vec<Range>) {
    (
        transforms
            .iter()
            .filter_map(|each| if each.1 { Some(each.0) } else { None })
            .collect::<Vec<Range>>(),
        transforms
            .iter()
            .filter_map(|each| if !each.1 { Some(each.0) } else { None })
            .collect::<Vec<Range>>(),
    )
}

pub fn transform_seed_range(seed_range: Range, map_range: MapRange) -> Vec<(Range, bool)> {
    let source_range = map_range.source;
    if seed_outside_range(seed_range, source_range) {
        return vec![(seed_range, false)];
    }

    let source_destination_difference = map_range.difference_between();
    if seed_in_map(seed_range, source_range) {
        println!("\nSeed in map\n");

        let vec = vec![(
            Range::new(
                seed_range.start + source_destination_difference,
                seed_range.end + source_destination_difference,
            ),
            true,
        )];

        return vec;
    }
    if map_in_seed(seed_range, source_range) {
        println!("\nMap in seed\n");
        let left = Range::new(seed_range.start, source_range.start - 1);
        let right = Range::new(source_range.end + 1, seed_range.end - 1);

        let middle = Range::new(
            source_range.start + source_destination_difference,
            source_range.end + source_destination_difference,
        );

        let vec = vec![(left, false), (right, false), (middle, true)];
        println!("{:?}", vec);
        return vec;
    }

    if seed_range.start <= source_range.start && seed_range.end < source_range.end {
        // map seed from source start to seed end

        let non_mapped = Range::new(seed_range.start, source_range.start - 1);

        let mapped = Range::new(
            source_range.start + source_destination_difference,
            seed_range.end + source_destination_difference,
        );

        return vec![(mapped, true), (non_mapped, false)];
    } else {
        // map seed from seed start to source end

        let mapped = Range::new(
            seed_range.start + source_destination_difference,
            source_range.end + source_destination_difference,
        );
        let non_mapped = Range::new(source_range.end + 1, seed_range.end);
        return vec![(mapped, true), (non_mapped, false)];
    }
}

/// Seed outside the map range
pub fn seed_outside_range(seed_range: Range, source_range: Range) -> bool {
    seed_range.start > source_range.end || seed_range.end < source_range.start
}
/// Seed range contained in the map range
pub fn seed_in_map(seed_range: Range, source_range: Range) -> bool {
    seed_range.start >= source_range.start && seed_range.end <= source_range.end
}
/// Map range contained in the seed range
pub fn map_in_seed(seed_range: Range, source_range: Range) -> bool {
    seed_range.start <= source_range.start && seed_range.end >= source_range.end
}

pub fn transform_line_through_range(seed_line: &[i64], map: ItemToItemSpec) -> Vec<i64> {
    let mappings = map
        .map_numbers
        .iter()
        .map(|line| MapRange::from(&line))
        .collect::<Vec<MapRange>>();
    println!("{:?}", seed_line);

    seed_line
        .iter()
        .map(|number| {
            match mappings
                .iter()
                .find(|mapping| mapping.source.number_falls_in_range(*number))
            {
                Some(number_in_range) => {
                    number.checked_sub(number_in_range.source.start).unwrap()
                        + number_in_range.destination.start
                }
                None => *number,
            }
        })
        .collect::<Vec<i64>>()
}

pub fn parse_seed_range(seed_list: &[i64]) -> Vec<Range> {
    seed_list
        .chunks(2)
        .map(|value| {
            Range::new(
                *value.first().unwrap(),
                (value.first().unwrap() + value.last().unwrap()) - 1, // include
                                                                      // start
            )
        })
        .collect::<Vec<Range>>()
}

pub fn parse_item_to_item(range: &str) -> ItemToItemSpec {
    let binding = range.lines().collect::<Vec<&str>>();
    let first_and_remaining = binding.split_first().unwrap();
    let item_to_item_type = parse_item_to_item_type(first_and_remaining.0);
    let item_to_item_lines = parse_item_to_item_lines(first_and_remaining.1);
    ItemToItemSpec::new(item_to_item_type, item_to_item_lines)
}
