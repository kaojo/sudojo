#![feature(test)]
extern crate test;
extern crate sudojo_sudoku_core;
extern crate sudojo_core;


#[cfg(test)]
mod tests {
    use sudojo_sudoku_core::sudoku::generator::{Generator, PuzzleGenerator, BackTraceGenerator, FastestGenerator, BacktraceReducer};
    use sudojo_sudoku_core::sudoku::game::Board;
    use sudojo_core::app::difficulty::EDifficulty;
    use test::Bencher;

    #[bench]
    fn test_super_fast_sudoku_generation(b: &mut Bencher) {
        let mut board = Board::new();
        b.iter(|| {
            board = FastestGenerator::generate_board();
        });
        println!("{}", board)
    }

    #[bench]
    fn test_super_old_sudoku_generation(b: &mut Bencher) {
        let mut board = Board::new();
        b.iter(|| {
            board = BackTraceGenerator::generate_board();
        });
        println!("{}", board)
    }

    #[bench]
    fn test_complete_puzzle_gen_easy(b: &mut Bencher) {
        let mut board = Board::new();
        b.iter(|| {
            board = PuzzleGenerator::generate(EDifficulty::Easy);
        });
        println!("{}", board)
    }

    #[bench]
    fn test_complete_puzzle_gen_very_hard(b: &mut Bencher) {
        let mut board = Board::new();
        b.iter(|| {
            board = PuzzleGenerator::generate(EDifficulty::VeryHard);
        });
        println!("{}", board)
    }

    #[bench]
    fn test_reduce_to_puzzle_easy(b: &mut Bencher) {
        let mut board = FastestGenerator::generate_board();
        let mut counter = 0;
        b.iter(|| {
            counter += 1;
            BacktraceReducer::reduce_to_puzzle(&mut board, EDifficulty::Easy);
        });
        println!("{}, {}", counter, board)
    }

    #[bench]
    fn test_reduce_to_puzzle_hard(b: &mut Bencher) {
        let mut board: Board = FastestGenerator::generate_board();
        let mut counter = 0;
        b.iter(|| {
            BacktraceReducer::reduce_to_puzzle(&mut board, EDifficulty::Hard);
        });
        let mut counter = 0;
        println!("{}, {}", counter, board)
    }
}