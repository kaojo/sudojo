
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Square {
    pub value: u8,
    pub initial: bool,
    pub conflict: bool,
    pub generated: bool,
}

impl Square {
    pub fn new(value: u8, initial: bool) -> Self {
        Square {
            value: value,
            initial: initial,
            conflict: false,
            generated: false,
        }
    }
    pub fn generate(value: u8, initial: bool) -> Self {
        Square {
            value: value,
            initial: initial,
            conflict: false,
            generated: true,
        }
    }
}
