use std::io;
use std::process;
use sudojo_core::app::{EStartChoice, App};
use sudoku;

pub struct AppStarter<T: App> {
    app: T,
}

impl<T: App> AppStarter<T> {
    pub fn new() -> AppStarter<sudoku::Sudoku> {
        AppStarter { app: sudoku::Sudoku::new()}
    }
}

impl<T: App> App for AppStarter<T> {
    fn start(&mut self, start_choice: &Option<EStartChoice>) {
        match *start_choice {
            Some(ref p) => {
                println!("Start choice is given: {:?}", p);
                &self.app.start(start_choice);
            }
            None => {
                let mut choice = get_start_choice();
                println!("{:?}", choice);
                while let EStartChoice::Invalid = choice {
                    choice = get_start_choice();
                }
                &self.app.start(&Some(choice));
            }
        }
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
