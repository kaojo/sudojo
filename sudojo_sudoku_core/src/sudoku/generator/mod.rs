use sudojo_core::app::difficulty::EDifficulty;
use super::game::{Board, Coordinate, Square, EGameState};
use super::game::rule::{QuadrantUniqueRule, HorizontalUniqueRule, VerticalUniqueRule};
use super::ai::{SolveController, ESolvingIntelligence};
use rand::distributions::{Range, IndependentSample};
use rand::{Rng, thread_rng};
use std::collections::HashSet;

pub mod standalone;

pub trait Generator {
    fn generate(difficulty: EDifficulty) -> Board;
}

pub struct SimpleGenerator {}

impl Generator for SimpleGenerator {
    fn generate(difficulty: EDifficulty) -> Board {
        let mut result: Result<Board, String> = do_generate(&difficulty);
        let mut counter = 1;
        while result.is_err() {
            counter += 1;
            result = do_generate(&difficulty);
        }
        debug!("Found board after {} iterations", counter);

        result.unwrap()
    }
}

pub struct BackTraceGenerator {}

impl Generator for BackTraceGenerator {
    fn generate(_: EDifficulty) -> Board {
        generate_completed_board_backtrace().unwrap()
    }
}

fn do_generate(difficulty: &EDifficulty) -> Result<Board, String> {
    let max_number = get_number_of_guesses(&difficulty);
    debug!("Max number of squares {}", max_number);
    let result = generate_board_rng(max_number)?;

    let solved_board = SolveController::new().solve(&result, &map_difficulty_to_intelligence(difficulty));
    match solved_board.get_state() {
        EGameState::Finished => {
            return Ok(result);
        }
        EGameState::Conflict => return Err(String::from("Board is not solvable.")),
        EGameState::Ok => Err(String::from("Solving algorithm is too bad and can't solve given board")),
    }
}

fn guess_coordinate(mut rng: &mut Rng, range: &Range<u8>) -> Coordinate {
    let x = range.ind_sample(&mut rng);
    let y = range.ind_sample(&mut rng);
    Coordinate::new(x, y)
}

fn get_number_of_guesses(difficulty: &EDifficulty) -> u8 {
    let mut rng = thread_rng();
    let base: u8;
    match difficulty {
        &EDifficulty::Easy => base = 30,
        &EDifficulty::Medium => base = 26,
        &EDifficulty::Hard => base = 24,
        &EDifficulty::VeryHard => base = 20,
    }
    let range = Range::new(0, 2);
    let value = range.ind_sample(&mut rng);
    base + value
}

fn map_difficulty_to_intelligence(difficulty: &EDifficulty) -> ESolvingIntelligence {
    match *difficulty {
        EDifficulty::Easy => ESolvingIntelligence::SimpleLogic,
        EDifficulty::Medium => ESolvingIntelligence::ComplexLogic,
        EDifficulty::Hard => ESolvingIntelligence::ComplexLogic,
        EDifficulty::VeryHard => ESolvingIntelligence::WithGuessing,
    }
}

fn generate_board_rng(max_number: u8) -> Result<Board, String> {
    let mut result: Board = Board::new();
    let mut guessed_coordinates: HashSet<Coordinate> = HashSet::new();
    let range = Range::new(1, 9);
    let mut rng = thread_rng();
    for _ in 0..max_number {
        let mut g_coordinate = guess_coordinate(&mut rng, &range);
        while guessed_coordinates.contains(&g_coordinate) {
            g_coordinate = guess_coordinate(&mut rng, &range)
        }
        guessed_coordinates.insert(g_coordinate);
    }

    for coordinate in guessed_coordinates {
        let mut possible_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        thread_rng().shuffle(&mut possible_values);
        let mut value_found: bool = false;
        let mut tmp = result.clone();
        for value in possible_values.iter() {
            let state = tmp.fill_square(coordinate, Square::new(*value, true))
                .expect("should always be possible to add a value.");
            match state {
                EGameState::Conflict => {
                    tmp.delete_force(&coordinate);
                    continue
                }
                _ => {
                    value_found = true;
                    result = tmp;
                    break;
                }
            }
        }
        if !value_found {
            let message = String::from("No value found.");
            debug!("{}", message);
            return Err(message);
        }
    }
    debug!("Board generated");
    result.initialized(true);
    Ok(result)
}

fn generate_completed_board_backtrace() -> Result<Board, String> {
    let mut board = Board::new();
    do_fill_with_backtrace(&mut board, Coordinate::new(1, 1))?;
    return Ok(board);
}

fn do_fill_with_backtrace(board: &mut Board, coord: Coordinate) -> Result<&mut Board, String> {
    let set: HashSet<u8> = get_shuffled_values(&board, &coord);
    for value in set {
        let state = board
            .fill_square(coord, Square::new(value, true))
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
                    Ok(p) => {}
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

fn get_shuffled_values(board: &Board, coordinate: &Coordinate) -> HashSet<u8> {
    let mut set: HashSet<u8> = HashSet::new();
    let mut possible_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = thread_rng();
    rng.shuffle(&mut possible_values);
    for value in possible_values.iter() {
        set.insert(*value);
    }
    for value in HorizontalUniqueRule::get_forbidden_values(board, coordinate) {
        set.remove(&value);
    }
    for value in VerticalUniqueRule::get_forbidden_values(board, coordinate) {
        set.remove(&value);
    }
    for value in QuadrantUniqueRule::get_forbidden_values(board, coordinate) {
        set.remove(&value);
    }
    set
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naiv_sudoku_generation() {
        let result = generate_board_rng(10);
        assert!(result.is_ok());
        let board = result.unwrap();
        assert!(!board.has_conflicts());
        println!("{}", board)
    }

    #[test]
    fn test_fast_sudoku_generation() {
        let result = generate_completed_board_backtrace();
        assert!(result.is_ok());
        let board = result.unwrap();
        assert!(!board.has_conflicts());
        println!("{}", board)
    }
}
