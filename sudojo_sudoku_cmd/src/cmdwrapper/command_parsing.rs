use sudojo_sudoku_core::sudoku::game::{Coordinate, Square};
use regex::Regex;

pub enum EActionType {
    Turn((Coordinate, Option<Square>)),
    Undo,
    Delete(Coordinate),
    Revert,
    Help,
    Suggest,
    Solve,
    Invalid,
    Quit,
}

pub struct CommandActionTypeParser {}

impl CommandActionTypeParser {
    pub fn print_help(&self) {
        info!("-------------------------------------------------------------------------------------------------------------------");
        info!("To perform an action first enter the type of action you want to perform");
        info!("t x,y,z - (x,y) are coordinates, z is the value for the square. 't' can be omitted since it is the default command.");
        info!("u - undo last turn");
        info!("d [x,y] - delete a square");
        info!("r - revert everything");
        info!("c - complete next turn for me");
        info!("s - solve sudoku for me");
        info!("q - quit the current game and go back to main menu");
        info!("h - print this help text");
        info!("-------------------------------------------------------------------------------------------------------------------");
        info!("");
    }

    pub fn get_action_type(&self, raw_command: &String) -> EActionType {
        if raw_command.starts_with("t") {
            let command: String = raw_command
                .chars()
                .skip(1)
                .take(raw_command.len())
                .collect();
            match parse_turn(command) {
                Some(ref p) => EActionType::Turn(p.clone()),
                None => EActionType::Invalid,
            }
        } else if raw_command.starts_with("u") {
            EActionType::Undo
        } else if raw_command.starts_with("d") {
            let command: String = raw_command
                .chars()
                .skip(1)
                .take(raw_command.len())
                .collect();
            match parse_delete(command) {
                Some(ref p) => EActionType::Delete(p.clone()),
                None => EActionType::Invalid,
            }
        } else if raw_command.starts_with("r") {
            EActionType::Revert
        } else if raw_command.starts_with("c") {
            EActionType::Suggest
        } else if raw_command.starts_with("s") {
            EActionType::Solve
        } else if raw_command.starts_with("q") {
            EActionType::Quit
        } else if raw_command.starts_with("h") {
            EActionType::Help
        } else if parse_turn(raw_command.clone()).is_some() {
            match parse_turn(raw_command.clone()) {
                Some(ref p) => EActionType::Turn(p.clone()),
                None => EActionType::Invalid,
            }
        } else {
            EActionType::Invalid
        }
    }
}

fn parse_turn(text: String) -> Option<(Coordinate, Option<Square>)> {
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
            Some((Coordinate::new(x, y), Some(Square::new(z, false))))
        }
        None => None,
    }
}
fn parse_delete(text: String) -> Option<(Coordinate)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<x>\d),(?P<y>\d)$").unwrap();
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
            Some((Coordinate::new(x, y)))
        }
        None => None,
    }
}
