use std::collections::HashMap;
use super::{Coordinate, EGameState, Square};
use super::rule::{HorizontalUniqueRule, QuadrantUniqueRule, VerticalUniqueRule};
use std::fmt;
use ansi_term::Colour::{Cyan, Red, Green, Purple};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Board {
    data: Vec<Square>,
    turn_history: Vec<Coordinate>,
    initialized: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: Vec::new(),
            turn_history: Vec::new(),
            initialized: false,
        }
    }

    pub fn fill_square(&mut self, coordinate: Coordinate, square: Square) -> Result<EGameState, String> {
        debug!("{:?}, {:?}", coordinate, square);
        if self.initialized && square.initial {
            return Err(String::from(
                "Initializing squares in the board is only allowed during init phase.",
            ));
        } else if !self.initialized && !square.initial {
            return Err(String::from(
                "Can't put non initial values in the board during init phase.",
            ));
        } else {
            match self.data.get(coordinate.get_index()) {
                Some(_) => return Err(String::from(
                    "A Field with these coordinates allready exists!",
                )),
                None => {
                    if self.initialized {
                        self.turn_history.push(coordinate);
                    }
                    self.data.insert(coordinate.get_index(), square);
                }
            }
            return Ok(Board::evaluate_after_add(self, &coordinate));
        }
    }

    pub fn delete_square(&mut self, coord: &Coordinate) -> Result<EGameState, String> {
        let mut error: bool = false;
        match self.data.get(coord.get_index()) {
            Some(ref p) => if p.initial {
                error = true;
            },
            None => (),
        }
        if error {
            return Err(String::from("Deleting an initial square is not allowed."));
        }
        self.data.remove(coord.get_index());
        Ok(self.get_state())
    }

    pub fn delete_force(&mut self, coordinate: &Coordinate) {
        self.data.remove(coordinate.get_index());
    }

    pub fn undo_last(&mut self) {
        match self.turn_history.clone().len() {
            0 => (),
            n => {
                let coord = &self.turn_history.clone()[n - 1];
                self.delete_square(coord).expect("should allways work");
                self.turn_history.remove(n - 1);
            }
        }
    }

    pub fn revert(&mut self) {
        self.data.retain(|&s| s.initial);
    }

    pub fn get_square(&self, coord: &Coordinate) -> Option<&Square> {
        self.data.get(coord.get_index())
    }

    pub fn initialized(&mut self, init: bool) {
        self.initialized = init;
    }

    pub fn get_data(&self) -> &Vec<Square> {
        &self.data
    }

    pub fn mark_conflicts(&mut self) -> bool {
        self.reset_conflicts();
        let cloned_data = self.data.clone();
        let mut conflicts: HashSet<Coordinate> = HashSet::new();
        for (index, _) in cloned_data.iter().enumerate() {
            let coord: Coordinate = Coordinate::from_index(index);
            match HorizontalUniqueRule::apply(&coord, &self) {
                EGameState::Conflict => {
                    conflicts.insert(coord);
                }
                _ => (),
            }
            match VerticalUniqueRule::apply(&coord, &self) {
                EGameState::Conflict => {
                    conflicts.insert(coord);
                }
                _ => (),
            }
            match QuadrantUniqueRule::apply(&coord, &self) {
                EGameState::Conflict => {
                    conflicts.insert(coord);
                }
                _ => (),
            }
        }
        for coord in conflicts.iter() {
            let square = self.data.get_mut(coord.get_index()).expect("Should be in list.");
            square.conflict = true;
        }
        !conflicts.is_empty()
    }

    pub fn has_conflicts(&self) -> bool {
        for (index, _) in self.data.iter().enumerate() {
            let coord = Coordinate::from_index(index);
            match HorizontalUniqueRule::apply(&coord, &self) {
                EGameState::Conflict => {
                    return true;
                }
                _ => (),
            }
            match VerticalUniqueRule::apply(&coord, &self) {
                EGameState::Conflict => {
                    return true;
                }
                _ => (),
            }
            match QuadrantUniqueRule::apply(&coord, &self) {
                EGameState::Conflict => {
                    return true;
                }
                _ => (),
            }
        }
        false
    }

    pub fn is_filled(&self) -> bool {
        debug!("{} / 81 squares filled.", self.data.len());
        return 81 == self.data.len();
    }

    pub fn get_state(&self) -> EGameState {
        if self.has_conflicts() {
            EGameState::Conflict
        } else if self.is_filled() {
            EGameState::Finished
        } else {
            EGameState::Ok
        }
    }

    fn reset_conflicts(&mut self) {
        for square in self.data.iter_mut() {
            square.conflict = false;
        }
    }

    fn evaluate_after_add(board: &Board, coordinate: &Coordinate) -> EGameState {
        if EGameState::Conflict == HorizontalUniqueRule::apply(coordinate, board) {
            return EGameState::Conflict;
        }
        if EGameState::Conflict == VerticalUniqueRule::apply(coordinate, board) {
            return EGameState::Conflict;
        }
        if EGameState::Conflict == QuadrantUniqueRule::apply(coordinate, board) {
            return EGameState::Conflict;
        }
        if board.is_filled() {
            return EGameState::Finished;
        }
        EGameState::Ok
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Board");
        let boarders = [1, 4, 7];
        for y in 1..10 {
            if boarders.contains(&y) {
                let _ = writeln!(f, "+-------+-------+-------+");
            }
            for x in 1..10 {
                if boarders.contains(&x) {
                    let _ = write!(f, "| ");
                }
                let square = self.get_square(&Coordinate::new(x, y));
                let _ = write!(
                    f,
                    "{} ",
                    match square {
                        None => String::from(" "),
                        Some(ref p) => {
                            let mut digit = p.value.to_string();
                            if p.initial {
                                digit = Cyan.paint(digit).to_string();
                            } else if p.conflict && !p.generated {
                                digit = Red.paint(digit).to_string();
                            } else if p.conflict && p.generated {
                                digit = Purple.paint(digit).to_string();
                            } else if p.generated {
                                digit = Green.paint(digit).to_string();
                            }
                            String::from(digit)
                        }
                    }
                );
            }
            let _ = writeln!(f, "|");
        }
        writeln!(f, "+-------+-------+-------+")
    }
}
