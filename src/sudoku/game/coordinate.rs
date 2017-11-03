#[derive(Clone, Debug, Eq, Hash)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Coordinate { x: x, y: y }
    }
}
