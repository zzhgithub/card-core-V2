mod cards;
mod config;
mod desk;
mod effect;
mod entity;
mod game;
mod lua_api;
mod player;

use config::AppConfig;
use desk::{DeckConfig, DeckLoader};
use entity::IdGenerator;
use lua_api::lua_api::LuaApi;

fn main() {
    let config_file_path = "config.toml";
    let app_config = if std::path::Path::new(config_file_path).exists() {
        AppConfig::load_from_file(config_file_path).expect("Failed to load config from TOML file")
    } else {
        let default_config = AppConfig::new_from_default();
        default_config
            .save_to_file(config_file_path)
            .expect("Failed to save default config to TOML file");
        default_config
    };

    let mut lua_api = LuaApi::new();

    let config = lua_api::config::Config::new(app_config.lua_scripts.script_directory);

    match lua_api::load_cards::load_cards(&mut lua_api, &config) {
        Ok(()) => {
            println!("成功加载卡片定义");
            println!("加载了 {} 张卡片:", lua_api.card_definitions.len());
            for (id, card) in &lua_api.card_definitions {
                println!(
                    "  - ID: {}, 名称: {}, 类型: {:?}",
                    id, card.name, card.card_type
                );
            }

            // 加载卡组并尝试创建游戏
            match DeckLoader::load_decks("desks") {
                Ok(decks) => {
                    println!("成功加载 {} 个卡组", decks.len());

                    if let Some(example_deck) = decks.get("example_deck") {
                        println!("找到 'example_deck'，卡片数量: {}", example_deck.len());

                        // 创建 ID 生成器
                        let id_generator = IdGenerator::default();

                        // 尝试使用示例卡组创建一个游戏
                        match game::game::Game::new(
                            example_deck.clone(), // player1_deck_ids
                            example_deck.clone(), // player2_deck_ids (for demo)
                            &lua_api,
                            id_generator,
                            &app_config.deck_config,
                        ) {
                            Ok(game) => {
                                println!("游戏创建成功！当前阶段: {:?}", game.current_phase);
                                println!("玩家1卡组实体数量: {}", game.player_decks[0].len());
                                println!("玩家2卡组实体数量: {}", game.player_decks[1].len());
                                println!("总卡片实体数量: {}", game.card_entities.len());
                            }
                            Err(e) => {
                                eprintln!("创建游戏失败: {:?}", e);
                            }
                        }
                    } else {
                        println!("未找到 'example_deck'");
                    }
                }
                Err(e) => {
                    eprintln!("加载卡组失败: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("加载卡片定义时出错: {}", e);
        }
    }
}
