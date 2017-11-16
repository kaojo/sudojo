use std::io;
use std::process;
use sudojo_core::app::{AppState, EAppState, EStartChoice, Start};
use sudojo_sudoku_core::sudoku::Sudoku;
use self::game_loop::GameLoop;

mod game_loop;
mod command_parsing;

pub struct AppStarter {
    app: Sudoku,
    app_state: EAppState,
}

impl AppStarter {
    pub fn new() -> AppStarter {
        AppStarter {
            app: Sudoku::new(),
            app_state: EAppState::Running,
        }
    }
}

impl Start for AppStarter {
    fn start(&mut self, start_choice: &Option<EStartChoice>) {
        match *start_choice {
            Some(ref p) => {
                info!("Start choice is given: {:?}", p);
                self.app.start(start_choice);
            }
            None => {
                let mut choice = get_start_choice();
                info!("{:?}", choice);
                while let EStartChoice::Invalid = choice {
                    choice = get_start_choice();
                }
                self.app.start(&Some(choice));
            }
        }
        self.app.do_loop();
    }
}

impl AppState for AppStarter {
    fn get_state(&self) -> &EAppState {
        return &self.app_state;
    }
}

fn get_start_choice() -> EStartChoice {
    info!("What do you want to do ?");
    info!("1) Start a new game.");
    info!("2) Continue last game.");
    info!("3) Quit.");
    info!("[1]: ");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line.");

    let result: EStartChoice;
    match choice.trim() {
        "" | "1" => {
            info!("Starting new game!");
            result = EStartChoice::NewGame
        }
        "2" => {
            info!("Continue last game");
            result = EStartChoice::LoadGame
        }
        "3" => {
            info!("Quiting.");
            process::exit(0);
        }
        _ => {
            info!("Invalid input");
            result = EStartChoice::Invalid
        }
    }

    return result;
}
