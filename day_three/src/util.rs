use std::collections::hash_map::Keys;

pub fn create_2d_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum NotableCharacters {
    Number {
        value: usize,
        coordinate: Coordinate,
    },

    Gear(Coordinate),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompleteCharacters {
    Number(Number),
    Gear(Coordinate),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    number: usize,
    x_start: usize,
    x_end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gear {
    coordinate: Coordinate,
}
impl Number {
    pub fn get_coord_range(&self) -> Vec<usize> {
        (self.x_start..=self.x_end)
            .into_iter()
            .collect::<Vec<usize>>()
    }
}

// pub fn parse_numbers_in_line(number_characters: Vec<NotableCharacters>) -> Option<Vec<Number>> {
//     let mut numbers = Vec::new();
//     let mut number_accumulator = String::new();
//     let mut this_start_coord = 0;
//     let mut this_end_coord = 0;
//
//     for character in number_characters {
//         let x = character.coordinate.x;
//
//         // handles pushing logic
//         if x - this_end_coord != 1 {
//             match number_accumulator.parse::<usize>() {
//                 Ok(v) => {
//                     numbers.push(Number {
//                         number: v,
//                         x_start: this_start_coord,
//                         x_end: this_end_coord,
//                     });
//                 }
//                 // Handle the start case with  nothing
//                 Err(_) => {}
//             }
//             number_accumulator = String::new();
//
//             number_accumulator += &character.value.to_string();
//             // reset all to the current x value - this is how much we know at this point
//             this_start_coord = x;
//             this_end_coord = x;
//             continue;
//         }
//
//         number_accumulator += &character.value.to_string();
//         this_end_coord = x;
//     }
//
//     // push final value
//     match number_accumulator.parse::<usize>() {
//         Ok(v) => {
//             numbers.push(Number {
//                 number: v,
//                 x_start: this_start_coord,
//                 x_end: this_end_coord,
//             });
//         }
//         // Handle the start case with  nothing
//         Err(_) => {}
//     }
//     if numbers.is_empty() {
//         None
//     } else {
//         Some(numbers)
//     }
// }

// pub fn get_numbers_in_line(line: &Vec<char>, y_coord: usize) -> Vec<NotableCharacters> {
//     let mut numbers = vec![];
//     for (index, character) in line.into_iter().enumerate() {
//         if character.is_numeric() {
//             numbers.push(NotableCharacters {
//                 value: character.to_digit(10).unwrap().try_into().unwrap(),
//                 coordinate: Coordinate {
//                     x: index.try_into().unwrap(),
//                     y: y_coord,
//                 },
//                 character_type: CharacterType::Number,
//             })
//         }
//     }
//     numbers
// }

pub fn get_notable_characters_in_line(line: &Vec<char>, y_coord: usize) -> Vec<NotableCharacters> {
    let mut notables = Vec::new();
    for (index, character) in line.into_iter().enumerate() {
        if character.is_numeric() {
            notables.push(NotableCharacters::Number {
                value: character.to_digit(10).unwrap().try_into().unwrap(),
                coordinate: Coordinate {
                    x: index.try_into().unwrap(),
                    y: y_coord,
                },
            })
        }

        if character == &'*' {
            notables.push(NotableCharacters::Gear(Coordinate {
                x: index.try_into().unwrap(),
                y: y_coord,
            }))
        }
    }
    notables
}

pub fn parse_complete_characters(
    notable_characters: &Vec<NotableCharacters>,
) -> Vec<CompleteCharacters> {
    let mut completes = Vec::new();
    let mut number_accumulator = String::new();
    let mut this_start_coord = 0;
    let mut this_end_coord = 0;

    for notable in notable_characters {
        match notable {
            NotableCharacters::Number { value, coordinate } => {
                let x = coordinate.x;
                // handles pushing logic
                if x - this_end_coord != 1 {
                    match number_accumulator.parse::<usize>() {
                        Ok(v) => {
                            completes.push(CompleteCharacters::Number(Number {
                                number: v,
                                x_start: this_start_coord,
                                x_end: this_end_coord,
                            }));
                        }
                        // Handle the start case with  nothing
                        Err(_) => {}
                    }
                    number_accumulator = String::new();
                    number_accumulator += &value.to_string();
                    // reset all to the current x value - this is how much we know at this point
                    this_start_coord = x;
                    this_end_coord = x;
                    continue;
                }

                number_accumulator += &value.to_string();
                this_end_coord = x;
            }
            NotableCharacters::Gear(c) => {
                //

                match number_accumulator.parse::<usize>() {
                    Ok(v) => {
                        completes.push(CompleteCharacters::Number(Number {
                            number: v,
                            x_start: this_start_coord,
                            x_end: this_end_coord,
                        }));
                    }
                    // Handle the start case with  nothing
                    Err(_) => {}
                }
                number_accumulator = String::new();
                completes.push(CompleteCharacters::Gear(c.clone()));
            }
        };
    }

    // push final value
    match number_accumulator.parse::<usize>() {
        Ok(v) => {
            completes.push(CompleteCharacters::Number(Number {
                number: v,
                x_start: this_start_coord,
                x_end: this_end_coord,
            }));
        }
        // Handle the start case with  nothing
        Err(_) => {}
    }
    completes
}

pub fn search_around_gears(
    gears: Vec<Coordinate>,
    complete: Vec<Vec<CompleteCharacters>>,
) -> usize {
    println!("Complete: {:?}", complete);

    let mut accum: usize = 0;
    for gear in gears {
        println!("|||Searching around gear|||\n {:?}", gear);
        let gear_index = gear.y;

        let above = match gear.y.checked_sub(1) {
            Some(value) => complete.get(value).cloned(),
            None => None,
        };
        println!("-----------");
        println!("||| Above line |||\n{:?}", above);
        println!("-----------");

        let on_line = complete.get(gear_index).unwrap().clone();

        println!("||| On line |||\n{:?}", on_line);
        println!("-----------");
        let below = match complete.get(gear.y + 1) {
            Some(value) => Some(value),
            None => None,
        };
        println!("||| Below line info |||\n\n{:?}", below);
        println!("-----------");
        let gear_result =
            search_around_singular_gear_and_multiply(above, on_line, below.cloned(), gear);
        accum += gear_result
    }
    accum
}

pub fn is_gear_ratio(gear_scope_coord: Coordinate, number_coord_x: usize) -> bool {
    gear_scope_coord.x == number_coord_x //
}

pub fn find_valid_number_for_gear_scope(
    gear_scope_coord: Coordinate,
    grid_line: &Vec<CompleteCharacters>,
) -> Option<CompleteCharacters> {
    for element in grid_line {
        match element {
            CompleteCharacters::Number(v) => {
                let num_coord_range = v.get_coord_range();
                for coord in num_coord_range {
                    if is_gear_ratio(gear_scope_coord.clone(), coord) {
                        println!("Found valid {:?}", element);
                        return Some(element.clone());
                    }
                }
            }
            CompleteCharacters::Gear(_) => continue,
        }
    }
    return None;
}

pub fn get_gear_scope(gear: Coordinate) -> Vec<Coordinate> {
    let left_of_gear = match gear.x.checked_sub(1) {
        Some(v) => v,
        None => 0,
    };
    let right_of_gear = gear.x + 1;

    let above_gear = match gear.y.checked_sub(1) {
        Some(v) => v,
        None => 0,
    };
    let below_gear = gear.y + 1;
    let horizontal_span: Vec<usize> = vec![left_of_gear, gear.x, right_of_gear];
    let vertical_span: Vec<usize> = vec![above_gear, gear.y, below_gear];
    let mut coords = Vec::new();
    for x in horizontal_span {
        for y in vertical_span.clone() {
            coords.push(Coordinate { x, y })
        }
    }

    coords
}

pub fn search_around_singular_gear_and_multiply(
    above_vector: Option<Vec<CompleteCharacters>>,
    on_line_vector: Vec<CompleteCharacters>,
    below_vector: Option<Vec<CompleteCharacters>>,
    gear: Coordinate,
) -> usize {
    let gear_scope = get_gear_scope(gear);
    let mut valid_above_nums: Vec<CompleteCharacters> = Vec::new();
    let mut valid_on_nums: Vec<CompleteCharacters> = Vec::new();
    let mut valid_below_nums: Vec<CompleteCharacters> = Vec::new();
    for scope_coord in gear_scope {
        println!("---Scanning scope coord---\n--- Coord {:?}", scope_coord);
        match above_vector {
            Some(ref above_vector) => {
                match find_valid_number_for_gear_scope(scope_coord.clone(), above_vector) {
                    Some(v) => valid_above_nums.push(v),
                    None => {}
                }
            }
            None => {}
        };

        match find_valid_number_for_gear_scope(scope_coord.clone(), &on_line_vector) {
            Some(value) => valid_on_nums.push(value),
            None => {}
        }

        match below_vector {
            Some(ref below_vector) => {
                match find_valid_number_for_gear_scope(scope_coord.clone(), below_vector) {
                    Some(value) => valid_below_nums.push(value),
                    None => {}
                }
            }
            None => {}
        };
    }

    valid_above_nums.dedup();
    valid_below_nums.dedup();
    valid_on_nums.dedup();

    let mut elements_to_product = Vec::new();
    valid_above_nums
        .into_iter()
        .for_each(|element| elements_to_product.push(element));
    valid_below_nums
        .into_iter()
        .for_each(|element| elements_to_product.push(element));
    valid_on_nums
        .into_iter()
        .for_each(|element| elements_to_product.push(element));

    if elements_to_product.len() <= 1 {
        return 0;
    }
    elements_to_product
        .into_iter()
        .map(|each| match each {
            CompleteCharacters::Number(v) => v.number,
            CompleteCharacters::Gear(_) => 1, // shouldn't happen - but if it does, multiply by 1 to do nothing
        })
        .fold(1, |accum, value| value.checked_mul(accum).unwrap())
}

pub fn is_symbol(character: &char) -> bool {
    !character.is_numeric() && character != &'.'
}

pub fn get_added_numbers(numbers: Vec<Number>, grid: Vec<Vec<char>>, y_coord: usize) -> usize {
    numbers
        .into_iter()
        .map(|number| search_around_number(number, &grid, y_coord))
        .filter_map(|potential| potential)
        .sum()
}

pub fn search_around_number(
    number: Number,
    grid: &Vec<Vec<char>>,
    y_coord: usize,
) -> Option<usize> {
    let x_start = number.x_start.checked_sub(1);
    let x_start = match x_start {
        Some(no_overflow) => no_overflow,
        None => 0,
    };
    let x_end = number.x_end + 1;
    let x_end = if x_end > grid.first().unwrap().len() - 1 {
        number.x_end
    } else {
        x_end
    };
    let top = y_coord.checked_sub(1);
    let top = match top {
        Some(no_overflow) => no_overflow,
        None => 0,
    };
    let bottom = y_coord + 1;
    let bottom = if bottom > grid.len() - 1 {
        grid.len() - 1
    } else {
        bottom
    };

    for x in x_start..=x_end {
        for y in top..=bottom {
            if is_symbol_at_coord(Coordinate { x, y }, grid) {
                return Some(number.number);
            }
        }
    }

    None
}

pub fn is_symbol_at_coord(coord: Coordinate, grid: &Vec<Vec<char>>) -> bool {
    let x = coord.x;
    let y = coord.y;

    is_symbol(&grid[y][x])
}
