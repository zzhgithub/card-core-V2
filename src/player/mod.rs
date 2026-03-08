pub type PlayerId = usize;
pub mod ai_controller;
pub mod ask_action;
pub mod cli_controller;
pub mod player_action;
pub mod zone;

pub use ai_controller::AIPlayerController;
pub use ask_action::AskAction;
pub use cli_controller::CommandLinePlayerController;
pub use player_action::PlayerAction;
pub use zone::PlayerZones;

pub trait PlayerController {
    fn ask_player_action(&self, ask_action: AskAction) -> PlayerAction;
}
