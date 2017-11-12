use sudojo_core::app::{AppState, EAppState, Turn};
use sudoku::Sudoku;
use std::io;
use super::command_parsing::CommandActionTypeParser;
use super::command_parsing::EActionType;

pub trait GameLoop {
    fn do_loop(&mut self);
}

impl GameLoop for Sudoku {
    fn do_loop(&mut self) {
        let parser = CommandActionTypeParser {};
        parser.print_help();
        while self.get_state() == &EAppState::Running {
            println!("{}", self.board);
            let raw_choice = get_raw_cmd();
            let cmd_type = parser.get_action_type(&raw_choice);
            match cmd_type {
                EActionType::Turn(ref p) => {
                    let result = self.do_turn(p.clone());
                    match result {
                        Err(ref p) => println!("Could not execute turn: {}", p),
                        Ok(_) => (),
                    }
                }
                EActionType::Delete(ref p) => {
                    let result = self.do_turn((p.clone(), None));
                    match result {
                        Err(ref p) => println!("Could not execute turn: {}", p),
                        Ok(_) => (),
                    }
                }
                EActionType::Help => {parser.print_help()}
                EActionType::Revert => {self.board.revert()}
                EActionType::Solve => {}
                EActionType::Suggest => {}
                EActionType::Undo => {self.board.undo_last()}
                EActionType::Quit => {self.app_state = EAppState::Exit}
                _ => {
                    println!("Command '{}' not recognized.", raw_choice);
                    println!("Enter 'h' to get help.");
                }
            }
        }
    }
}


fn get_raw_cmd() -> String {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line.");
    choice
}
