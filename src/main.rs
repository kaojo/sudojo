extern crate sudojo;
extern crate sudojo_core;

use sudojo::cmdwrapper;
use sudojo::sudoku::Sudoku;
use sudojo_core::app::{App};

fn main() {
    println!("Starting sudojo!");
    let sudoku = Sudoku::new();
    let mut app_starter = cmdwrapper::AppStarter::new(sudoku);
    app_starter.start(&None);

}
