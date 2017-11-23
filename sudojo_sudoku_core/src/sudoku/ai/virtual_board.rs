use std::collections::{HashMap};
use super::super::game::{Coordinate, Board};
use super::super::ai::ESolvingIntelligence;
use super::super::game::rule::{HorizontalUniqueRule, VerticalUniqueRule, QuadrantUniqueRule, RowQuadrantCombinationRule};
use super::Field;
use super::super::util::iterators::{board_iterator, QuadrantSquaresIterator};
use std::fmt;
use ansi_term::Colour::{Cyan, Green};

#[derive(Debug)]
pub struct VirtualBoard {
    data: Vec<Field>,
}

impl VirtualBoard {
    pub fn new(board: &Board, intelligence: &ESolvingIntelligence) -> Self {
        let mut v_board = VirtualBoard {
            data: Vec::new(),
        };

        let mut empty_fields: Vec<Coordinate> = Vec::new();
        for (index, square) in board.get_data().iter().enumerate() {
            let coordinate = Coordinate::from_index(index);
            match square {
                &Some(p) => {
                    // insert already known data
                    v_board.data.push(Field::from_square(&p));
                }
                &None => {
                    // initialize unknown fields in virtual board
                    v_board.data.push(Field::new());
                    empty_fields.push(coordinate);
                }
            }
        }

        let disallowed_values_row: Vec<Vec<u8>> = HorizontalUniqueRule::get_disallowed_values(board);
        let disallowed_values_column: Vec<Vec<u8>> = VerticalUniqueRule::get_disallowed_values(board);
        let disallowed_values_quadrants: Vec<Vec<u8>> = QuadrantUniqueRule::get_disallowed_values(board);

        for coordinate in empty_fields {
            if let Some(ref mut p) = v_board.data.get_mut(coordinate.get_index()) {
                if let Some(ref q) = disallowed_values_row.get(coordinate.y as usize) {
                    for x in q.into_iter() {
                        p.disallow_value(*x);
                    }
                }
                if let Some(ref q) = disallowed_values_column.get(coordinate.x as usize) {
                    for y in q.into_iter() {
                        p.disallow_value(*y);
                    }
                }
                let x_quadrant = (coordinate.x as f32 / 3 as f32).ceil() as u8;
                let y_quadrant = (coordinate.y as f32 / 3 as f32).ceil() as u8;
                let index = (y_quadrant - 1) * 3 + x_quadrant - 1;
                if let Some(ref q) = disallowed_values_quadrants.get(index as usize) {
                    for x in q.into_iter() {
                        p.disallow_value(*x);
                    }
                }
            }
        }
        if intelligence == &ESolvingIntelligence::ComplexLogic {
            VirtualBoard::remove_row_quad_comb_rule(&mut v_board);
        }
        debug!("{}", v_board);
        v_board
    }

    fn remove_row_quad_comb_rule(v_board: &mut VirtualBoard) {
        for (x, y) in board_iterator() {
            let coordinate = Coordinate::new(x, y);
            let field = v_board.data.get(coordinate.get_index()).expect("All fields should be initialized by now.").clone();
            if field.is_initial() {
                continue;
            }
            for value in field.get_possible_values() {
                let exclusive_in_quadrant_horizontally: bool = RowQuadrantCombinationRule::is_exclusive_in_quadrant_horizontally(&v_board, &coordinate, &value);
                if exclusive_in_quadrant_horizontally {
                    for (qx, qy) in QuadrantSquaresIterator::from_board_coordinates(coordinate.x, coordinate.y) {
                        if qy != coordinate.y {
                            v_board.data.get_mut(Coordinate::new(qx, qy).get_index()).expect("all data is initialized").disallow_value(*value);
                        }
                    }
                }
                let exclusive_in_quadrant_vertically: bool = RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&v_board, &coordinate, &value);
                if exclusive_in_quadrant_vertically {
                    for (qx, qy) in QuadrantSquaresIterator::from_board_coordinates(coordinate.x, coordinate.y) {
                        if qx != coordinate.x {
                            v_board.data.get_mut(Coordinate::new(qx, qy).get_index()).expect("all data is initialized").disallow_value(*value);
                        }
                    }
                }
            }
        }
    }

    pub fn from(data: HashMap<Coordinate, Field>) -> Self {
        let mut vec = Vec::new();
        // insert already known data
        for i in 0..81 {
            match data.get(&Coordinate::from_index(i)) {
                None => {
                    vec.push(Field::new());
                }
                Some(p) => {
                    vec.push(p.clone());
                }
            }
        }

        VirtualBoard {
            data: vec
        }
    }

    pub fn get_field(&self, coordinate: &Coordinate) -> Option<&Field> {
        self.data.get(coordinate.get_index())
    }

    pub fn get_data(&self) -> &Vec<Field> {
        &self.data
    }
}

impl fmt::Display for VirtualBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "VirtualBoard");
        for y in 1..10 {
            for i in 1..4 {
                for x in 1..10 {
                    let field = self.data.get(Coordinate::new(x, y).get_index());
                    write!(f, " | ")?;
                    for value in ((i - 1) * 3 + 1)..(i * 3 + 1) {
                        write_single_value(f, field, value);
                    }
                    if [3, 6, 9].contains(&x) {
                        write!(f, " |")?;
                    }
                }
                writeln!(f)?;
            }
            if [3, 6, 9].contains(&y) {
                writeln!(f)?;
            }
        }

        writeln!(f)
    }
}

fn write_single_value(f: &mut fmt::Formatter, field: Option<&Field>, value: u8) {
    let _ = write!(f, "{}", match field {
        None => String::from(" "),
        Some(p) => {
            if p.get_possible_values().contains(&value) {
                if p.is_initial() {
                    Cyan.paint(value.to_string()).to_string()
                } else {
                    Green.paint(value.to_string()).to_string()
                }
            } else {
                String::from(" ")
            }
        }
    });
}
