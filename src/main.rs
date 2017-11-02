extern crate sudojo;
extern crate sudojo_core;

use sudojo::cmdwrapper;#
use sudojo_core::app::{App};

fn main() {
    println!("Starting sudojo!");
    let mut app_starter = cmdwrapper::AppStarter::new();
    app_starter.start(&None);

}
