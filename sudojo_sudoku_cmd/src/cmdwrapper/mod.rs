use std::io;
use std::process;
use sudojo_core::app::{AppState, EAppState, EStartChoice, Start};
use sudojo_core::app::difficulty::EDifficulty;
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

    pub fn start(&mut self) {
        let choice: EStartChoice = AppStarter::get_start_choice();
        self.app.start(choice);
        self.app.do_loop();
    }

    fn get_start_choice() -> EStartChoice {
        let mut result = get_start_choice();
        info!("{:?}", result);
        while let EStartChoice::Invalid = result {
            result = get_start_choice();
        }
        result
    }

    fn get_difficulty() -> EDifficulty {
        let mut difficulty: Option<EDifficulty> = get_difficulty();
        while difficulty.is_none() {
            difficulty = get_difficulty();
        }
        difficulty.expect("Existence should have been checked before hand.")
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
            let difficulty: EDifficulty = AppStarter::get_difficulty();
            info!("Chosen difficulty is {:?}", difficulty);
            result = EStartChoice::NewGame(difficulty);
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

fn get_difficulty() -> Option<EDifficulty> {
    info!("Choose a difficulty for the new game:");
    info!("1) Easy");
    info!("2) Medium");
    info!("3) Hard");
    info!("4) Very hard");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line.");

    let result: Option<EDifficulty>;
    match choice.trim() {
        "1" => {
            result = Some(EDifficulty::Easy)
        }
        "2" => {
            result = Some(EDifficulty::Medium)
        }
        "3" => {
            result = Some(EDifficulty::Hard)
        }
        "4" => {
            result = Some(EDifficulty::VeryHard)
        }
        _ => {
            result = None
        }
    }

    result
}
