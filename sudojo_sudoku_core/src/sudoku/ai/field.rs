use std::collections::HashSet;
use super::super::game::Square;

#[derive(Debug)]
pub struct Field {
    possible_values: HashSet<u8>,
    initial: bool,
}

impl Field {
    pub fn from_square(square: &Square) -> Self {
        let mut field = Field {
            possible_values: HashSet::new(),
            initial: true,
        };
        field.possible_values.insert(square.value);
        field
    }
    pub fn from_value(value: u8) -> Self {
        let mut field = Field {
            possible_values: HashSet::new(),
            initial: false,
        };
        field.possible_values.insert(value);
        field
    }
}
