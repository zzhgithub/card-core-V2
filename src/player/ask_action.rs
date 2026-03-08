/// 请求用户动作的描述
#[derive(Debug, Clone)]
pub enum AskAction {
    CurrentPlayer,
    Opponent,
    Both(Vec<String>),
}
