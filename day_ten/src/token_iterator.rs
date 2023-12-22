use grid::Grid;

use crate::{
    models::{Coordinate, Direction, GridItem, Token},
    solve::{get_coords_to_check, get_start_pipe},
};

pub struct TokenIterator {
    pub grid: Grid<GridItem>,
    pub current_item: GridItem,
    pub start_item: GridItem,
    pub steps_taken: i32,
    pub direction_facing: Direction,
}

impl TokenIterator {
    pub fn new(grid: Grid<GridItem>, start_item: GridItem) -> Self {
        Self {
            grid: grid.clone(),
            current_item: start_item,
            steps_taken: 0,
            direction_facing: Direction::from_joining_pipe(&get_start_pipe(&grid, &start_item))
                .first()
                .unwrap()
                .clone(),
            start_item,
        }
    }
}
impl Iterator for TokenIterator {
    // current_steps   current item
    //            ^     ^
    type Item = (i32, GridItem);

    fn next(&mut self) -> Option<Self::Item> {
        let current_coords = self.current_item.coord;
        let current_direction = self.direction_facing;

        let squares_to_check = get_coords_to_check(current_coords, Some(current_direction));

        for square in squares_to_check {
            if square.1 == self.start_item.coord {
                self.current_item = GridItem::new(Token::Ground, Coordinate::new(-1, -1));
                return Some((self.steps_taken, self.start_item));
            }
            match self.grid.get(
                match square.1.y.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        continue;
                    }
                },
                match square.1.x.try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        continue;
                    }
                },
            ) {
                Some(grid_item) => match grid_item.token.get_new_direction(current_direction) {
                    Some(new_direction) => {
                        self.direction_facing = new_direction;
                        self.current_item = *grid_item;
                        self.steps_taken += 1;
                        return Some((self.steps_taken, self.current_item));
                    }
                    None => continue,
                },
                None => continue,
            };
        }
        None
    }
}
