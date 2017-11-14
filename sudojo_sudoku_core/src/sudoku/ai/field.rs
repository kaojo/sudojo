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
    pub fn new() -> Self {
        let mut field = Field {
            possible_values: HashSet::new(),
            initial: false,
        };
        for x in 1..10 {
            field.possible_values.insert(x);
        }

        field
    }

    pub fn get_possible_values(&self) -> &HashSet<u8> {
        &self.possible_values
    }

    pub fn disallow_value(&mut self, value: u8) {
        self.possible_values.remove(&value);
    }

    pub fn is_initial(&self) -> bool {
        self.initial
    }

    pub fn has_possible_value(&self, value: &u8) -> bool {
        self.possible_values.contains(value)
    }
}
