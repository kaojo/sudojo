extern crate sudojo_sudoku_core;
extern crate sudojo_core;

use sudojo_sudoku_core::sudoku::generator::FastestGenerator;
//use sudojo_core::app::difficulty::EDifficulty;

fn main() {
    let board = FastestGenerator::generate_board();
    println!("{}", board);

}
