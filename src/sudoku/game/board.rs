use std::collections::HashMap;
use super::{Coordinate, EGameState, Square};
use super::rule::{HorizontalUniqueRule, QuadrantUniqueRule, VerticalUniqueRule};
use std::fmt;
use ansi_term::Colour::{Cyan, Red};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Board {
    data: HashMap<Coordinate, Square>,
    turn_history: Vec<Coordinate>,
    initialized: bool,
    conflicts: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: HashMap::new(),
            turn_history: Vec::new(),
            initialized: false,
            conflicts: false,
        }
    }

    pub fn fill_square(&mut self, coord: Coordinate, square: Square) -> Result<EGameState, String> {
        if self.initialized && square.initial {
            return Err(String::from(
                "Initializing squares in the board is only allowed during init phase.",
            ));
        } else if !self.initialized && !square.initial {
            return Err(String::from(
                "Can't put non initial values in the board during init phase.",
            ));
        } else {
            if self.data.contains_key(&coord) {
                return Err(String::from(
                    "A Field with these coordinates allready exists!",
                ));
            } else {
                if self.initialized {
                    self.turn_history.push(coord.clone());
                }
                self.data.insert(coord, square);
            }
            return self.get_state();
        }
    }

    pub fn delete_square(&mut self, coord: &Coordinate) -> Result<EGameState, String> {
        let mut error: bool = false;
        match self.data.get(coord) {
            Some(ref p) => if p.initial {
                error = true;
            },
            None => (),
        }
        if error {
            return Err(String::from("Deleting an initial square is not allowed."));
        }
        self.data.remove(&coord);
        self.get_state()
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
        let cloned_data = self.data.clone();
        for (coord, square) in cloned_data.iter() {
            if !square.initial {
                self.delete_square(&coord).expect("should allways work");
            }
        }
    }

    pub fn get_square(&self, coord: &Coordinate) -> Option<&Square> {
        self.data.get(coord)
    }

    pub fn initialized(&mut self, init: bool) {
        self.initialized = init;
    }

    fn mark_conflicts(&mut self) -> bool {
        let cloned_data = self.data.clone();
        let mut conflicts: HashSet<&Coordinate> = HashSet::new();
        for (coord, square) in cloned_data.iter() {
            match HorizontalUniqueRule::apply(&coord, &square, &self) {
                EGameState::Conflict => {
                    conflicts.insert(coord);
                }
                _ => (),
            }
            match VerticalUniqueRule::apply(&coord, &square, &self) {
                EGameState::Conflict => {
                    conflicts.insert(coord);
                }
                _ => (),
            }
            match QuadrantUniqueRule::apply(&coord, &square, &self) {
                EGameState::Conflict => {
                    conflicts.insert(coord);
                }
                _ => (),
            }
        }
        for coord in conflicts.iter() {
            let square = self.data.get_mut(coord).expect("Should be in list.");
            square.conflict = true;
        }
        self.conflicts = conflicts.is_empty();
        self.conflicts
    }

    pub fn is_filled(&self) -> bool {
        println!("{} / 81 squares filled.", self.data.len());
        return 81 == self.data.len();
    }

    fn get_state(&mut self) -> Result<EGameState, String> {
        self.reset_conflicts();
        if self.mark_conflicts() {
            Ok(EGameState::Conflict)
        } else if self.is_filled() {
            Ok(EGameState::Finished)
        } else {
            Ok(EGameState::Ok)
        }
    }

    fn reset_conflicts(&mut self) {
        for square in self.data.values_mut() {
            square.conflict = false;
        }
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
                            } else if p.conflict {
                                digit = Red.paint(digit).to_string();
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
