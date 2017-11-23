use super::{Coordinate, Board, EGameState};
use super::super::util::iterators::{quadrant_iterator, QuadrantSquaresIterator};
use super::super::ai::VirtualBoard;
use std::collections::HashMap;

pub struct HorizontalUniqueRule {}

impl HorizontalUniqueRule {
    pub fn apply(coordinate: &Coordinate, board: &Board) -> EGameState {
        let value: u8;
        match board.get_square(coordinate) {
            &None => return EGameState::Ok,
            &Some(p) => value = p.value,
        }
        for x in 1..10 {
            if x != coordinate.x {
                let compare_square = board.get_square(&Coordinate::new(x, coordinate.y));
                if compare_square.is_some() {
                    if value == compare_square.unwrap().value {
                        return EGameState::Conflict;
                    }
                }
            }
        }
        EGameState::Ok
    }

    pub fn get_disallowed_values(board: &Board) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::new();
        for y in 1..10 {
            let mut row_result: Vec<u8> = Vec::new();
            for x in 1..10 {
                if let &Some(ref p) = board.get_square(&Coordinate::new(x, y)) {
                    row_result.push(p.value);
                }
            }
            result.push(row_result);
        }
        result
    }

    pub fn get_forbidden_values(board: &Board, coordinate: &Coordinate) -> Vec<u8> {
        let mut row_result = Vec::new();
        for x in 1..10 {
            if let &Some(ref p) = board.get_square(&Coordinate::new(x, coordinate.y)) {
                row_result.push(p.value);
            }
        }
        row_result
    }
}

pub struct VerticalUniqueRule {}

impl VerticalUniqueRule {
    pub fn apply(coordinate: &Coordinate, board: &Board) -> EGameState {
        let value: u8;
        match board.get_square(coordinate) {
            &None => return EGameState::Ok,
            &Some(p) => value = p.value,
        }
        for y in 1..10 {
            if y != coordinate.y {
                let compare_square = board.get_square(&Coordinate::new(coordinate.x, y));
                if compare_square.is_some() {
                    if value == compare_square.unwrap().value {
                        return EGameState::Conflict;
                    }
                }
            }
        }
        EGameState::Ok
    }

    pub fn get_disallowed_values(board: &Board) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::new();
        for x in 1..10 {
            let mut column_result: Vec<u8> = Vec::new();
            for y in 1..10 {
                if let &Some(ref p) = board.get_square(&Coordinate::new(x, y)) {
                    column_result.push(p.value);
                }
            }
            result.push(column_result);
        }
        result
    }

    pub fn get_forbidden_values(board: &Board, coordinate: &Coordinate) -> Vec<u8> {
        let mut column_result = Vec::new();
        for y in 1..10 {
            if let &Some(ref p) = board.get_square(&Coordinate::new(coordinate.x, y)) {
                column_result.push(p.value);
            }
        }
        column_result
    }
}

pub struct QuadrantUniqueRule {}

impl QuadrantUniqueRule {
    pub fn apply(coordinate: &Coordinate, board: &Board) -> EGameState {
        let value: u8;
        match board.get_square(coordinate) {
            &None => return EGameState::Ok,
            &Some(p) => value = p.value,
        }
        let x_quadrant = calculate_quadrant(coordinate.x);
        let y_quadrant = calculate_quadrant(coordinate.y);
        for y in (y_quadrant * 3 - 2)..(y_quadrant * 3 + 1) {
            for x in (x_quadrant * 3 - 2)..(x_quadrant * 3 + 1) {
                if y != coordinate.y && x != coordinate.x {
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

    pub fn get_disallowed_values(board: &Board) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::new();
        for iterator in quadrant_iterator() {
            let mut quadrant_result: Vec<u8> = Vec::new();
            let q_x = iterator.q_x;
            let q_y = iterator.q_y;
            for (x, y) in iterator {
                if let &Some(ref p) = board.get_square(&Coordinate::new(x, y)) {
                    quadrant_result.push(p.value);
                }
            }
            result.push(quadrant_result);
        }
        result
    }

    pub fn get_forbidden_values(board: &Board, coordinate: &Coordinate) -> Vec<u8> {
        let mut quadrant_result: Vec<u8> = Vec::new();
        for (_, y) in QuadrantSquaresIterator::from_board_coordinates(coordinate.x, coordinate.y) {
            if let &Some(ref p) = board.get_square(&Coordinate::new(coordinate.x, y)) {
                quadrant_result.push(p.value);
            }
        }
        quadrant_result
    }
}

pub struct RowQuadrantCombinationRule {}

impl RowQuadrantCombinationRule {
    pub fn is_exclusive_in_quadrant_horizontally(v_board: &VirtualBoard, coordinate: &Coordinate, value: &u8) -> bool {
        for x in 1..10 {
            let x_quadrant = calculate_quadrant(coordinate.x);
            //not in same quadrant
            if !(((x_quadrant - 1) * 3 + 1) <= x && x <= (x_quadrant * 3)) {
                if let Some(ref p) = v_board.get_field(&Coordinate::new(x, coordinate.y)) {
                    if p.get_possible_values().contains(value) {
                        return false;
                    }
                }
            }
        }
        true
    }
    pub fn is_exclusive_in_quadrant_vertically(v_board: &VirtualBoard, coordinate: &Coordinate, value: &u8) -> bool {
        for y in 1..10 {
            let y_quadrant = calculate_quadrant(coordinate.y);
            //not in same quadrant
            if !(((y_quadrant - 1) * 3 + 1) <= y && y <= (y_quadrant * 3)) {
                if let Some(ref p) = v_board.get_field(&Coordinate::new(coordinate.x, y)) {
                    if p.get_possible_values().contains(value) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn calculate_quadrant(x: u8) -> u8 {
    return (x as f32 / 3 as f32).ceil() as u8;
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::ai::Field;
    use super::super::super::util::iterators::board_iterator;

    #[test]
    fn test_calculate_quadrant() {
        assert_eq!(1, calculate_quadrant(1));
        assert_eq!(1, calculate_quadrant(2));
        assert_eq!(1, calculate_quadrant(3));
        assert_eq!(2, calculate_quadrant(4));
        assert_eq!(2, calculate_quadrant(5));
        assert_eq!(2, calculate_quadrant(6));
        assert_eq!(3, calculate_quadrant(7));
        assert_eq!(3, calculate_quadrant(8));
        assert_eq!(3, calculate_quadrant(9));
    }

    #[test]
    fn is_exclusive_in_quadrant_horizontally() {
        let mut map: HashMap<Coordinate, Field> = HashMap::new();
        let mut set: Vec<u8> = Vec::new();
        set.push(1);
        //is unique in quadrant
        map.insert(Coordinate::new(1, 1), Field::from_possible_values(set.clone()));
        // following values should be removed
        map.insert(Coordinate::new(1, 2), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(2, 2), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(5, 2), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(9, 2), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(3, 3), Field::from_possible_values(set.clone()));
        // is not unique in quadrant
        set.clear();
        map.insert(Coordinate::new(2, 1), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(3, 1), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(4, 1), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(5, 1), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(6, 1), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(7, 1), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(8, 1), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(9, 1), Field::from_possible_values(set.clone()));

        let board = VirtualBoard::from(map);
        let coordinate_1_1 = Coordinate::new(1, 1);
        assert!(RowQuadrantCombinationRule::is_exclusive_in_quadrant_horizontally(&board, &coordinate_1_1, &1));
        let coordinate_1_2 = Coordinate::new(1, 2);
        assert!(!RowQuadrantCombinationRule::is_exclusive_in_quadrant_horizontally(&board, &coordinate_1_2, &1));
        let coordinate_4_1 = Coordinate::new(4, 1);
        assert!(!RowQuadrantCombinationRule::is_exclusive_in_quadrant_horizontally(&board, &coordinate_4_1, &1));
        let coordinate_4_2 = Coordinate::new(4, 2);
        assert!(!RowQuadrantCombinationRule::is_exclusive_in_quadrant_horizontally(&board, &coordinate_4_2, &1));
    }

    #[test]
    fn is_exclusive_in_quadrant_vertically() {
        let mut map: HashMap<Coordinate, Field> = HashMap::new();
        let mut set: Vec<u8> = Vec::new();
        set.push(9);

        for (x, y) in board_iterator() {
            map.insert(Coordinate::new(x, y), Field::from_possible_values(set.clone()));
        }
        set.clear();
        set.push(1);
        set.push(2);
        map.insert(Coordinate::new(6, 1), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(6, 2), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(6, 3), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(6, 7), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(6, 8), Field::from_possible_values(set.clone()));
        map.insert(Coordinate::new(6, 9), Field::from_possible_values(set.clone()));

        let board = VirtualBoard::from(map);
        assert!(!RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&board, &Coordinate::new(6, 1), &9));
        assert!(!RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&board, &Coordinate::new(6, 2), &9));
        assert!(!RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&board, &Coordinate::new(6, 3), &9));
        assert!(RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&board, &Coordinate::new(6, 4), &9));
        assert!(RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&board, &Coordinate::new(6, 5), &9));
        assert!(RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&board, &Coordinate::new(6, 6), &9));
        assert!(!RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&board, &Coordinate::new(6, 7), &9));
        assert!(!RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&board, &Coordinate::new(6, 8), &9));
        assert!(!RowQuadrantCombinationRule::is_exclusive_in_quadrant_vertically(&board, &Coordinate::new(6, 9), &9));
    }
}
