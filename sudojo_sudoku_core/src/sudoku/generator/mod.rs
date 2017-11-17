use sudojo_core::app::difficulty::EDifficulty;
use super::game::{Board, Coordinate, Square};

pub trait Generator {
    fn generate(difficulty: EDifficulty) -> Board;
}

pub struct SimpleGenerator {

}

impl Generator for SimpleGenerator {
    fn generate(difficulty: EDifficulty) -> Board  {
        init()
    }
}


fn init() -> Board {
    let mut board = Board::new();
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
    board
}