pub type PlayerId = usize;
pub mod ask_action;
pub mod player_action;
pub use ask_action::AskAction;
pub use player_action::PlayerAction;

pub trait PlayerController {
    fn ask_player_action(&self, ask_action: AskAction) -> PlayerAction;
}
