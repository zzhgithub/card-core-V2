use crate::cards::card_def::{CardAttribute, CardType};
use crate::player::PlayerId;
use serde::{Deserialize, Serialize};

// 主要效果结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Effect {
    pub trigger: Trigger,                          // 效果何时触发
    pub optional: bool,                            // 是否由玩家选择触发
    pub activation_limit: Option<ActivationLimit>, // 激活频率限制
    pub conditions: Option<Condition>,             // 效果可使用的条件
    pub choices: Vec<Choice>,                      // 目标选择选项
    pub actions: Vec<Action>,                      // 实际效果执行
    pub costs: Option<CostChoice>,                 // 超出卡片成本所需的支付
    pub name: String,                              // 可读名称
    pub description: String,                       // 显示的描述
}

impl Effect {
    pub fn new(description: String, effect_type: String) -> Self {
        // 为了向后兼容旧代码使用此方法名称
        Self {
            trigger: Trigger::Custom(effect_type.clone()),
            optional: false,
            activation_limit: None,
            conditions: None,
            choices: vec![],
            actions: vec![],
            costs: None,
            name: effect_type,
            description,
        }
    }

    pub fn new_detailed(
        name: String,
        description: String,
        trigger: Trigger,
        optional: bool,
    ) -> Self {
        Self {
            trigger,
            optional,
            activation_limit: None,
            conditions: None,
            choices: vec![],
            actions: vec![],
            costs: None,
            name,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Trigger {
    TurnStart(bool),   // bool: true 表示自己的回合, false 表示对手的回合
    OwnMainPhase,      // 在自己的主阶段
    OpponentMainPhase, // 在对手的主阶段
    EitherMainPhase,   // 在任一玩家的主阶段
    AttackPhase,       // 当攻击时
    DefensePhase,      // 当被攻击时
    DamagePhase,       // 在伤害计算时
    Exposed,           // 当卡片暴露时
    Destroyed,         // 当卡片被破坏时
    Summoned,          // 当卡片被召唤/打出时
    EndPhase,          // 回合结束时
    Custom(String),    // 自定义触发条件
    OnDemand,          // 可随时使用的效 果
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivationLimit {
    OncePerTurn,           // 每回合一次
    OncePerGame,           // 每局游戏一次
    OncePerPlayer(String), // 每个独特标识符（如卡片名称）每回合一次
    Limited(u32),          // 特定次数
    PerCombatPhase,        // 每战斗阶段一次
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    // 基础条件
    HandCardCountComparison {
        player: PlayerId,
        comparison: ComparisonOp,
        value: u32,
    },
    FieldCardCountComparison {
        player: PlayerId,
        attribute: Option<CardAttribute>,
        comparison: ComparisonOp,
        value: u32,
    },
    CostZoneCardCountComparison {
        player: PlayerId,
        attribute: Option<CardAttribute>,
        comparison: ComparisonOp,
        value: u32,
    },
    RealPointComparison {
        player: PlayerId,
        comparison: ComparisonOp,
        value: u32,
    },
    HealthComparison {
        player: PlayerId,
        comparison: ComparisonOp,
        value: u32,
    },

    // 组合条件
    And(Box<Condition>, Box<Condition>), // 逻辑与
    Or(Box<Condition>, Box<Condition>),  // 逻辑或
    Not(Box<Condition>),                 // 逻辑非

    // 复杂条件
    CardInZone {
        player: PlayerId,
        zone: ZoneType,
        card_properties: CardPropertyFilter,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ZoneType {
    Hand,
    FieldFront, // 前线位置
    FieldBack,  // 后线位置
    Deck,
    Graveyard,
    CostZone,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardPropertyFilter {
    Attribute(CardAttribute),
    CardType(CardType),
    Name(String),
    SpecificCardId(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComparisonOp {
    Equals(u32),
    GreaterThan(u32),
    LessThan(u32),
    GreaterThanOrEqual(u32),
    LessThanOrEqual(u32),
    NotEquals(u32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Choice {
    SelectAnyCardFromPlayer {
        target: PlayerId,
        zone: ZoneType,
        constraints: Option<CardConstraint>,
        count: u32, // 要选择多少卡片
    },
    SelectSpecificPosition {
        target: PlayerId,
        position_range: PositionRange,
    },
    SelectNumberOfCards {
        target: PlayerId,
        min: u32,
        max: u32,
    },
    ChooseAction {
        options: Vec<String>,
        allow_multiple: bool,
    },
    SelectAttribute {
        attributes: Vec<CardAttribute>,
    },
    NoChoice, // 效果无需选择特定目标
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardConstraint {
    SpecificAttribute(CardAttribute),
    SpecificType(CardType),
    MinCost(u32),
    MaxCost(u32),
    Damaged, // 对于被破坏/损坏的单 位
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PositionRange {
    FrontLine,
    BackLine,
    AnyFieldPosition,
    Specific(Vec<usize>), // 特定战场索引
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    DrawCard(PlayerId, u32),                            // 抽卡
    DestroyCard(TargetSelector),                        // 销毁场地中的选定卡片
    SendCardToGraveyard(TargetSelector),                // 将卡片送入墓地
    AddRealPoint(PlayerId, i32),                        // 添加/移除真实点数
    ChangeCardAttribute(TargetSelector, CardAttribute), // 更改卡片属性
    Heal(PlayerId, u32),                                // 恢复生命
    DealDamage(TargetSelector, u32),                    // 造成伤害
    MoveCard(TargetSelector, ZoneType, u32),            // 在各区域之间移动卡片
    ChangeHealth(TargetSelector, i32),                  // 更改HP
    GainControl(PlayerId, TargetSelector),              // 获取对对手卡片的控制权
    AddCardToHand(PlayerId, String),                    // 向手牌中添加特定卡片
    DiscardCards(PlayerId, u32),                        // 强制弃牌
    GainLife(PlayerId, u32),                            // 増加最大HP
    SetStatus(TargetSelector, StatusModifier),          // 应用状态条件
    Custom(String, Vec<String>),                        // 特殊情况行动
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetSelector {
    ChosenInChoice(usize),    // 从前一个选择中选择目标(参考选择索引)
    AllPlayerCards(PlayerId), // 玩家控制的所有卡片
    TopCards(ZoneType, u32),  // 区域中的前N张卡片
    SpecificCard(u64),        // 特定卡片实体
    PreviousActionTargets,    // 效果序列中之前动作的目标
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusModifier {
    CannotAttack,
    CannotBeAttacked,
    DoubleDamage,
    HalfDamage,
    PreventNextDamage(u32),
    CannotPayCost,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CostChoice {
    PayCardsFromZone {
        zone: ZoneType,
        target: PlayerId,
        required_count: u32,
        constraints: Option<CardConstraint>,
    }, // から特定区域支付カード
    PayRealPoints {
        amount: u32,
    }, // 支付真实点数
    PayLife {
        amount: u32,
    }, // 支付生命点数
    Sacrifice {
        target: TargetSelector,
    }, // 片祭特定卡片
    MultipleCosts {
        options: Vec<Cost>,
    }, // 从多个成本选项中选择一个
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Cost {
    CardsFromZone {
        zone: ZoneType,
        target: PlayerId,
        count: u32,
    },
    RealPoints {
        amount: u32,
    },
    LifePoints {
        amount: u32,
    },
    SpecificCard(u64), // 特定カード必须被支付/牺牲
}
