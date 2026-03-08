use crate::player::{AskAction, PlayerAction, PlayerController};

pub struct CommandLinePlayerController;

impl CommandLinePlayerController {
    pub fn new() -> Self {
        Self {}
    }
}

impl PlayerController for CommandLinePlayerController {
    fn ask_player_action(&self, _: AskAction) -> PlayerAction {
        println!("Awaiting user input for action");

        PlayerAction::Pass
    }
}
