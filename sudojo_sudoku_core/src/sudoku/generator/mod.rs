use sudojo_core::app::difficulty::EDifficulty;
use super::game::{Board, Coordinate, Square, EGameState};
use super::ai::SolveController;
use rand::distributions::{Range, IndependentSample};
use rand::{Rng, thread_rng};
use std::collections::HashSet;

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

fn do_generate(difficulty: &EDifficulty) -> Result<Board, String> {
    let mut result: Board = Board::new();

    let mut rng = thread_rng();
    let max_number = get_number_of_guesses(&difficulty, &mut rng);
    let mut counter = 1;
    let mut guessed_coordinates: HashSet<Coordinate> = HashSet::new();
    let range = Range::new(1, 9);
    while counter <= max_number {
        debug!("Guess counter {}", counter);
        counter += 1;
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
            let state = tmp.fill_square(coordinate.clone(), Square::new(*value, true))
                .expect("should allways be possible to add a value.");
            match state {
                EGameState::Conflict => {
                    tmp.delete_force(&coordinate);
                    continue
                }
                EGameState::Finished => return Err(String::from("Error!!! We generated a completed sudoku puzzle. that should not happen.")),
                EGameState::Ok => {
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
    result.initialized(true);

    let solved_board = SolveController::new().solve(&result);
    match solved_board.get_state() {
        EGameState::Finished => {
            return Ok(result);
        }
        EGameState::Conflict => return Err(String::from("Board is not solvable.")),
        EGameState::Ok => panic!("Solving algorithm is bad and can't solve given board"),
    }
}

fn guess_coordinate(mut rng: &mut Rng, range: &Range<u8>) -> Coordinate {
    let x = range.ind_sample(&mut rng);
    let y = range.ind_sample(&mut rng);
    Coordinate::new(x, y)
}

fn guess_value(mut rng: &mut Rng, range: &Range<u8>) -> u8 {
    range.ind_sample(&mut rng)
}

fn get_number_of_guesses(difficulty: &EDifficulty, mut rng: &mut Rng) -> u8 {
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
