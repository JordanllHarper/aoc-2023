use std::vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pipe {
    // Vertical Pipe '|'
    VPipe,
    // Horizontal Pipe '-'
    HPipe,
    // North East Bend 'L'
    NEBend,
    // North West Bend 'J'
    NWBend,
    // South West Bend '7'
    SWBend,
    // South East Bend 'F'
    SEBend,
}

impl Pipe {
    pub fn get_joining(first_direction: Direction, last_direction: Direction) -> Pipe {
        match (first_direction, last_direction) {
            (Direction::North, Direction::East) => Pipe::NEBend,
            (Direction::North, Direction::South) => Pipe::VPipe,
            (Direction::North, Direction::West) => Pipe::NWBend,
            (Direction::East, Direction::North) => Pipe::NEBend,
            (Direction::East, Direction::South) => Pipe::SEBend,
            (Direction::East, Direction::West) => Pipe::HPipe,
            (Direction::South, Direction::North) => Pipe::VPipe,
            (Direction::South, Direction::East) => Pipe::SEBend,
            (Direction::South, Direction::West) => Pipe::SWBend,
            (Direction::West, Direction::North) => Pipe::NWBend,
            (Direction::West, Direction::East) => Pipe::HPipe,
            (Direction::West, Direction::South) => Pipe::SWBend,
            _ => panic!(), // something has gone very wrong here
        }
    }
    pub fn get_valid_above_start_pipes() -> Vec<Pipe> {
        vec![Pipe::VPipe, Pipe::SWBend, Pipe::SEBend]
    }
    pub fn get_valid_below_start_pipes() -> Vec<Pipe> {
        vec![Pipe::VPipe, Pipe::NWBend, Pipe::NEBend]
    }

    pub fn get_valid_left_start_pipes() -> Vec<Pipe> {
        vec![Pipe::HPipe, Pipe::SWBend, Pipe::NWBend]
    }

    pub fn get_valid_right_start_pipes() -> Vec<Pipe> {
        vec![Pipe::HPipe, Pipe::NEBend, Pipe::SEBend]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Pipe(Pipe),
    Ground, // .
    Start,  // S
}
pub struct CoordinatePair {
    start: Coordinate,
    end: Coordinate,
}
impl CoordinatePair {
    /// NOTE: Coords here will draw straight line from start x to end x, using y at start
    pub fn horiz_coords_from_pair(&self) -> Vec<Coordinate> {
        (self.start.x..=self.end.x)
            .into_iter()
            .map(|each| Coordinate::new(each, self.start.y)) // maintain y coords
            .collect()
    }

    pub fn vert_coords_from_pair(&self) -> Vec<Coordinate> {
        (self.start.y..=self.end.y)
            .into_iter()
            .map(|each| Coordinate::new(self.start.x, each)) // maintain y coords
            .collect()
    }
}

impl CoordinatePair {
    pub fn new(start: Coordinate, end: Coordinate) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridItem {
    pub token: Token,
    pub coord: Coordinate,
}

impl GridItem {
    pub fn new(token: Token, coord: Coordinate) -> Self {
        Self { token, coord }
    }
}
impl Token {
    pub fn from_character(character: char) -> Self {
        match character {
            '|' => Self::Pipe(Pipe::VPipe),
            '-' => Self::Pipe(Pipe::HPipe),
            'L' => Self::Pipe(Pipe::NEBend),
            'J' => Self::Pipe(Pipe::NWBend),
            '7' => Self::Pipe(Pipe::SWBend),
            'F' => Self::Pipe(Pipe::SEBend),
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!(), // this should never happen
        }
    }

    pub fn to_nice_characters(&self) -> char {
        match self {
            Token::Ground => ' ',
            Token::Start => 'S',
            Token::Pipe(v) => match v {
                Pipe::VPipe => '│',
                Pipe::HPipe => '─',
                Pipe::NEBend => '└',
                Pipe::NWBend => '┘',
                Pipe::SWBend => '┐',
                Pipe::SEBend => '┌',
            },
        }
    }

    // Some(direction) if you can get a new direction from it, otherwise None
    pub fn get_new_direction(&self, current_direction: Direction) -> Option<Direction> {
        match self {
            crate::models::Token::Ground => None,
            Token::Pipe(p) => match p {
                Pipe::VPipe => match current_direction {
                    Direction::North => Some(Direction::North),
                    Direction::South => Some(Direction::South),
                    _ => None,
                },
                Pipe::HPipe => match current_direction {
                    Direction::East => Some(Direction::East),
                    Direction::West => Some(Direction::West),
                    _ => None,
                },
                Pipe::NEBend => match current_direction {
                    Direction::South => Some(Direction::East),
                    Direction::West => Some(Direction::North),
                    _ => None,
                },
                Pipe::NWBend => match current_direction {
                    Direction::South => Some(Direction::West),
                    Direction::East => Some(Direction::North),
                    _ => None,
                },
                Pipe::SWBend => match current_direction {
                    Direction::North => Some(Direction::West),
                    Direction::East => Some(Direction::South),
                    _ => None,
                },
                Pipe::SEBend => match current_direction {
                    Direction::North => Some(Direction::East),
                    Direction::West => Some(Direction::South),
                    _ => None,
                },
            },
            Token::Start => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn from_joining_pipe(pipe: &Pipe) -> Vec<Direction> {
        match pipe {
            Pipe::VPipe => vec![Direction::North, Direction::South],
            Pipe::HPipe => vec![Direction::East, Direction::West],
            Pipe::NEBend => vec![Direction::North, Direction::East],
            Pipe::NWBend => vec![Direction::North, Direction::West],
            Pipe::SWBend => vec![Direction::South, Direction::West],
            Pipe::SEBend => vec![Direction::South, Direction::East],
        }
    }
    pub fn listed() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }
}
