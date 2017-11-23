extern crate sudojo_sudoku_core;
extern crate sudojo_core;

use sudojo_sudoku_core::sudoku::generator::{PuzzleGenerator, Generator};
use sudojo_core::app::difficulty::EDifficulty;

fn main() {
    let board = PuzzleGenerator::generate(EDifficulty::VeryHard);
    println!("{}", board);

}
