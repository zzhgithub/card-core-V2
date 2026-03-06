/// 请求用户动作的描述
pub enum AskAction {
    CurrentPlayer,
    Opponent,
    Both(Vec<String>),
}
