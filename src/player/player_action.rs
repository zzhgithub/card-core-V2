#[derive(Debug)]
pub enum PlayerAction {
    PlayCard(String),
    Pass,
    UseResource(i32),
    DeclareAttack,
    BlockAttack,
    EndTurn,
}
