use crate::models::{Galaxy, GalaxyPair, SpaceType};

pub fn get_positive_difference(a: i64, b: i64) -> i64 {
    let difference = a - b;
    if difference.is_negative() {
        -difference
    } else {
        difference
    }
}
pub fn get_galaxies(current: Vec<Vec<SpaceType>>) -> Vec<Galaxy> {
    let mut result = Vec::new();
    for (y, row) in current.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if let SpaceType::Galaxy(id) = space {
                result.push(Galaxy {
                    id: *id,
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
        }
    }
    result
}
pub fn get_galaxy_pairs(galaxies: Vec<Galaxy>) -> Vec<GalaxyPair> {
    let mut result = Vec::new();
    for (i, galaxy) in galaxies.clone().iter().enumerate() {
        for second_galaxy in galaxies.clone().split_at(i).1 {
            if galaxy != second_galaxy {
                result.push(GalaxyPair {
                    a: *galaxy,
                    b: *second_galaxy,
                });
            }
        }
    }
    result
}
pub fn expand_rows(current: Vec<Vec<SpaceType>>) -> Vec<Vec<SpaceType>> {
    let mut indexes_to_add_at: Vec<Vec<SpaceType>> = Vec::new();

    for (index, row) in current.iter().enumerate() {
        indexes_to_add_at.push(row.clone());
        if row.iter().filter(|each| each != &&SpaceType::Empty).count() == 0 {
            indexes_to_add_at.push(row.clone());
        }
    }

    indexes_to_add_at
}
pub fn get_column_adjustments(current: &Vec<Vec<SpaceType>>) -> Vec<i64> {
    let mut indexes_to_add_at: Vec<i64> = Vec::new();
    let end_column_index = current.first().unwrap().len();
    for i in 0..end_column_index {
        let this_column = current.iter().map(|row| row[i]).collect::<Vec<_>>();
        if this_column
            .iter()
            .filter(|each| each != &&SpaceType::Empty)
            .count()
            == 0
        {
            indexes_to_add_at.push(i.try_into().unwrap());
        }
    }

    indexes_to_add_at
}

pub fn get_row_adjustments(current: &Vec<Vec<SpaceType>>) -> Vec<i64> {
    let mut indexes_to_add_at: Vec<i64> = Vec::new();
    for (i, row) in current.iter().enumerate() {
        if row.iter().filter(|each| each != &&SpaceType::Empty).count() == 0 {
            indexes_to_add_at.push(i.try_into().unwrap());
        }
    }
    indexes_to_add_at
}
pub fn manhattan_distance(a: &Galaxy, b: &Galaxy) -> i64 {
    get_positive_difference(a.x, b.x) + get_positive_difference(a.y, b.y)
}
pub fn adjust_galaxy_positions_pt1(
    galaxies: Vec<Galaxy>,
    column_adjustments: Vec<i64>,
) -> Vec<Galaxy> {
    let mut result = Vec::new();
    for galaxy in galaxies {
        let mut new_galaxy = galaxy.clone();
        for adjustment in column_adjustments.clone() {
            if galaxy.x > adjustment {
                new_galaxy.x += 1;
            }
        }
        result.push(new_galaxy);
    }
    result
}

pub fn adjust_galaxy_positions_pt2(
    galaxies: Vec<Galaxy>,
    column_adjustments: Vec<i64>,
    row_adjustments: Vec<i64>,
    amount_to_expand: i64,
) -> Vec<Galaxy> {
    let mut result = Vec::new();
    for galaxy in galaxies {
        let mut new_galaxy = galaxy.clone();
        for adjustment in column_adjustments.clone() {
            if galaxy.x > adjustment {
                new_galaxy.x += amount_to_expand;
            }
        }

        for adjustment in row_adjustments.clone() {
            if galaxy.y > adjustment {
                new_galaxy.y += amount_to_expand;
            }
        }
        result.push(new_galaxy);
    }
    result
}
