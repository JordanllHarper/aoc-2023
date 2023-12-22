pub enum Cube {
    Red(i32),
    Green(i32),
    Blue(i32),
}
pub struct CubeSet {
    pub cubes: Vec<Cube>,
}

impl CubeSet {
    pub fn new(cubes: Vec<Cube>) -> Self {
        Self { cubes }
    }
}

pub struct Game {
    pub id: i32,
    pub cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn new(id: i32, cubes: Vec<CubeSet>) -> Self {
        Self {
            id,
            cube_sets: cubes,
        }
    }
}
