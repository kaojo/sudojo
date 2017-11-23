use super::SuggestionController;
use super::super::game::{Board, EGameState, Coordinate, Square};
use super::{VirtualBoard, Field};
use super::super::util::iterators::board_iterator;

pub struct SolveController {
    intelligence: ESolvingIntelligence,
}

impl SolveController {
    pub fn new(intelligence: ESolvingIntelligence) -> Self {
        let result = SolveController {
            intelligence,
        };

        result
    }

    pub fn solve(&self, b: &Board) -> Result<Board, String> {
        iterate_solve(b.clone(), &self.intelligence)
    }
}

#[derive(Debug, PartialEq)]
pub enum ESolvingIntelligence {
    SimpleLogic,
    ComplexLogic,
    WithGuessing
}

fn iterate_solve(b: Board, intelligence: &ESolvingIntelligence) -> Result<Board, String> {
    if b.get_state() != EGameState::Ok {
        return Ok(b);
    }
    let board = do_solving_iteration(b.clone(), intelligence.clone());
    debug!("{:?}", board.get_state());
    if &ESolvingIntelligence::WithGuessing == intelligence && board.get_state() == EGameState::Ok {
        let v_board = VirtualBoard::new(&board, intelligence);
        let tmp = get_guess_data(v_board);
        if tmp.is_none() {
            let message = "No possible guess values found";
            debug!("{}", message);
            return Err(String::from(message));
        }
        let (index, field) = tmp.unwrap();
        for value in field.get_possible_values().into_iter() {
            let mut tmp_board = board.clone();
            debug!("Guessing value {} at index {:?}", value, index);
            match tmp_board.fill_square(index, Square::guess(*value)) {
                Err(p) => error!("{}", p),
                Ok(EGameState::Conflict) => {
                    debug!("Conflict");
                    tmp_board.undo_last();
                }
                Ok(EGameState::Finished) => return Ok(tmp_board),
                Ok(EGameState::Ok) => {
                    debug!("Ok");
                    let res = iterate_solve(tmp_board, intelligence)?;
                    match res.get_state() {
                        EGameState::Conflict => {
                            debug!("Inner iterate result: {:?}", res.get_state());
                            continue;
                        }
                        _ => {
                            debug!("Inner iterate result: {:?}", res.get_state());
                            return Ok(res);
                        }
                    }
                }
            }
        }
    }
    Ok(board)
}

fn do_solving_iteration(mut board: Board, intelligence: &ESolvingIntelligence) -> Board {
    let mut suggestion_controller: SuggestionController = SuggestionController::new(&board, intelligence);
    let mut state: EGameState = board.get_state();
    while state == EGameState::Ok && !suggestion_controller.get_suggestions().is_empty() {
        for suggestion in suggestion_controller.get_suggestions() {
            let (index, square) = suggestion;
            match board.fill_square(index, square) {
                Ok(p) => {
                    state = p;
                    ()
                }
                Err(e) => {
                    debug!("{}", e);
                    break;
                }
            }
        }
        board.mark_conflicts();
        debug!("{}", board);
        suggestion_controller = SuggestionController::new(&board, intelligence);
    }

    board
}

fn get_guess_data(v_board: VirtualBoard) -> Option<(usize, Field)> {
    for threshold in 2..10 {
        for i in 0..81 {
            if let Some(p) = v_board.get_field(i) {
                if !p.is_initial() && p.get_possible_values().len() == threshold {
                    return Some((i, p.clone()));
                }
            }
        }
    }
    None
}