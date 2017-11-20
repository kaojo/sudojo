#![feature(test)]
extern crate test;
extern crate sudojo_sudoku_core;
extern crate sudojo_core;


#[cfg(test)]
mod tests {
    use super::*;
    use sudojo_sudoku_core::sudoku::generator::{BackTraceGenerator, Generator};
    use sudojo_sudoku_core::sudoku::game::Board;
    use sudojo_core::app::difficulty::EDifficulty;
    use test::Bencher;

    #[bench]
    fn test_fast_sudoku_generation(b: &mut Bencher) {
        let mut board = Board::new();
        b.iter(|| {
            board = BackTraceGenerator::generate(EDifficulty::Easy);
        });
        println!("{}", board)
    }
}