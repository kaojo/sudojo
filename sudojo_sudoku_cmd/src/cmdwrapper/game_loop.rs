use sudojo_core::app::{AppState, EAppState, Turn};
use sudojo_sudoku_core::sudoku::ai::SuggestionController;
use sudojo_sudoku_core::sudoku::Sudoku;
use sudojo_sudoku_core::sudoku::game::{Coordinate, Square};
use std::io;
use std::collections::HashSet;
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
                EActionType::Suggest => {
                    let suggestion_controller : SuggestionController = SuggestionController::new(&self.board);
                    let possible_turns: HashSet<(Coordinate, Square)> = suggestion_controller.get_suggestions();
                }
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
