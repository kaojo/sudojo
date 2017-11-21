#![feature(test)]
extern crate test;
extern crate sudojo_sudoku_core;
extern crate sudojo_core;


#[cfg(test)]
mod tests {
    use sudojo_sudoku_core::sudoku::generator::{BackTraceGenerator, FastestGenerator};
    use sudojo_sudoku_core::sudoku::game::Board;
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
}