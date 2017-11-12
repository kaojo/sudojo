use sudoku::game::{Coordinate, Square};
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
        println!("To perform an action first enter the type of action you want to perform");
        println!("t x,y,z - (x,y) are coordinates, z is the value for the square.");
        println!("u - undo last turn");
        println!("d [x,y] - delete a square");
        println!("r - revert everything");
        println!("c - complete next turn for me");
        println!("s - solve sudoku for me");
        println!("q - quit the current game and go back to main menu");
        println!("h - print this help text");
        println!("");
    }

    pub fn get_action_type(&self, raw_command: &String) -> EActionType {
        if raw_command.starts_with("t") {
            let command: String = raw_command.chars().skip(1).take(raw_command.len()).collect();
            match parse_turn(command) {
                Some(ref p) => EActionType::Turn(p.clone()),
                None => EActionType::Invalid,
            }
        } 
        else if raw_command.starts_with("u") {
            EActionType::Undo
        }
        else if raw_command.starts_with("d") {
            let command: String = raw_command.chars().skip(1).take(raw_command.len()).collect();
            match parse_delete(command) {
                Some(ref p) => EActionType::Delete(p.clone()),
                None => EActionType::Invalid,
            }
        }
        else if raw_command.starts_with("r") {
            EActionType::Revert
        }
        else if raw_command.starts_with("c") {
            EActionType::Suggest
        }
        else if raw_command.starts_with("s") {
            EActionType::Solve
        }
        else if raw_command.starts_with("q") {
            EActionType::Quit
        }
        else if raw_command.starts_with("h") {
            EActionType::Help
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
