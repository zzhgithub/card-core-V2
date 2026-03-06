use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// 脚本文件夹路径
    pub script_directory: String,
}

impl Config {
    /// 创建新的配置
    pub fn new(script_directory: String) -> Self {
        Self { script_directory }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            script_directory: "cardscripts".to_string(),
        }
    }
}
