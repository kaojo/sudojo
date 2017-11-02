use sudoku::game::{Board, Coordinate, Square};
use sudojo_core::app::{EStartChoice, App};

mod game;

pub struct Sudoku {
    board: Board,
}

impl Sudoku {
    pub fn new() -> Self {
        Sudoku {
            board: Board::new(),
        }
    }

    fn init(&mut self) {
        for x in 1..10 {
            for y in 1..10 {
                let result = self.board
                    .fill_square(Coordinate::new(x, y), Square::new(None, true));
                match result {
                    Ok(_) => (),
                    Err(ref p) => panic!("Error {} during initialization at x {}, y {}", p, x, y),
                }
            }
        }
        println!("{}", self.board);
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
            let result = self.board.fill_square(Coordinate::new(x, y), Square::new(value, true));
            match result {
                Ok(_) => (),
                Err(ref p) => panic!("Error {} during initialization", p), // at x {}, y {}", p, &coord.x, &coord.y),
            }
        }
        println!("{}", self.board);
    }
}

impl App for Sudoku {
    fn start(&mut self, start_choice: &Option<EStartChoice>) {
        println!("Sudoku: {:?}", start_choice);
        match *start_choice {
            Some(EStartChoice::NewGame) => {
                println!("Starting new game!");
                &self.init();
            }
            _ => println!("Choice not supported yet"),
        }
    }
}
