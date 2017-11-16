extern crate sudojo_sudoku_core;
extern crate sudojo_core;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod cmdwrapper;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
