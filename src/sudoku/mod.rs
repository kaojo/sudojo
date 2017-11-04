use sudoku::game::{Board, Coordinate, Square};
use sudojo_core::app::{Action, App, AppState, EAction, EAppState, EStartChoice, Start, Tick, Turn};

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

impl App<(Coordinate, Square)> for Sudoku {}

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

impl Turn<(Coordinate, Square)> for Sudoku {
    fn do_turn(&mut self, turn: (Coordinate, Square)) {
        let (coord, square) = turn;
        self.board.fill_square(coord, square).expect("not allowed");
        println!("{}", &self.board);
    }
}

impl Tick for Sudoku {
    fn do_tick(&self) {
        panic!("Not implemented!")
    }
}

impl Action for Sudoku {
    fn get_action(&self) -> EAction {
        EAction::Turn
    }
}

fn init(board: &mut Board) {
    for x in 1..10 {
        for y in 1..10 {
            let result = board.fill_square(Coordinate::new(x, y), Square::new(None, true));
            match result {
                Ok(_) => (),
                Err(ref p) => panic!("Error {} during initialization at x {}, y {}", p, x, y),
            }
        }
    }
    let square_map = [
        (3, 1, Some(8)),
        (4, 1, Some(1)),
        (6, 1, Some(2)),
        (8, 1, Some(4)),
        (9, 1, Some(7)),
        (4, 2, Some(7)),
        (7, 2, Some(5)),
        (2, 3, Some(7)),
        (7, 3, Some(1)),
        (8, 3, Some(8)),
        (1, 4, Some(8)),
        (4, 4, Some(4)),
        (6, 4, Some(9)),
        (2, 5, Some(2)),
        (8, 5, Some(1)),
        (4, 6, Some(2)),
        (6, 6, Some(6)),
        (9, 6, Some(3)),
        (2, 7, Some(8)),
        (3, 7, Some(2)),
        (8, 7, Some(9)),
        (3, 8, Some(6)),
        (6, 8, Some(7)),
        (1, 9, Some(3)),
        (2, 9, Some(1)),
        (4, 9, Some(6)),
        (6, 9, Some(5)),
        (7, 9, Some(7)),
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
