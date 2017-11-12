use super::VirtualBoard;
use super::super::game::{Board, Coordinate, Square};
use std::collections::HashSet;

pub struct SuggestionController {
    virtual_board: VirtualBoard,
}

impl SuggestionController {
    pub fn new(board: &Board) -> Self {
        SuggestionController {
            virtual_board: VirtualBoard::new(board),
        }
    }

    pub fn get_suggestions(&self) -> HashSet<(Coordinate, Square)> {
        let mut result = HashSet::new();


        result
    }
}