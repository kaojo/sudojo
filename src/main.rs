extern crate sudojo;

use sudojo::cmdwrapper::{self, App};
use sudojo::sudoku::Sudoku;

fn main() {
    println!("Starting sudojo!");
    let sudoku = Sudoku::new();
    let mut app_starter = cmdwrapper::AppStarter::new(sudoku);
    app_starter.start(&None);

}
