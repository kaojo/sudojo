

#[derive(Debug)]
pub struct Square {
    pub value: Option<u8>,
    pub initial: bool,
}

impl Square {
    pub fn new(value: Option<u8>, initial: bool) -> Self {
        Square {
            value: value,
            initial: initial,
        }
    }
}