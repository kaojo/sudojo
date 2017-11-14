use std::collections::{HashMap, HashSet};
use super::super::game::{Coordinate, Board};
use super::super::game::rule::{HorizontalUniqueRule, VerticalUniqueRule, QuadrantUniqueRule};
use super::Field;
use super::super::util::iterators::board_iterator;

#[derive(Debug)]
pub struct VirtualBoard {
    data: HashMap<Coordinate, Field>,
}

impl VirtualBoard {
    pub fn new(board: &Board) -> Self {
        let mut v_board = VirtualBoard {
            data: HashMap::new(),
        };

        // insert already known data
        for (coord, square) in board.get_data() {
            v_board.data.insert(coord.clone(), Field::from_square(square));
        }

        // initialize unknown fields in virtual board
        let mut empty_fields: HashSet<Coordinate> = HashSet::new();
        for (x, y) in board_iterator() {
            match v_board.data.get(&Coordinate::new(x, y)) {
                Some(_) => (),
                None => {
                    empty_fields.insert(Coordinate::new(x, y));
                    v_board.data.insert(Coordinate::new(x, y), Field::new());
                    ()
                }
            }
        }
        let disallowed_values_row: HashMap<u8, HashSet<u8>> = HorizontalUniqueRule::get_disallowed_values(board);
        let disallowed_values_column: HashMap<u8, HashSet<u8>> = VerticalUniqueRule::get_disallowed_values(board);
        let disallowed_values_quadrants: HashMap<Coordinate, HashSet<u8>> = QuadrantUniqueRule::get_disallowed_values(board);

        for coord in empty_fields {
            if let Some(ref mut p) = v_board.data.get_mut(&coord) {
                if let Some(ref q) = disallowed_values_row.get(&(coord.y)) {
                    for x in q.into_iter() {
                        p.disallow_value(*x);
                    }
                }
                if let Some(ref q) = disallowed_values_column.get(&(coord.x)) {
                    for y in q.into_iter() {
                        p.disallow_value(*y);
                    }
                }
                let x_quadrant = (coord.x as f32 / 3 as f32).ceil() as u8;
                let y_quadrant = (coord.y as f32 / 3 as f32).ceil() as u8;
                if let Some(ref q) = disallowed_values_quadrants.get(&Coordinate::new(x_quadrant, y_quadrant)) {
                    for x in q.into_iter() {
                        p.disallow_value(*x);
                    }
                }
            }
        }

        v_board
    }
}