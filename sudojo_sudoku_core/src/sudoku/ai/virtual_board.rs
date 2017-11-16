use std::collections::{HashMap, HashSet};
use super::super::game::{Coordinate, Board};
use super::super::game::rule::{HorizontalUniqueRule, VerticalUniqueRule, QuadrantUniqueRule, RowQuadrantCombinationRule};
use super::Field;
use super::super::util::iterators::{board_iterator, QuadrantSquaresIterator};
use std::fmt;
use ansi_term::Colour::{Cyan, Red, Green, Purple};

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

        VirtualBoard::remove_row_quad_comb_rule(&mut v_board);
        println!("{}", v_board);
        v_board
    }

    fn remove_row_quad_comb_rule(v_board: &mut VirtualBoard) {
        for (x, y) in board_iterator() {
            let coord = Coordinate::new(x, y);
            let field = v_board.data.get(&coord).expect("All fields should be initialized by now.").clone();
            if field.is_initial() {
                continue;
            }
            for value in field.get_possible_values() {
                let exclusive_in_quadrant_horizontally: bool = RowQuadrantCombinationRule::is_exclusive_in_quadrant_horizontally(&v_board, &coord, &value);
                if exclusive_in_quadrant_horizontally {
                    for (qx, qy) in QuadrantSquaresIterator::from_board_coordinates(coord.x, coord.y) {
                        if qy != coord.y {
                            let removed = v_board.data.get_mut(&Coordinate::new(qx, qy)).expect("").disallow_value(*value);
                            if removed {
                                println!("{:?}, {}, {}, {}, {}", coord, qx, qy, value, removed);
                            }
                        }
                    }
                }
                let exclusive_in_quadrant_vertically: bool = RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&v_board, &coord, &value);
                if exclusive_in_quadrant_vertically {
                    for (qx, qy) in QuadrantSquaresIterator::from_board_coordinates(coord.x, coord.y) {
                        if qx != coord.x {
                            let removed = v_board.data.get_mut(&Coordinate::new(qx, qy)).expect("").disallow_value(*value);
                            if removed {
                                println!("{:?}, {}, {}, {}, {}", coord, qx, qy, value, removed);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn from(data: HashMap<Coordinate, Field>) -> Self {
        VirtualBoard {
            data
        }
    }

    pub fn get_field(&self, coord: &Coordinate) -> Option<&Field> {
        self.data.get(coord)
    }

    pub fn get_data(&self) -> &HashMap<Coordinate, Field> {
        &self.data
    }
}

impl fmt::Display for VirtualBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 1..10 {
            for i in 1..4 {
                for x in 1..10 {
                    let field = self.data.get(&Coordinate::new(x, y));
                    write!(f, " | ");
                    for value in ((i - 1) * 3 + 1)..(i * 3 + 1) {
                        write_single_value(f, field, value);
                    }
                    if [3, 6, 9].contains(&x) {
                        write!(f, " |");
                    }
                }
                writeln!(f);
            }
            writeln!(f);
        }

        writeln!(f)
    }
}

fn write_single_value(f: &mut fmt::Formatter, field: Option<&Field>, value: u8) {
    write!(f, "{}", match field {
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
