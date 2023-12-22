#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpringType {
    Known(KnownSpringType),
    Unknown,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KnownSpringType {
    Operational,
    Damaged,
}

impl SpringType {
    pub fn from_char(character: char) -> SpringType {
        match character {
            '.' => SpringType::Known(KnownSpringType::Operational),
            '#' => SpringType::Known(KnownSpringType::Damaged),
            '?' => SpringType::Unknown,
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record {
    pub springs: Vec<SpringType>,
    pub numbers: Vec<i32>,
}

impl Record {
    pub fn new(springs: Vec<SpringType>, numbers: Vec<i32>) -> Self {
        Self { springs, numbers }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KnownSpringSpan {
    pub start: i32,
    pub spring_type: KnownSpringType,
    pub end: i32,
}

impl KnownSpringSpan {
    pub fn new(start: i32, spring_type: KnownSpringType, end: i32) -> Self {
        Self {
            start,
            spring_type,
            end,
        }
    }
    pub fn length(&self) -> i32 {
        (self.end - self.start) + 1
    }
    pub fn is_next_to(&self, other: &Self) -> bool {
        self.end + 1 == other.start || self.start - 1 == other.end
    }
    pub fn matches_num(&self, number: i32) -> bool {
        self.length() == number
    }
}
