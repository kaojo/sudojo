pub mod cmdwrapper;
pub mod sudoku;
extern crate ansi_term;
extern crate sudojo_core;
extern crate regex;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
