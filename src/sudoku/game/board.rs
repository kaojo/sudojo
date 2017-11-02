use std::collections::HashMap;
use super::{Coordinate, Square};
use std::fmt;
use ansi_term::Colour::Red;

#[derive(Clone, Debug)]
pub struct Board {
    data: HashMap<Coordinate, Square>,
    initialized: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: HashMap::new(),
            initialized: false,
        }
    }

    pub fn fill_square(&mut self, coord: Coordinate, square: Square) -> Result<(), String> {
        if self.initialized && square.initial {
            return Err(String::from(
                "Initializing squares in the board is only allowed during init phase.",
            ));
        } else if !self.initialized && !square.initial {
            return Err(String::from(
                "Can't put non initial values in the board during init phase.",
            ));
        } else {
            let result: Option<Square> = self.data.insert(coord, square);
            match result {
                Some(ref p) => match p.value {
                    Some(_) => {
                        return Err(String::from(
                            "A Field with these coordinates allready exists!",
                        ))
                    }
                    None => Ok(()),
                },
                None => return Ok(()),
            }
        }
    }

    pub fn get_square(&self, coord: &Coordinate) -> Option<&Square> {
        self.data.get(coord)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Board");
        let boarders = [1, 4, 7];
        for y in 1..10 {
            if boarders.contains(&y) {
                let _ = writeln!(f, "-------------------------");
            }
            for x in 1..10 {
                if boarders.contains(&x) {
                    let _ = write!(f, "| ");
                }
                let default_square = Square::new(None, true);
                let square = self.get_square(&Coordinate::new(x, y))
                        .unwrap_or(&default_square);
                let _ = write!(
                    f,
                    "{} ",
                    match square.value
                    {
                        None => String::from(" "),
                        Some(ref p) => {
                            let mut digit = p.to_string();
                            if square.conflict {
                                digit = Red.paint(digit).to_string();
                            }
                            String::from(digit)
                            },
                    }
                );
            }
            let _ = writeln!(f, "|");
        }
        writeln!(f, "-------------------------")
    }
}
