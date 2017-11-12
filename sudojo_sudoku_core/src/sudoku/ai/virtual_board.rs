use std::collections::HashMap;
use super::super::game::{Coordinate, Board};
use super::Field;

#[derive(Debug)]
pub struct VirtualBoard {
    data: HashMap<Coordinate, Field>,
}

impl VirtualBoard {
    pub fn new(board: &Board) -> Self {
        let mut v_board = VirtualBoard {
            data: HashMap::new(),
        };

        for (coord, square) in board.get_data() {
            v_board.data.insert(coord.clone(), Field::from_square(square));
        }

        v_board
    }
}