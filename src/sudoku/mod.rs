use sudoku::game::{Board, Coordinate, Square};
use sudojo_core::app::{EStartChoice, EAppState, EAction, App, Turn, Tick, Start, AppState, Action};
use std::io;
use regex::Regex;

mod game;

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

impl App for Sudoku {
}

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
    }
}

impl AppState for Sudoku {
    fn get_state(&self) -> &EAppState {
        return &self.app_state
    }
}

impl Turn for Sudoku {
    fn do_turn(&self) {
        let mut choice = get_turn();
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

fn get_turn() -> String {
    println!("Next turn: ");
    println!("x,y,z");
    println!("h - help");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line.");


    choice
}

fn is_turn(text: &str) -> Option<(Coordinate, Square)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<x>\d),(?P<y>\d),(?P<z>\d)$").unwrap();
    }
    match RE.captures(text.trim()) {
        Some(ref p) => {
            p.name("x");
            Some((Coordinate::new(1, 2), Square::new(Some(1), false)))
        },
        None => None
    }
}

fn init(board : &mut Board) {
    for x in 1..10 {
        for y in 1..10 {
            let result = board
                .fill_square(Coordinate::new(x, y), Square::new(None, true));
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
    for &(x,y,value) in square_map.into_iter() {
        let result = board.fill_square(Coordinate::new(x, y), Square::new(value, true));
        match result {
            Ok(_) => (),
            Err(ref p) => panic!("Error {} during initialization", p), // at x {}, y {}", p, &coord.x, &coord.y),
        }
    }
}
