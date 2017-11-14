use super::{Coordinate, Square, Board, EGameState};
use super::super::util::iterators::quadrant_iterator;
use std::collections::{HashMap, HashSet};

pub struct HorizontalUniqueRule {}

impl HorizontalUniqueRule {
    pub fn apply(coord: &Coordinate, square: &Square, board: &Board) -> EGameState {
        let value = square.value;
        for x in 1..10 {
            if x != coord.x {
                let compare_square = board.get_square(&Coordinate::new(x, coord.y));
                if compare_square.is_some() {
                    if value == compare_square.unwrap().value {
                        return EGameState::Conflict;
                    }
                }
            }
        }
        EGameState::Ok
    }

    pub fn get_disallowed_values(board: &Board) -> HashMap<u8, HashSet<u8>> {
        let mut result: HashMap<u8, HashSet<u8>> = HashMap::new();
        for y in 1..10 {
            let mut row_result: HashSet<u8> = HashSet::new();
            for x in 1..10 {
                if let Some(ref p) = board.get_square(&Coordinate::new(x, y)) {
                    row_result.insert(p.value);
                }
            }
            result.insert(y, row_result);
        }
        result
    }
}

pub struct VerticalUniqueRule {}

impl VerticalUniqueRule {
    pub fn apply(coord: &Coordinate, square: &Square, board: &Board) -> EGameState {
        let value = square.value;
        for y in 1..10 {
            if y != coord.y {
                let compare_square = board.get_square(&Coordinate::new(coord.x, y));
                if compare_square.is_some() {
                    if value == compare_square.unwrap().value {
                        return EGameState::Conflict;
                    }
                }
            }
        }
        EGameState::Ok
    }

    pub fn get_disallowed_values(board: &Board) -> HashMap<u8, HashSet<u8>> {
        let mut result: HashMap<u8, HashSet<u8>> = HashMap::new();
        for x in 1..10 {
            let mut column_result: HashSet<u8> = HashSet::new();
            for y in 1..10 {
                if let Some(ref p) = board.get_square(&Coordinate::new(x, y)) {
                    column_result.insert(p.value);
                }
            }
            result.insert(x, column_result);
        }
        result
    }
}

pub struct QuadrantUniqueRule {}

impl QuadrantUniqueRule {
    pub fn apply(coord: &Coordinate, square: &Square, board: &Board) -> EGameState {
        let value = square.value;
        let x_quadrant = (coord.x as f32 / 3 as f32).ceil() as u8;
        let y_quadrant = (coord.y as f32 / 3 as f32).ceil() as u8;
        for y in (y_quadrant * 3 - 2)..(y_quadrant * 3 + 1) {
            for x in (x_quadrant * 3 - 2)..(x_quadrant * 3 + 1) {
                if y != coord.y && x != coord.x {
                    let compare_square = board.get_square(&Coordinate::new(x, y));
                    if compare_square.is_some() {
                        if value == compare_square.unwrap().value {
                            return EGameState::Conflict;
                        }
                    }
                }
            }
        }
        EGameState::Ok
    }

    pub fn get_disallowed_values(board: &Board) -> HashMap<Coordinate, HashSet<u8>> {
        let mut result: HashMap<Coordinate, HashSet<u8>> = HashMap::new();
        for iterator in quadrant_iterator() {
            let mut quadrant_result: HashSet<u8> = HashSet::new();
            let q_x = iterator.q_x;
            let q_y = iterator.q_y;
            for (x, y) in iterator {
                if let Some(ref p) = board.get_square(&Coordinate::new(x, y)) {
                    quadrant_result.insert(p.value);
                }
            }
            result.insert(Coordinate::new(q_x, q_y), quadrant_result);
        }
        result
    }
}