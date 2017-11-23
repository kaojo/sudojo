pub use self::completed_sudoku::{FastestGenerator, BackTraceGenerator};
pub use self::reduce::BacktraceReducer;

use sudojo_core::app::difficulty::EDifficulty;
use super::game::Board;

mod completed_sudoku;
mod reduce;

pub trait Generator {
    fn generate(difficulty: EDifficulty) -> Board;
}

pub struct PuzzleGenerator {}

impl Generator for PuzzleGenerator {
    fn generate(difficulty: EDifficulty) -> Board {
        let mut board: Board = FastestGenerator::generate_board();
        debug!("{}", board);
        BacktraceReducer::reduce_to_puzzle(&mut board, difficulty).expect("should always work");
        debug!("{}", board);
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_generation() {
        let board = PuzzleGenerator::generate(EDifficulty::VeryHard);
        println!("{}", board)
    }
}
