use sudojo_core::app::{EAppState, Turn, AppState};
use sudoku::Sudoku;
use sudoku::game::Coordinate;
use sudoku::game::Square;
use regex::Regex;
use std::io;

pub trait GameLoop {
    fn do_loop(&mut self);
}

impl GameLoop for Sudoku {
    fn do_loop(&mut self) {
        while self.get_state() == &EAppState::Running {
            let mut choice = get_turn();
            while let None = choice {
                choice = get_turn();
            }
            let result = self.do_turn(choice.unwrap());
            match result {
                Err(ref p) => println!("Could not execute turn: {}", p),
                Ok(ref p) => println!("{}", p),
            }
        }
    }
}


fn get_turn() -> Option<(Coordinate, Square)> {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line.");
    parse_turn(&choice)
}

fn parse_turn(text: &str) -> Option<(Coordinate, Square)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<x>\d),(?P<y>\d),(?P<z>\d)$").unwrap();
    }
    match RE.captures(text.trim()) {
        Some(ref p) => {
            let x = p.name("x")
                .expect("x should not be optional in regex expression")
                .as_str()
                .parse::<u8>()
                .expect("should be an integer");
            let y = p.name("y")
                .expect("y should not be optional in regex expression")
                .as_str()
                .parse::<u8>()
                .expect("should be an integer");
            let z = p.name("z")
                .expect("z should not be optional in regex expression")
                .as_str()
                .parse::<u8>()
                .expect("should be an integer");
            println!("{},{},{}", x, y, z);
            Some((Coordinate::new(x, y), Square::new(z, false)))
        }
        None => None,
    }
}
