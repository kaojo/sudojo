use std::io;
use std::process;
use sudojo_core::app::{App, AppState, EAction, EAppState, EStartChoice, Start};
use sudoku::Sudoku;
use sudoku::game::{Coordinate, Square};
use regex::Regex;

pub struct AppStarter {
    app: Box<App<(Coordinate, Square)>>,
    app_state: EAppState,
}

impl AppStarter {
    pub fn new() -> AppStarter {
        AppStarter {
            app: Box::from(Sudoku::new()),
            app_state: EAppState::Running,
        }
    }
}

impl Start for AppStarter {
    fn start(&mut self, start_choice: &Option<EStartChoice>) {
        match *start_choice {
            Some(ref p) => {
                println!("Start choice is given: {:?}", p);
                self.app.start(start_choice);
            }
            None => {
                let mut choice = get_start_choice();
                println!("{:?}", choice);
                while let EStartChoice::Invalid = choice {
                    choice = get_start_choice();
                }
                self.app.start(&Some(choice));
            }
        }
        while self.app.as_ref().get_state() == &EAppState::Running {
            let action = self.app.get_action();
            match action {
                EAction::Turn => {
                    let mut choice = get_turn();
                    while let None = choice {
                        choice = get_turn();
                    }
                    self.app.do_turn(choice.unwrap());
                }
                _ => (),
            }
        }
    }
}

impl AppState for AppStarter {
    fn get_state(&self) -> &EAppState {
        return &self.app_state;
    }
}

fn get_turn() -> Option<(Coordinate, Square)> {
    println!("Next turn: ");
    println!("x,y,z");
    println!("u - undo last turn");
    println!("r - revert everything");
    println!("h - do next allowed turn for me");
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
            Some((Coordinate::new(x, y), Square::new(Some(z), false)))
        }
        None => None,
    }
}


fn get_start_choice() -> EStartChoice {
    println!("What do you want to do ?");
    println!("1) Start a new game.");
    println!("2) Continue last game.");
    println!("3) Quit.");
    println!("[1]: ");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line.");

    let result: EStartChoice;
    match choice.trim() {
        "" | "1" => {
            println!("Starting new game!");
            result = EStartChoice::NewGame
        }
        "2" => {
            println!("Continue last game");
            result = EStartChoice::LoadGame
        }
        "3" => {
            println!("Quiting.");
            process::exit(0);
        }
        _ => {
            println!("Invalid input");
            result = EStartChoice::Invalid
        }
    }

    return result;
}
