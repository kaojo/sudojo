use std::ops::Range;

pub struct BoardIterator {
    x: u8,
    next_x: u8,
    y: u8,
    next_y: u8,
}

impl Iterator for BoardIterator {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<(u8, u8)> {
        self.x = self.next_x;
        self.y = self.next_y;

        // stop condition
        if self.next_y == 10 {
            return None;
        }

        // update next values
        if self.x < 9 {
            self.next_x = self.x + 1;
        } else {
            self.next_x = 1;
            self.next_y = self.y + 1;
        }

        Some((self.x, self.y))
    }
}

pub fn board_iterator() -> BoardIterator {
    BoardIterator {
        x: 1,
        next_x: 1,
        y: 1,
        next_y: 1,
    }
}

pub struct QuadrantIterator {
    x: u8,
    next_x: u8,
    y: u8,
    next_y: u8,
}

impl Iterator for QuadrantIterator {
    type Item = QuadrantSquaresIterator;

    fn next(&mut self) -> Option<QuadrantSquaresIterator> {
        self.x = self.next_x;
        self.y = self.next_y;

        // stop condition
        if self.next_y == 4 {
            return None;
        }

        // update next values
        if self.x < 3 {
            self.next_x = self.x + 1;
        } else {
            self.next_x = 1;
            self.next_y = self.y + 1;
        }

        Some(quadrant_squares_iterator(self.x, self.y))
    }
}

pub fn quadrant_iterator() -> QuadrantIterator {
    QuadrantIterator {
        x: 1,
        next_x: 1,
        y: 1,
        next_y: 1,
    }
}

pub struct QuadrantSquaresIterator {
    pub q_x: u8,
    pub q_y: u8,
    x: u8,
    next_x: u8,
    y: u8,
    next_y: u8,
}

impl Iterator for QuadrantSquaresIterator {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<(u8, u8)> {
        self.x = self.next_x;
        self.y = self.next_y;

        // stop condition
        if self.next_y == self.q_y * 3 + 1 {
            return None;
        }

        // update next values
        if self.x < self.q_x * 3 {
            self.next_x = self.x + 1;
        } else {
            self.next_x = (self.q_x - 1) * 3 + 1;
            self.next_y = self.y + 1;
        }

        Some((self.x, self.y))
    }
}

pub fn quadrant_squares_iterator(q_x: u8, q_y: u8) -> QuadrantSquaresIterator {
    QuadrantSquaresIterator {
        q_x,
        q_y,
        x: (q_x - 1) * 3 + 1,
        next_x: (q_x - 1) * 3 + 1,
        y: (q_y - 1) * 3 + 1,
        next_y: (q_y - 1) * 3 + 1,
    }
}
 impl QuadrantSquaresIterator {
     pub fn from_board_coordinates(b_x: u8, b_y: u8) -> QuadrantSquaresIterator {
         let q_x = (b_x as f32 / 3 as f32).ceil() as u8;
         let q_y = (b_y as f32 / 3 as f32).ceil() as u8;

         QuadrantSquaresIterator {
             q_x,
             q_y,
             x: (q_x - 1) * 3 + 1,
             next_x: (q_x - 1) * 3 + 1,
             y: (q_y - 1) * 3 + 1,
             next_y: (q_y - 1) * 3 + 1,
         }
     }
 }


pub struct IndexRowIterator {}

impl Iterator for IndexRowIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub fn index_row_iterator(index: usize) -> Range<usize> {
    let start = index - index % 9;
    let end = index - index % 9 + 9;
    (start as usize)..(end as usize)
}

pub fn index_column_iterator(index: usize) -> Vec<usize> {
    let mut result = vec![];
    for row in 0..9 {
        result.push((row * 9 + index % 9) as usize);
    }
    result
}

pub fn index_quadrant_iterator(index: usize) -> Vec<usize> {
    let mut result = Vec::new();

    let row = index / 9;
    let column = index % 9;

    let top_left = (row - row % 3) * 9 + column - column % 3;

    for i in 0..3 {
        let lower = top_left + i * 9;
        for value in lower..(lower + 3) {
            result.push(value as usize)
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_row_iterator() {
        for value in index_row_iterator(8) {
            assert!(value <= 8);
        }
        for value in index_row_iterator(80) {
            assert!(value >= 72);
            assert!(value <= 80);
        }
    }

    #[test]
    fn test_index_column_iterator() {
        let mut res = vec![6, 7, 8, 15, 16, 17, 24, 25, 26];
        assert_eq!(res, index_quadrant_iterator(6));
        assert_eq!(res, index_quadrant_iterator(7));
        assert_eq!(res, index_quadrant_iterator(8));
        assert_eq!(res, index_quadrant_iterator(15));
        assert_eq!(res, index_quadrant_iterator(16));
        assert_eq!(res, index_quadrant_iterator(17));
        assert_eq!(res, index_quadrant_iterator(24));
        assert_eq!(res, index_quadrant_iterator(25));
        assert_eq!(res, index_quadrant_iterator(26));
        res = vec![0, 1, 2, 9, 10, 11, 18, 19, 20];
        assert_eq!(res, index_quadrant_iterator(1));
        res = vec![60, 61, 62, 69, 70, 71, 78, 79, 80];
        assert_eq!(res, index_quadrant_iterator(80));
    }
}