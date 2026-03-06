pub type PlayerId = usize;
pub mod asking_player;
pub mod player_action;
pub use asking_player::AskingPlayer;
pub use player_action::PlayerAction;

pub trait PlayerController {
    fn ask_player_action(&self, asking_player: AskingPlayer) -> PlayerAction;
}
