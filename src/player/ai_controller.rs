use crate::player::{AskAction, PlayerAction, PlayerController};

pub struct AIPlayerController {
    difficulty: u8,
}

impl AIPlayerController {
    pub fn new(difficulty: u8) -> Self {
        Self {
            difficulty: difficulty.min(10).max(1),
        }
    }
}

impl PlayerController for AIPlayerController {
    fn ask_player_action(&self, _ask_action: AskAction) -> PlayerAction {
        if self.difficulty >= 7 {
            PlayerAction::DeclareAttack
        } else {
            PlayerAction::Pass
        }
    }
}
