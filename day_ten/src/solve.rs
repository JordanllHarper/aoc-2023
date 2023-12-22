use grid::Grid;

use crate::{
    models::{Coordinate, Direction, GridItem, Pipe, Token},
    token_iterator::TokenIterator,
};

pub fn get_coords_to_check(
    coord: Coordinate,
    direction_facing: Option<Direction>,
) -> Vec<(Direction, Coordinate)> {
    let x = coord.x;
    let y = coord.y;

    let left = x - 1;
    let right = x + 1;
    let above = y - 1;
    let below = y + 1;

    let north_of = (Direction::North, Coordinate::new(x, above));
    let south_of = (Direction::South, Coordinate::new(x, below));
    let east_of = (Direction::East, Coordinate::new(right, y));
    let west_of = (Direction::West, Coordinate::new(left, y));
    if let Some(v) = direction_facing {
        return vec![match v {
            Direction::North => north_of,
            Direction::East => east_of,
            Direction::South => south_of,
            Direction::West => west_of,
        }];
    }

    vec![north_of, east_of, west_of, south_of]
}
pub fn get_valid_grid_item(
    grid: &Grid<GridItem>,
    item_to_check: (Direction, Coordinate),
) -> Option<(Direction, GridItem)> {
    let coords = item_to_check.1;
    match grid.get(
        match coords.x.try_into() {
            Ok(v) => v,
            Err(_) => return None,
        },
        match coords.y.try_into() {
            Ok(v) => v,
            Err(_) => return None,
        },
    ) {
        Some(v) => Some((item_to_check.0, *v)),
        None => None,
    }
}
pub fn get_start_pipe(grid: &Grid<GridItem>, start_coords: &GridItem) -> Pipe {
    let valid_pipes: Vec<(Direction, Pipe)> = get_coords_to_check(start_coords.coord, None)
        .into_iter()
        .filter_map(|item| get_valid_grid_item(&grid, item))
        .filter_map(|(direction, item)| match item.token {
            Token::Pipe(v) => Some((direction, v)),
            Token::Ground => None,
            Token::Start => None,
        })
        .filter_map(|(direction, pipe)| {
            if Pipe::get_valid_above_start_pipes().contains(&pipe)
                || Pipe::get_valid_below_start_pipes().contains(&pipe)
                || Pipe::get_valid_left_start_pipes().contains(&pipe)
                || Pipe::get_valid_right_start_pipes().contains(&pipe)
            {
                Some((direction, pipe))
            } else {
                None
            }
        })
        .collect();

    let first_pipe_coords = valid_pipes.first().unwrap().0;
    let last_pipe_coords = valid_pipes.last().unwrap().0;
    let joining = Pipe::get_joining(first_pipe_coords, last_pipe_coords);

    joining
}
pub fn get_start(grid: Grid<GridItem>) -> GridItem {
    *grid
        .clone()
        .iter()
        .filter(|each| each.token == Token::Start)
        .collect::<Vec<&GridItem>>()
        .first()
        .unwrap()
        .clone()
}
pub fn get_steps_furthest_away(grid: Grid<GridItem>) -> i32 {
    let start_coords = get_start(grid.clone());
    let grid_iter = TokenIterator::new(
        grid.clone(),
        GridItem::new(Token::Start, start_coords.coord),
    );

    grid_iter.last().unwrap().0 + 1
}
pub fn parse_pipe_shape(grid: Grid<GridItem>) -> Vec<GridItem> {
    let binding = grid.clone();
    let start_coords = binding
        .iter()
        .filter(|each| each.token == Token::Start)
        .collect::<Vec<&GridItem>>();

    let start_coords = start_coords.first().unwrap();
    TokenIterator::new(grid.clone(), **start_coords)
        .map(|each| each.1)
        .collect()
}
pub fn did_intersect_pipe(token: Pipe) -> bool {
    match token {
        Pipe::VPipe => true,
        Pipe::SWBend => true,
        Pipe::SEBend => true,
        _ => false,
    }
}
/// Assumes line is horizontal and in the lower part of the grid square
pub fn is_intersect(token: Token, start_pipe: Pipe) -> bool {
    match token {
        Token::Pipe(v) => did_intersect_pipe(v),
        Token::Ground => false,
        Token::Start => did_intersect_pipe(start_pipe),
    }
}
pub fn get_inner_shape_squares(
    grid: Grid<GridItem>,
    shape: Vec<GridItem>,
    start_pipe: Pipe,
) -> Vec<GridItem> {
    let mut inside_shape = vec![];

    for each_element in grid.iter() {
        if shape.contains(each_element) {
            continue;
        }
        let end_of_row = Coordinate::new(grid.cols().try_into().unwrap(), each_element.coord.y);
        let mut intersections_for_coord = 0;
        for x_coord in each_element.coord.x..=end_of_row.x {
            let current_checked_coord = Coordinate::new(x_coord, each_element.coord.y);
            match grid.get(
                match current_checked_coord.y.try_into() {
                    Ok(v) => v,
                    Err(_) => continue,
                },
                match current_checked_coord.x.try_into() {
                    Ok(v) => v,
                    Err(_) => continue,
                },
            ) {
                Some(item_in_grid) => {
                    if is_intersect(item_in_grid.token, start_pipe) && shape.contains(item_in_grid)
                    {
                        println!("intersecting with {:?}", item_in_grid);
                        intersections_for_coord += 1;
                    }
                }
                None => {}
            };
        }

        if intersections_for_coord % 2 == 0 {
            // even so outside -> skip
            continue;
        } else {
            // odd so inside -> push
            inside_shape.push(*each_element);
        }
    }
    inside_shape
}
