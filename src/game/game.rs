use crate::game::game_phase::GamePhase;
use crate::player::{PlayerController, PlayerId};

pub struct Game {
    pub current_phase: GamePhase,
    pub players: Vec<Box<dyn PlayerController>>,
    pub current_player_id: PlayerId,
    // 游戏状态和其他相关信息
}
