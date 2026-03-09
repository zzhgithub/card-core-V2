use crate::effect::effect_def::Effect;
use mlua::UserData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 卡片类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardType {
    /// 人物卡
    Character,
    /// 策略卡
    Strategy,
    /// 物品卡
    Item,
    /// 传奇卡
    Legend,
}

/// 卡片属性枚举（理性、神性、灵性）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardAttribute {
    /// 理性
    Rationality,
    /// 神性
    Divinity,
    /// 灵性
    Spirituality,
}

/// 卡片范畴枚举
#[derive(Debug, Clone, PartialEq)]
pub enum CardCategory {
    /// 数学
    Math,
    /// 科学
    Science,
    /// 文艺
    Literature,
    /// 哲学
    Philosophy,
    /// 神秘
    Mystery,
}

/// 人物卡特有属性
#[derive(Debug, Clone)]
pub struct CharacterCardSpecifics {
    /// 攻击力
    pub attack: u32,
}

/// 策略卡特有属性
#[derive(Debug, Clone)]
pub enum StrategyCardAttribute {
    /// 普通
    Normal,
    /// 诡计
    Trickery,
    /// 瞬时
    Instant,
}

#[derive(Debug, Clone)]
pub struct StrategyCardSpecifics {
    /// 策略属性：普通、诡计、瞬时
    pub strategy_attribute: StrategyCardAttribute,
}

impl StrategyCardSpecifics {
    pub fn new(strategy_attr: StrategyCardAttribute) -> Self {
        Self {
            strategy_attribute: strategy_attr,
        }
    }
}

/// 物品卡特有属性
#[derive(Debug, Clone)]
pub enum ItemCardAttribute {
    /// 普通
    Normal,
    /// 存留
    Retention,
}

#[derive(Debug, Clone)]
pub struct ItemCardSpecifics {
    /// 物品属性：普通、存留
    pub item_attribute: ItemCardAttribute,
}

// 为不同卡类型定义特有属性的容器
#[derive(Debug, Clone)]
pub enum CardSpecificData {
    Character(CharacterCardSpecifics),
    Strategy(StrategyCardSpecifics),
    Item(ItemCardSpecifics),
    Legend, // 传奇卡特有数据待定义
}

/// 卡片结构体定义
#[derive(Debug, Clone)]
pub struct Card {
    /// 卡片ID
    pub id: String,
    /// 卡片类型
    pub card_type: CardType,
    /// 卡片属性（理性、神性、灵性）
    pub attribute: CardAttribute,
    /// 卡片名称
    pub name: String,
    /// 卡片字段（自动属性）
    pub fields: HashMap<String, String>, // 可能需要更具体的类型定义
    /// 卡片范畴（数学、科学、文艺、哲学、神秘）
    pub category: CardCategory,
    /// 费用
    pub cost: u32,
    /// 效果：HashMap key为e1,e2 以此类推，value为Effect效果对象
    pub effects: HashMap<String, Effect>,
    /// 特有属性数据
    pub specific_data: CardSpecificData,
}

impl Card {
    /// 创建基本卡片实例的构造函数
    pub fn new(
        id: String,
        card_type: CardType,
        attribute: CardAttribute,
        name: String,
        category: CardCategory,
        cost: u32,
    ) -> Self {
        let specific_data = match card_type {
            CardType::Character => {
                CardSpecificData::Character(CharacterCardSpecifics { attack: 0 })
            }
            CardType::Strategy => CardSpecificData::Strategy(StrategyCardSpecifics {
                strategy_attribute: StrategyCardAttribute::Normal,
            }),
            CardType::Item => CardSpecificData::Item(ItemCardSpecifics {
                item_attribute: ItemCardAttribute::Normal,
            }),
            CardType::Legend => CardSpecificData::Legend,
        };

        Card {
            id,
            card_type,
            attribute,
            name,
            fields: HashMap::new(),
            category,
            cost,
            effects: HashMap::new(),
            specific_data,
        }
    }

    /// 添加效果到卡片
    pub fn add_effect(&mut self, key: String, effect: Effect) {
        self.effects.insert(key, effect);
    }
}

impl UserData for Card {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_id", |_, this, ()| Ok(this.id.clone()));
        methods.add_method("get_name", |_, this, ()| Ok(this.name.clone()));
        methods.add_method("get_type", |_, this, ()| {
            Ok(match this.card_type {
                CardType::Character => "character",
                CardType::Strategy => "strategy",
                CardType::Item => "item",
                CardType::Legend => "legend",
            }
            .to_string())
        });
    }
}

// CardBuilder 实现
#[derive(Debug, Clone, Default)]
pub struct CardBuilder {
    id: Option<String>,
    card_type: Option<CardType>,
    attribute: Option<CardAttribute>,
    name: Option<String>,
    category: Option<CardCategory>,
    cost: u32,
    attack: u32,                                       // 仅用于人物卡
    strategy_attribute: Option<StrategyCardAttribute>, // 仅用于策略卡
    item_attribute: Option<ItemCardAttribute>,         // 仅用于物品卡
    effects: HashMap<String, Effect>,
    fields: HashMap<String, String>,
}

impl CardBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            card_type: None,
            attribute: None,
            name: None,
            category: None,
            cost: 0,
            attack: 0,
            strategy_attribute: None,
            item_attribute: None,
            effects: HashMap::new(),
            fields: HashMap::new(),
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn card_type(mut self, card_type: CardType) -> Self {
        self.card_type = Some(card_type);
        self
    }

    pub fn attribute(mut self, attribute: CardAttribute) -> Self {
        self.attribute = Some(attribute);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn category(mut self, category: CardCategory) -> Self {
        self.category = Some(category);
        self
    }

    pub fn cost(mut self, cost: u32) -> Self {
        self.cost = cost;
        self
    }

    pub fn attack(mut self, attack: u32) -> Self {
        self.attack = attack;
        self
    }

    pub fn strategy_attribute(mut self, attr: StrategyCardAttribute) -> Self {
        self.strategy_attribute = Some(attr);
        self
    }

    pub fn item_attribute(mut self, attr: ItemCardAttribute) -> Self {
        self.item_attribute = Some(attr);
        self
    }

    pub fn effect(mut self, key: String, effect: Effect) -> Self {
        self.effects.insert(key, effect);
        self
    }

    pub fn field(mut self, key: String, value: String) -> Self {
        self.fields.insert(key, value);
        self
    }

    pub fn build(self) -> Result<Card, String> {
        let id = self.id.ok_or("Card ID is required")?;
        let card_type = self.card_type.ok_or("Card type is required")?;
        let attribute = self.attribute.ok_or("Card attribute is required")?;
        let name = self.name.ok_or("Card name is required")?;
        let category = self.category.ok_or("Cards category is required")?;

        let specific_data = match card_type {
            CardType::Character => CardSpecificData::Character(CharacterCardSpecifics {
                attack: self.attack,
            }),
            CardType::Strategy => {
                let strategy_attr = self
                    .strategy_attribute
                    .unwrap_or(StrategyCardAttribute::Normal);
                CardSpecificData::Strategy(StrategyCardSpecifics {
                    strategy_attribute: strategy_attr,
                })
            }
            CardType::Item => {
                let item_attr = self.item_attribute.unwrap_or(ItemCardAttribute::Normal);
                CardSpecificData::Item(ItemCardSpecifics {
                    item_attribute: item_attr,
                })
            }
            CardType::Legend => CardSpecificData::Legend,
        };

        Ok(Card {
            id,
            card_type,
            attribute,
            name,
            fields: self.fields,
            category,
            cost: self.cost,
            effects: self.effects,
            specific_data,
        })
    }
}

impl UserData for CardBuilder {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("id", |_, builder, id: String| {
            builder.id = Some(id);
            Ok(())
        });

        methods.add_method_mut("name", |_, builder, name: String| {
            builder.name = Some(name);
            Ok(())
        });

        methods.add_method_mut("card_type", |_, builder, card_type_str: String| {
            let card_type = match card_type_str.as_str() {
                "character" => CardType::Character,
                "strategy" => CardType::Strategy,
                "item" => CardType::Item,
                "legend" => CardType::Legend,
                _ => return Err(mlua::Error::external("Invalid card type")),
            };
            builder.card_type = Some(card_type);
            Ok(())
        });

        methods.add_method_mut("attribute", |_, builder, attr_str: String| {
            let attribute = match attr_str.as_str() {
                "rationality" => CardAttribute::Rationality,
                "divinity" => CardAttribute::Divinity,
                "spirituality" => CardAttribute::Spirituality,
                _ => return Err(mlua::Error::external("Invalid attribute")),
            };
            builder.attribute = Some(attribute);
            Ok(())
        });

        methods.add_method_mut("category", |_, builder, category_str: String| {
            let category = match category_str.as_str() {
                "math" => CardCategory::Math,
                "science" => CardCategory::Science,
                "literature" => CardCategory::Literature,
                "philosophy" => CardCategory::Philosophy,
                "mystery" => CardCategory::Mystery,
                _ => return Err(mlua::Error::external("Invalid category")),
            };
            builder.category = Some(category);
            Ok(())
        });

        methods.add_method_mut("cost", |_, builder, cost: u32| {
            builder.cost = cost;
            Ok(())
        });

        methods.add_method_mut("attack", |_, builder, attack: u32| {
            builder.attack = attack;
            Ok(())
        });

        methods.add_method_mut("strategy_attribute", |_, builder, attr_str: String| {
            let strategy_attr = match attr_str.as_str() {
                "normal" => StrategyCardAttribute::Normal,
                "trickery" => StrategyCardAttribute::Trickery,
                "instant" => StrategyCardAttribute::Instant,
                _ => return Err(mlua::Error::external("Invalid strategy attribute")),
            };
            builder.strategy_attribute = Some(strategy_attr);
            Ok(())
        });

        methods.add_method_mut("item_attribute", |_, builder, attr_str: String| {
            let item_attr = match attr_str.as_str() {
                "normal" => ItemCardAttribute::Normal,
                "retention" => ItemCardAttribute::Retention,
                _ => return Err(mlua::Error::external("Invalid item attribute")),
            };
            builder.item_attribute = Some(item_attr);
            Ok(())
        });

        methods.add_method_mut(
            "effect",
            |_, builder, (key, description, effect_type): (String, String, String)| {
                let effect = Effect::new(description, effect_type);
                builder.effects.insert(key, effect);
                Ok(())
            },
        );

        methods.add_method_mut("field", |_, builder, (key, value): (String, String)| {
            builder.fields.insert(key, value);
            Ok(())
        });
    }
}
