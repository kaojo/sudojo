use super::VirtualBoard;
use super::super::game::{Board, Coordinate, Square};
use super::super::util::iterators::{board_iterator, quadrant_squares_iterator};
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
        let mut result: HashSet<(Coordinate, Square)> = HashSet::new();
        // add suggestions where only one virtual value is allowed in a square
        for (x, y) in board_iterator() {
            if let Some(ref p) = self.virtual_board.get_field(&Coordinate::new(x, y)) {
                let possible_values: &HashSet<u8> = p.get_possible_values();
                if !p.is_initial() && possible_values.len() == 1 {
                    let value = possible_values.iter().next().expect("should be there");
                    result.insert((Coordinate::new(x, y), Square::new(*value, false)));
                }
            }
        }
        // add suggestions where values are counted in each row/column/quadrant -> (count == 1) means only allowed here
        for (coord, field) in self.virtual_board.get_data().iter() {
            let x = coord.x;
            let y = coord.y;
            for value in field.get_possible_values().iter() {
                if !field.is_initial() && (self.count_horizontal(value, &y) == 1 || self.count_vertical(value, &x) == 1 || self.count_quarter(value, &x, &y) == 1) {
                    result.insert((Coordinate::new(x, y), Square::new(*value, false)));
                    break;
                }
            }
        }

        result
    }

    fn count_horizontal(&self, value: &u8, y: &u8) -> u8 {
        let mut count: u8 = 0;
        for x in 1..10 {
            let field = self.virtual_board.get_field(&Coordinate::new(x, *y)).expect("Should be there");
            if field.has_possible_value(value) {
                count += 1;
            }
        }
        count
    }

    fn count_vertical(&self, value: &u8, x: &u8) -> u8 {
        let mut count: u8 = 0;
        for y in 1..10 {
            let field = self.virtual_board.get_field(&Coordinate::new(*x, y)).expect("Should be there");
            if field.has_possible_value(value) {
                count += 1;
            }
        }
        count
    }

    fn count_quarter(&self, value: &u8, x: &u8, y: &u8) -> u8 {
        let mut count: u8 = 0;
        let x_quadrant = (*x as f32 / 3 as f32).ceil() as u8;
        let y_quadrant = (*y as f32 / 3 as f32).ceil() as u8;

        for (qx, qy) in quadrant_squares_iterator(x_quadrant, y_quadrant) {
            let field = self.virtual_board.get_field(&Coordinate::new(qx, qy)).expect("Should be there");
            if field.has_possible_value(value) {
                count += 1;
            }
        }
        count
    }
}
