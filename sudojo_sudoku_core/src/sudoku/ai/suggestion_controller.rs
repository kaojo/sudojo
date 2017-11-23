use super::VirtualBoard;
use super::ESolvingIntelligence;
use super::super::game::{Board, Coordinate, Square};
use super::super::util::iterators::{board_iterator, QuadrantSquaresIterator};

pub struct SuggestionController {
    virtual_board: VirtualBoard,
}

impl SuggestionController {
    pub fn new(board: &Board, intelligence: &ESolvingIntelligence) -> Self {
        SuggestionController {
            virtual_board: VirtualBoard::new(board, intelligence),
        }
    }

    pub fn get_suggestions(&self) -> Vec<(usize, Square)> {
        let mut result: Vec<(usize, Square)> = Vec::new();
        // add suggestions where only one virtual value is allowed in a square
        for i in 0..81 {
            if let Some(ref p) = self.virtual_board.get_field(i) {
                let possible_values: &Vec<u8> = p.get_possible_values();
                if !p.is_initial() && possible_values.len() == 1 {
                    let value = possible_values.iter().next().expect("should be there");
                    result.push((i, Square::generate(*value)));
                }
            }
        }
        // add suggestions where values are counted in each row/column/quadrant -> (count == 1) means only allowed here
        for (index, field) in self.virtual_board.get_data().iter().enumerate() {
            let coordinate = Coordinate::from_index(index);
            let x = coordinate.x;
            let y = coordinate.y;
            for value in field.get_possible_values().iter() {
                if !field.is_initial() && (self.count_horizontal(value, &y) == 1 || self.count_vertical(value, &x) == 1 || self.count_quarter(value, &x, &y) == 1) {
                    result.push((index, Square::generate(*value)));
                    break;
                }
            }
        }

        result
    }

    fn count_horizontal(&self, value: &u8, y: &u8) -> u8 {
        let mut count: u8 = 0;
        for x in 1..10 {
            let field = self.virtual_board.get_field(Coordinate::calc_index(x, *y)).expect("Should be there");
            if field.has_possible_value(value) {
                count += 1;
            }
        }
        count
    }

    fn count_vertical(&self, value: &u8, x: &u8) -> u8 {
        let mut count: u8 = 0;
        for y in 1..10 {
            let field = self.virtual_board.get_field(Coordinate::calc_index(*x, y)).expect("Should be there");
            if field.has_possible_value(value) {
                count += 1;
            }
        }
        count
    }

    fn count_quarter(&self, value: &u8, x: &u8, y: &u8) -> u8 {
        let mut count: u8 = 0;

        for (qx, qy) in QuadrantSquaresIterator::from_board_coordinates(*x, *y) {
            let field = self.virtual_board.get_field(Coordinate::calc_index(qx, qy)).expect("Should be there");
            if field.has_possible_value(value) {
                count += 1;
            }
        }
        count
    }
}
