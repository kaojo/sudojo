use super::super::game::{Board, Coordinate, Square};
use super::super::ai::{SolveController, ESolvingIntelligence};
use rand::distributions::{Range, IndependentSample};
use sudojo_core::app::difficulty::EDifficulty;
use rand::{Rng, thread_rng};

pub struct BacktraceReducer {}

impl BacktraceReducer {
    pub fn reduce_to_puzzle<'a>(mut board: &'a mut Board, difficulty: EDifficulty) -> Result<&'a mut Board, String> {
        let number_of_pre_filled_guesses = get_number_of_guesses(&difficulty);
        debug!("number_of_pre_filled_guesses: {}", number_of_pre_filled_guesses);
        iterate_remove_square(&mut board, number_of_pre_filled_guesses, &difficulty)?;
        Ok(board)
    }
}


fn iterate_remove_square<'a>(board: &'a mut Board, target_size: u8, difficulty: &EDifficulty) -> Result<&'a mut Board, String> {
    debug!("{}", board.get_size());
    if (board.get_size() as u8) <= target_size {
        return Ok(board);
    }
    for (coordinate, square) in get_coordinates(board) {
        board.delete_force(&coordinate);
        let mut error;

        let sc = SolveController::new(map_difficulty_to_intelligence(difficulty));
        match sc.solve(&board) {
            Err(_) => error = true,
            Ok(_) => match iterate_remove_square(board, target_size, difficulty) {
                Err(_) => error = true,
                Ok(_) => return Ok(board),
            },
        }
        if error {
            debug!("{:?}, {:?}", coordinate, square);
            debug!("{}", board);
            board.fill_square(coordinate.get_index(), square)?;
        }
    }

    Err(String::from("not solvable"))
}

fn get_coordinates(board: &Board) -> Vec<(Coordinate, Square)> {
    let mut rng = thread_rng();
    let mut range = (0..81).collect::<Vec<_>>();

    rng.shuffle(&mut range);

    let mut result = Vec::new();
    for i in range.iter() {
        if let &Some(p) = board.get_data().get(*i).expect("all squares should be filled") {
            result.push((Coordinate::from_index(*i), p));
        }
    }
    result
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


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::FastestGenerator;

    #[test]
    fn test_get_coordinates() {
        let board = FastestGenerator::generate_board();
        let coords = get_coordinates(&board);
        assert_eq!(coords.len(), 81);
    }
}
