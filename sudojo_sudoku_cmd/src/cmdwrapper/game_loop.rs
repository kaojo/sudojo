use sudojo_core::app::{AppState, EAppState, Turn};
use sudojo_sudoku_core::sudoku::ai::{SuggestionController, SolveController, ESolvingIntelligence};
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
            self.board.mark_conflicts();
            info!("{}", self.board);
            if self.board.is_filled() {
                print_success_message();
            }
            let raw_choice = get_raw_cmd();
            let cmd_type = parser.get_action_type(&raw_choice);
            match cmd_type {
                EActionType::Turn(ref p) => {
                    let result = self.do_turn(p.clone());
                    match result {
                        Err(ref p) => info!("Could not execute turn: {}", p),
                        Ok(_) => (),
                    }
                }
                EActionType::Delete(ref p) => {
                    let result = self.do_turn((p.clone(), None));
                    match result {
                        Err(ref p) => info!("Could not execute turn: {}", p),
                        Ok(_) => (),
                    }
                }
                EActionType::Help => { parser.print_help() }
                EActionType::Revert => { self.board.revert() }
                EActionType::Solve => {
                    let sc = SolveController::new();
                    self.board = sc.solve(&self.board, &ESolvingIntelligence::WithGuessing);
                }
                EActionType::Suggest => {
                    let suggestion_controller: SuggestionController = SuggestionController::new(&self.board, &ESolvingIntelligence::ComplexLogic);
                    let possible_turns: HashSet<(Coordinate, Square)> = suggestion_controller.get_suggestions();
                    for suggestion in possible_turns.into_iter() {
                        let (coord, square) = suggestion;
                        info!("{},{},{}", coord.x, coord.y, square.value)
                    }
                }
                EActionType::Undo => { self.board.undo_last() }
                EActionType::Quit => { self.app_state = EAppState::Exit }
                _ => {
                    info!("Command '{}' not recognized.", raw_choice);
                    info!("Enter 'h' to get help.");
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

fn print_success_message() {
    println!(r"

   _____ _    _  _____ _____ ______  _____ _____
  / ____| |  | |/ ____/ ____|  ____|/ ____/ ____|
 | (___ | |  | | |   | |    | |__  | (___| (___
  \___ \| |  | | |   | |    |  __|  \___ \\___ \
  ____) | |__| | |___| |____| |____ ____) |___) |
 |_____/ \____/ \_____\_____|______|_____/_____/


")
}
