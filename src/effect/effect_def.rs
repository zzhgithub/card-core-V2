/// 效果结构体定义
#[derive(Debug, Clone)]
pub struct Effect {
    /// 效果描述
    pub description: String,
    /// 效果类型
    pub effect_type: String,
}

impl Effect {
    /// 创建新的Effect实例
    pub fn new(description: String, effect_type: String) -> Self {
        Self {
            description,
            effect_type,
        }
    }
}
