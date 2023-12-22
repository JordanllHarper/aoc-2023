use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}
impl Direction {
    pub fn from_character(character: char) -> Direction {
        match character {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(), // this should never happen
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub directions: Vec<Direction>,
    pub nodes: HashMap<String, Destination>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Destination {
    pub left: String,
    pub right: String,
}

impl Destination {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
}

impl Map {
    pub fn new(directions: Vec<Direction>, nodes: HashMap<String, Destination>) -> Self {
        Self { directions, nodes }
    }
}
