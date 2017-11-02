
#[derive(Debug)]
pub enum EStartChoice {
    NewGame,
    LoadGame,
    Quit,
    Invalid,
}

pub trait App {
    fn start(&mut self, &Option<EStartChoice>);
}
