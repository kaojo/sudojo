#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Coordinate { x: x, y: y }
    }

    pub fn get_index(&self) -> usize {
        return (9 * (self.y-1) + self.x -1) as usize;
    }

    pub fn from_index(index: usize) -> Self {
        let x: u8 = index as u8 % 9 + 1;
        let y: u8 = index as u8 / 9 + 1;
        Coordinate {
            x,
            y,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index() {
        assert_eq!(0, Coordinate::new(1,1,).get_index());
        assert_eq!(8, Coordinate::new(9,1,).get_index());
        assert_eq!(9, Coordinate::new(1,2,).get_index());
        assert_eq!(80, Coordinate::new(9,9,).get_index());
    }

    #[test]
    fn test_from_index() {
        //x
        assert_eq!(1, Coordinate::from_index(0).x);
        assert_eq!(2, Coordinate::from_index(1).x);
        assert_eq!(3, Coordinate::from_index(2).x);
        assert_eq!(4, Coordinate::from_index(3).x);
        assert_eq!(5, Coordinate::from_index(4).x);
        assert_eq!(6, Coordinate::from_index(5).x);
        assert_eq!(7, Coordinate::from_index(6).x);
        assert_eq!(8, Coordinate::from_index(7).x);
        assert_eq!(9, Coordinate::from_index(8).x);
        assert_eq!(1, Coordinate::from_index(9).x);
        assert_eq!(1, Coordinate::from_index(18).x);
        assert_eq!(1, Coordinate::from_index(81).x);
        //y
        assert_eq!(1, Coordinate::from_index(0).y);
        assert_eq!(1, Coordinate::from_index(8).y);
        assert_eq!(2, Coordinate::from_index(9).y);
        assert_eq!(9, Coordinate::from_index(80).y);
    }
}