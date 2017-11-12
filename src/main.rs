extern crate sudojo_core;
extern crate sudojo_sudoku_cmd;

use sudojo_sudoku_cmd::cmdwrapper;
use sudojo_core::app::Start;

fn main() {
    println!("Starting sudojo!");
    let mut app_starter = cmdwrapper::AppStarter::new();
    app_starter.start(&None);
}
