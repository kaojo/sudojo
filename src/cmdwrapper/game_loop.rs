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
            let raw_choice = get_raw_cmd();
            let cmd_type = parser.get_action_type(&raw_choice);
            match cmd_type {
                EActionType::Turn(ref p) => {
                    let result = self.do_turn(p.clone());
                    match result {
                        Err(ref p) => println!("Could not execute turn: {}", p),
                        Ok(ref p) => println!("{}", p),
                    }
                }
                EActionType::Delete(ref p) => {}
                EActionType::Help => {}
                EActionType::Revert => {}
                EActionType::Solve => {}
                EActionType::Suggest => {}
                EActionType::Undo => {}
                EActionType::Quit => {}
                _ => {}
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
