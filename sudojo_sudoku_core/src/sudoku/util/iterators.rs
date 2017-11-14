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