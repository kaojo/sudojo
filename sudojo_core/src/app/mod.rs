
#[derive(Debug)]
pub enum EStartChoice {
    NewGame,
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

pub enum EAction {
    Turn,
    Tick,
    Exit(EExitAction),
}

pub trait Turn {
    fn do_turn(&self);
}

pub trait Tick {
    fn do_tick(&self);
}

pub trait Start {
    fn start(&mut self, &Option<EStartChoice>);
}

pub trait AppState {
    fn get_state(&self) -> &EAppState;
}

pub trait Action {
    fn get_action(&self) -> EAction;
}

pub trait App: Turn + Tick + Start + AppState + Action {
}
