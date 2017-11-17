pub mod difficulty;
use self::difficulty::EDifficulty;

#[derive(Debug)]
pub enum EStartChoice {
    NewGame(EDifficulty),
    LoadGame,
    Quit,
    Invalid,
}

#[derive(PartialEq, Eq)]
pub enum EExitAction {
    Save(Option<String>),
    Exit,
}

#[derive(PartialEq, Eq)]
pub enum EAppState {
    Exit,
    Running,
}

pub trait Turn<T, K> {
    fn do_turn(&mut self, turn: T) -> Result<&K, String>;
}

pub trait Tick {
    fn do_tick(&self);
}

pub trait Start {
    fn start(&mut self, EStartChoice);
}

pub trait AppState {
    fn get_state(&self) -> &EAppState;
}

pub trait App<T, K>: Turn<T, K> + Tick + Start + AppState {
}
