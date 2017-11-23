use super::super::game::Square;

#[derive(Debug, Clone)]
pub struct Field {
    possible_values: Vec<u8>,
    initial: bool,
}

impl Field {
    pub fn from_square(square: &Square) -> Self {
        let mut field = Field {
            possible_values: Vec::new(),
            initial: true,
        };
        field.possible_values.push(square.value);
        field
    }
    pub fn new() -> Self {
        let mut field = Field {
            possible_values: Vec::new(),
            initial: false,
        };
        for x in 1..10 {
            field.possible_values.push(x);
        }

        field
    }
    pub fn from_possible_values(values: Vec<u8>) -> Self {
        Field {
            possible_values: values,
            initial: false,
        }
    }

    pub fn get_possible_values(&self) -> &Vec<u8> {
        &self.possible_values
    }

    pub fn disallow_value(&mut self, value: u8) {
        self.possible_values.retain(|v| *v != value)
    }

    pub fn is_initial(&self) -> bool {
        self.initial
    }

    pub fn has_possible_value(&self, value: &u8) -> bool {
        self.possible_values.contains(value)
    }
}
