/// 游戏阶段枚举
#[derive(Debug, Clone, PartialEq)]
pub enum GamePhase {
    /// 开始阶段
    Start,
    /// 抽卡阶段
    Draw,
    /// 回收阶段
    Recycle,
    /// 主要阶段1
    MainPhase1,
    /// 战斗阶段
    Battle,
    /// 主要阶段2
    MainPhase2,
    /// 结束阶段
    End,
    GameOver,
}
