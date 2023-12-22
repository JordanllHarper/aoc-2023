#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeToDistance {
    pub time: i64,
    pub best_distance: i64,
}

impl TimeToDistance {
    pub fn new(time: i64, distance: i64) -> Self {
        Self {
            time,
            best_distance: distance,
        }
    }
}
