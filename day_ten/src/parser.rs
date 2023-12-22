use grid::Grid;

use crate::models::{Coordinate, GridItem, Token};

pub fn parse_to_tokens(grid: Vec<Vec<char>>) -> Grid<GridItem> {
    let grid_size = grid.first().unwrap().len();
    let vector_grid = grid
        .into_iter()
        .enumerate()
        .map(|(y, each)| {
            each.into_iter()
                .enumerate()
                .map(|(x, character)| {
                    GridItem::new(
                        Token::from_character(character),
                        Coordinate::new(x.try_into().unwrap(), y.try_into().unwrap()),
                    )
                })
                .collect::<Vec<GridItem>>()
        })
        .flatten()
        .collect::<Vec<GridItem>>();
    Grid::from_vec(vector_grid, grid_size)
}
