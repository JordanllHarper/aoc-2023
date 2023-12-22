#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpaceType {
    Empty,
    Galaxy(i64),
}

impl SpaceType {
    pub fn from_char(c: char, id: i64) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy(id),
            _ => panic!("Invalid token"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Galaxy {
    pub id: i64,
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GalaxyPair {
    pub a: Galaxy,
    pub b: Galaxy,
}
