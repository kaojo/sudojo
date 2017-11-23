use rand::{thread_rng, Rng};
use super::super::game::{Board, Coordinate, Square, EGameState};
use super::super::game::rule::{HorizontalUniqueRule, VerticalUniqueRule, QuadrantUniqueRule};

pub struct FastestGenerator {}

impl FastestGenerator {
    pub fn generate_board() -> Board {
        let mut init = init_new_data();
        let vec = iterate_add(&mut init).unwrap();

        Board::from_vec(map_to_board_data(vec))
    }
}

pub struct BackTraceGenerator {}

impl BackTraceGenerator {
    pub fn generate_board() -> Board {
        generate_completed_board_backtrace().unwrap()
    }
}

fn generate_completed_board_backtrace() -> Result<Board, String> {
    let mut board = Board::new();
    do_fill_with_backtrace(&mut board, Coordinate::new(1, 1))?;
    return Ok(board);
}

fn do_fill_with_backtrace(board: &mut Board, coord: Coordinate) -> Result<&mut Board, String> {
    let set: Vec<u8> = get_shuffled_values(&board, &coord);
    for value in set {
        let state = board
            .fill_square(coord.get_index(), Square::new(value, true))
            .expect("Should allways return ok.");

        match state {
            EGameState::Finished => return Ok(board),
            EGameState::Conflict => board.delete_force(&coord),
            EGameState::Ok => {
                let c = get_next_coordinate(&coord);
                if c.is_none() {
                    return Ok(board);
                }
                let mut error = false;
                match do_fill_with_backtrace(board, c.unwrap()) {
                    Err(_) => {
                        error = true;
                    }
                    Ok(_) => {}
                }
                if error {
                    board.delete_force(&coord);
                    continue;
                }
                return Ok(board);
            }
        }
    }
    return Err(String::from("No suitable value found for field"));
}

fn get_shuffled_values(board: &Board, coordinate: &Coordinate) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = thread_rng();

    let mut forbidden_values = HorizontalUniqueRule::get_forbidden_values(board, coordinate);
    for value in VerticalUniqueRule::get_forbidden_values(board, coordinate) {
        forbidden_values.push(value);
    }
    for value in QuadrantUniqueRule::get_forbidden_values(board, coordinate) {
        forbidden_values.push(value);
    }

    let filter = |x: &mut u8| {
        forbidden_values.contains(x)
    };
    let mut i = 0;
    while i != vec.len() {
        if filter(&mut vec[i]) {
            vec.remove(i);
        } else {
            i += 1;
        }
    }

    rng.shuffle(&mut vec);
    vec
}

fn get_next_coordinate(coord: &Coordinate) -> Option<Coordinate> {
    let x = coord.x;
    let y = coord.y;

    if x < 9 {
        return Some(Coordinate::new(x + 1, y));
    } else if y < 9 {
        return Some(Coordinate::new(1, y + 1));
    }
    None
}

fn init_new_data() -> Vec<u8> {
    let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = thread_rng();
    rng.shuffle(&mut vec);
    vec
}

fn map_to_board_data(vec: &Vec<u8>) -> Vec<Option<Square>> {
    vec.iter().map(|value| Some(Square::new(*value, true))).collect::<Vec<_>>()
}

fn is_insert_allowed(vec: &Vec<u8>, value: &u8) -> bool {
    //row
    let len = vec.len();
    for i in (len - len % 9)..len {
        if vec.get(i).unwrap() == value {
            return false;
        }
    }
    let mut i = len % 9;
    while i < len {
        if vec.get(i).unwrap() == value {
            return false;
        }
        i = i + 9;
    }
    i = (len / 9 - len / 9 % 3) * 9 + len % 9 - len % 9 % 3;
    for z in 0..3 {
        for index in (i + z * 9)..((i + z * 9) + 3) {
            if index < len && vec.get(index).unwrap() == value {
                return false;
            }
        }
    }
    true
}

fn get_allowed_values(board: &Vec<u8>) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = thread_rng();

    let filter = |x: &u8| {
        !is_insert_allowed(board, x)
    };
    let mut i = 0;
    while i != vec.len() {
        if filter(&mut vec[i]) {
            vec.remove(i);
        } else {
            i += 1;
        }
    }

    rng.shuffle(&mut vec);
    vec
}

fn iterate_add(vec: &mut Vec<u8>) -> Result<&mut Vec<u8>, String> {
    let set: Vec<u8> = get_allowed_values(&vec);
    for value in set.into_iter() {
        vec.push(value);
        if vec.len() == 81 {
            return Ok(vec);
        }
        let mut error = false;

        match iterate_add(vec) {
            Err(_) => {
                error = true;
            }
            Ok(_) => ()
        }
        if error {
            vec.pop();
            continue;
        }
        return Ok(vec);
    }
    Err(String::from("No values found."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let board = FastestGenerator::generate_board();
        assert!(board.is_filled());
        assert!(!board.has_conflicts());
        assert_ne!(board, FastestGenerator::generate_board())
    }


    #[test]
    fn test_fast_sudoku_generation() {
        let board = BackTraceGenerator::generate_board();
        assert!(board.is_filled());
        assert!(!board.has_conflicts());
        assert_ne!(board, FastestGenerator::generate_board())
    }
}