extern crate sudojo_sudoku_core;
extern crate sudojo_core;

use sudojo_sudoku_core::sudoku::generator::{BackTraceGenerator, Generator};
use sudojo_core::app::difficulty::EDifficulty;

fn main() {
    let board = BackTraceGenerator::generate(EDifficulty::Easy);
    println!("{}", board);

}
