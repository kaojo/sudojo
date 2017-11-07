use sudoku::game::{Board, Coordinate, Square};
use sudojo_core::app::{App, AppState, EAppState, EStartChoice, Start, Tick, Turn};

pub mod game;

pub struct Sudoku {
    board: Board,
    app_state: EAppState,
}

impl Sudoku {
    pub fn new() -> Self {
        Sudoku {
            board: Board::new(),
            app_state: EAppState::Running,
        }
    }
}

impl App<(Coordinate, Square), Board> for Sudoku {}

impl Start for Sudoku {
    fn start(&mut self, start_choice: &Option<EStartChoice>) {
        println!("Sudoku: {:?}", start_choice);
        match *start_choice {
            Some(EStartChoice::NewGame) => {
                println!("Starting new game!");
                init(&mut self.board);
            }
            _ => println!("Choice not supported yet"),
        }
        println!("{}", &self.board);
    }
}

impl AppState for Sudoku {
    fn get_state(&self) -> &EAppState {
        return &self.app_state;
    }
}

impl Turn<(Coordinate, Square), Board> for Sudoku {
    fn do_turn(&mut self, turn: (Coordinate, Square)) -> Result<&Board, String>{
        let (coord, square) = turn;
        self.board.fill_square(coord, square)?;
        Ok(&self.board)
    }
}

impl Tick for Sudoku {
    fn do_tick(&self) {
        panic!("Not implemented!")
    }
}

fn init(board: &mut Board) {
    let square_map = [
        (3, 1, 8),
        (4, 1, 1),
        (6, 1, 2),
        (8, 1, 4),
        (9, 1, 7),
        (4, 2, 7),
        (7, 2, 5),
        (2, 3, 7),
        (7, 3, 1),
        (8, 3, 8),
        (1, 4, 8),
        (4, 4, 4),
        (6, 4, 9),
        (2, 5, 2),
        (8, 5, 1),
        (4, 6, 2),
        (6, 6, 6),
        (9, 6, 3),
        (2, 7, 8),
        (3, 7, 2),
        (8, 7, 9),
        (3, 8, 6),
        (6, 8, 7),
        (1, 9, 3),
        (2, 9, 1),
        (4, 9, 6),
        (6, 9, 5),
        (7, 9, 7),
    ];
    for &(x, y, value) in square_map.into_iter() {
        let result = board.fill_square(Coordinate::new(x, y), Square::new(value, true));
        match result {
            Ok(_) => (),
            Err(ref p) => panic!("Error {} during initialization", p),
        }
    }
    board.initialized(true);
}
