use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub lua_scripts: LuaScriptsConfig,
    // 将来的其他配置可以添加到这里,如游戏参数等
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LuaScriptsConfig {
    pub script_directory: String,
}

impl Default for LuaScriptsConfig {
    fn default() -> Self {
        Self {
            script_directory: "lua".to_string(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            lua_scripts: LuaScriptsConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    pub fn new_from_default() -> Self {
        Self::default()
    }
}
