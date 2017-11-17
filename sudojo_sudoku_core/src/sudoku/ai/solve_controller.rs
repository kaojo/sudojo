use super::SuggestionController;
use super::super::game::{Board, EGameState, Coordinate, Square};
use super::{VirtualBoard, Field};
use super::super::util::iterators::board_iterator;

pub struct SolveController {}

impl SolveController {
    pub fn new() -> Self {
        let result = SolveController {};

        result
    }

    pub fn solve(&self, b: &Board) -> Board {
        iterate_solve(b.clone())
    }
}

fn iterate_solve(b: Board) -> Board {
    if b.get_state() != EGameState::Ok {
        return b;
    }
    let board = do_solving_iteration(b.clone());
    debug!("{:?}", board.get_state());
    if board.get_state() == EGameState::Ok {
        let v_board = VirtualBoard::new(&board);
        let (coordinate, field) = get_guess_data(v_board).expect("Should resolve");
        for value in field.get_possible_values().into_iter() {
            let mut tmp_board = board.clone();
            debug!("Guessing value {} at coordinate {:?}",value, coordinate);
            match tmp_board.fill_square(coordinate.clone(), Square::guess(*value)) {
                Err(p) => error!("{}", p),
                Ok(EGameState::Conflict) => {
                    debug!("Conflict");
                    tmp_board.undo_last();
                }
                Ok(EGameState::Finished) => return tmp_board,
                Ok(EGameState::Ok) => {
                    debug!("Ok");
                    let res = iterate_solve(tmp_board);
                    {
                        match res.get_state() {
                            EGameState::Conflict => {
                                debug!("Inner iterate result: {:?}", res.get_state());
                                continue;
                            }
                            _ => {
                                debug!("Inner iterate result: {:?}", res.get_state());
                                return res
                            },
                        }
                    }
                }
            }
        }
    }
    board
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
                    info!("{}", e);
                    break;
                }
            }
        }
        board.mark_conflicts();
        debug!("{}", board);
        suggestion_controller = SuggestionController::new(&board);
    }

    board
}

fn get_guess_data(v_board: VirtualBoard) -> Option<(Coordinate, Field)> {
    for threshold in 2..10 {
        for (x, y) in board_iterator() {
            let coord = Coordinate::new(x, y);
            if let Some(p) = v_board.get_field(&coord) {
                if !p.is_initial() && p.get_possible_values().len() == threshold {
                    return Some((coord, p.clone()));
                }
            }
        }
    }
    None
}