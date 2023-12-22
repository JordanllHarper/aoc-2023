#[derive(Debug)]
pub enum Schematic {
    Number {
        value: i32,
        start_index: usize,
        end_index: usize,
    },
    Symbol {
        position: usize,
    },
}
