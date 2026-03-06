mod cards;
mod config;
mod effect;
mod game;
mod lua_api;

use config::AppConfig;
use lua_api::lua_api::LuaApi;

fn main() {
    // 从TOML文件加载配置
    let config_file_path = "config.toml";
    let app_config = if std::path::Path::new(config_file_path).exists() {
        AppConfig::load_from_file(config_file_path).expect("Failed to load config from TOML file")
    } else {
        // 使用默认配置并保存到文件
        let default_config = AppConfig::new_from_default();
        default_config
            .save_to_file(config_file_path)
            .expect("Failed to save default config to TOML file");
        default_config
    };

    let mut lua_api = LuaApi::new();

    // 使用TOML配置中的脚本目录
    let config = lua_api::config::Config::new(app_config.lua_scripts.script_directory);

    match lua_api::load_cards::load_cards(&mut lua_api, &config) {
        Ok(()) => {
            println!("成功加载卡片定义");
            // 打印ApiLua中的hashmap的值
            println!("加载了 {} 张卡片:", lua_api.card_definitions.len());
            for (id, card) in &lua_api.card_definitions {
                println!(
                    "  - ID: {}, 名称: {}, 类型: {:?}",
                    id, card.name, card.card_type
                );
            }
        }
        Err(e) => {
            eprintln!("加载卡片定义时出错: {}", e);
        }
    }
}
