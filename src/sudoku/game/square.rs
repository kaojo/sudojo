

#[derive(Clone, Debug)]
pub struct Square {
    pub value: Option<u8>,
    pub initial: bool,
    pub conflict: bool,
}

impl Square {
    pub fn new(value: Option<u8>, initial: bool) -> Self {
        Square {
            value: value,
            initial: initial,
            conflict: false,
        }
    }
}