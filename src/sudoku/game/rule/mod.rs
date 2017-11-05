use super::{Coordinate, Square, Board, EGameState};

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
}

pub struct QuadrantUniqueRule {}

impl QuadrantUniqueRule {
    pub fn apply(coord: &Coordinate, square: &Square, board: &Board) -> EGameState {
        let value = square.value;
        let x_quadrant = (coord.x as f32 / 3 as f32).ceil() as u8;
        let y_quadrant = (coord.y as f32 / 3 as f32).ceil() as u8;
        for y in (y_quadrant * 3 - 2)..(y_quadrant * 3 + 1) {
            for x in (x_quadrant * 3 - 2)..(x_quadrant * 3 + 1) {
                println!("Q-Rule {},{}", x, y);
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
}