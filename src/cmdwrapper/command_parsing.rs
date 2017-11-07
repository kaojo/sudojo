
pub enum EActionType {
    Turn(Option<str>),
    Undo,
    Delete(Option<(str, str)>,
    Revert,
    Help,
    Suggest,
    Solve
}

pub struct CommandActionTypeParser {}

impl CommandActionTypeParser {
    pub fn print_help(&self) {
        println!("To perform an action first enter the type of action you want to perform");
        println!("t [x,y,z] - (x,y) are coordinates, z is the value for the square");
        println!("u - undo last turn");
        println!("d:x,y - delete a square");
        println!("r - revert everything");
        println!("c - complete next turn for me");
        println!("s - solve sudoku for me");
        println!("h - print this help text");
    }

    pub fn get_action_type(&self, raw_command: &String) -> EActionType {
        
    }
}
