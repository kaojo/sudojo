use sudoku::game::{Board, Coordinate, Square};
use sudojo_core::app::{App, AppState, EAppState, EStartChoice, Start, Tick, Turn};
use self::generator::{Generator, SimpleGenerator};

pub mod game;
pub mod ai;
pub mod util;
mod generator;

pub struct Sudoku {
    pub board: Board,
    pub app_state: EAppState,
}

impl Sudoku {
    pub fn new() -> Self {
        Sudoku {
            board: Board::new(),
            app_state: EAppState::Running,
        }
    }
}

impl App<(Coordinate, Option<Square>), Board> for Sudoku {}

impl Start for Sudoku {
    fn start(&mut self, start_choice: EStartChoice) {
        match start_choice {
            EStartChoice::NewGame(p) => {
                info!("Starting new game!");
                self.board = SimpleGenerator::generate(p);
            }
            _ => info!("Choice not supported yet"),
        }
    }
}

impl AppState for Sudoku {
    fn get_state(&self) -> &EAppState {
        return &self.app_state;
    }
}

impl Turn<(Coordinate, Option<Square>), Board> for Sudoku {
    fn do_turn(&mut self, turn: (Coordinate, Option<Square>)) -> Result<&Board, String> {
        let (coord, square) = turn;
        match square {
            Some(ref p) => {
                self.board.fill_square(coord, p.clone())?;
                return Ok(&self.board);
            }
            None => {
                self.board.delete_square(&coord)?;
                return Ok(&self.board);
            }
        }
    }
}

impl Tick for Sudoku {
    fn do_tick(&self) {
        panic!("Not implemented!")
    }
}
