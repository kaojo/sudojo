extern crate sudojo_core;
extern crate sudojo_sudoku_cmd;
extern crate log4rs;
#[macro_use]
extern crate log;

use sudojo_sudoku_cmd::cmdwrapper;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Starting sudojo!");
    let mut app_starter = cmdwrapper::AppStarter::new();
    app_starter.start();
}
