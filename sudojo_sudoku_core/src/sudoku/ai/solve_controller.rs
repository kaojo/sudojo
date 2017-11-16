use super::SuggestionController;
use super::super::game::{Board, EGameState, Coordinate};
use super::{VirtualBoard, Field};
use super::super::util::iterators::board_iterator;

pub struct SolveController {}

impl SolveController {
    pub fn new() -> Self {
        let result = SolveController {};

        result
    }

    pub fn solve(&self, b: &Board) -> Board {
        let mut board: Board = b.clone();

        board = do_solving_iteration(board);
/*        if board.get_state() != EGameState::Finished {
            let v_board = VirtualBoard::new(&board);
            let mut guess_data: Vec<(Coordinate, Field)> = get_guess_data(v_board);
            while let Some(p) = guess_data.pop() {
                let (coord, field) = p;
                for value in field.get_possible_values().into_iter() {

                }
            }
        }*/

        board
    }
}

fn do_solving_iteration(mut board: Board) -> Board {
    let mut suggestion_controller: SuggestionController = SuggestionController::new(&board);
    let mut state: EGameState = board.get_state();
    while state == EGameState::Ok && !suggestion_controller.get_suggestions().is_empty() {
        for suggestion in suggestion_controller.get_suggestions() {
            let (coord, square) = suggestion;
            match board.fill_square(coord, square) {
                Ok(p) => {
                    state = p;
                    ()
                }
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            }
        }
        // println!("{}", board);
        suggestion_controller = SuggestionController::new(&board);
    }

    board
}

fn get_guess_data(v_board: VirtualBoard) -> Vec<(Coordinate, Field)> {
    let mut result: Vec<(Coordinate, Field)> = Vec::new();
    for threshold in 2..10 {
        for (x, y) in board_iterator() {
            let coord = Coordinate::new(x, y);
            if let Some(p) = v_board.get_field(&coord) {
                if !p.is_initial() && p.get_possible_values().len() == threshold {
                    result.push((coord, (*p).clone()));
                }
            }
        }
    }
    result.reverse();
    result
}