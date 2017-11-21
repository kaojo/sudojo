#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Coordinate { x: x, y: y }
    }
}
