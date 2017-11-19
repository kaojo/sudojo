pub mod sudoku;
extern crate ansi_term;
extern crate sudojo_core;
#[macro_use]
extern crate log;
extern crate rand;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
