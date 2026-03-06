use crate::lua_api::config::Config;
use crate::lua_api::lua_api::LuaApi;
use mlua::Lua;
use std::fs;
use std::path::Path;

/// 递归加载卡片脚本
pub fn load_cards(lua_api: &mut LuaApi, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let lua = Lua::new();

    // 配置lua环境
    lua_api.install(&lua)?;

    // 从配置中获取脚本目录路径
    let script_dir = Path::new(&config.script_directory);

    // 递归遍历脚本目录
    walk_directory(lua_api, &lua, script_dir)?;

    Ok(())
}

// 递归遍历目录
fn walk_directory(
    lua_api: &mut LuaApi,
    lua: &Lua,
    dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if !dir.exists() {
        eprintln!("警告: 目录不存在: {:?}", dir);
        return Ok(());
    }

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // 检查是否为lua脚本
            if let Some(ext) = path.extension() {
                if ext == "lua" {
                    // 执行Lua脚本
                    match fs::read_to_string(&path) {
                        Ok(content) => match lua.load(&content).exec() {
                            Ok(_) => {}
                            Err(err) => eprintln!("执行脚本错误 {:?}: {}", path, err),
                        },
                        Err(err) => eprintln!("读取脚本文件错误 {:?}: {}", path, err),
                    }
                } else {
                    // 打印警告信息
                    eprintln!("警告: 跳过非lua脚本文件: {:?}", path);
                }
            } else {
                // 打印警告信息
                eprintln!("警告: 跳过非lua脚本文件: {:?}", path);
            }
        } else if path.is_dir() {
            // 递归进入子目录
            walk_directory(lua_api, lua, &path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_load_cards_with_test_script() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let lua_dir = temp_dir.path().join("cardscripts");
        fs::create_dir(&lua_dir).unwrap();

        // 创建测试lua脚本
        let test_script = r#"
card_define("TEST001", function(builder)
    builder:id("TEST001")
    builder:name("测试卡")
    builder:card_type("character")
    builder:attribute("rationality")
    builder:category("science")
    builder:cost(3)
    builder:attack(100)
    builder:effect("e1", "随机抽取一张策略牌", "draw_random_strategy")
    builder:field("special_ability", "可以跳过一次攻击")
end)
"#;
        let test_script_path = lua_dir.join("test001.lua");
        let mut file = fs::File::create(&test_script_path).unwrap();
        file.write_all(test_script.as_bytes()).unwrap();

        // 创建LuaApi实例和配置
        let mut lua_api = LuaApi::new();

        let config = Config::new(lua_dir.to_string_lossy().to_string());

        // 加载卡片
        let result = load_cards(&mut lua_api, &config);
        assert!(result.is_ok());

        // 验证卡片是否正确加载
        assert_eq!(lua_api.card_definitions.len(), 1);
        let card = lua_api.card_definitions.get("TEST001").unwrap();
        assert_eq!(card.id, "TEST001");
        assert_eq!(card.name, "测试卡");
        assert_eq!(card.cost, 3);
    }

    #[test]
    fn test_non_lua_files_warning() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let lua_dir = temp_dir.path().join("cardscripts");
        fs::create_dir(&lua_dir).unwrap();

        // 创建非lua脚本
        let non_lua_script = "This is a test file that is not lua";
        let non_lua_script_path = lua_dir.join("test.txt");
        let mut file = fs::File::create(&non_lua_script_path).unwrap();
        file.write_all(non_lua_script.as_bytes()).unwrap();

        // 创建LuaApi实例和配置
        let mut lua_api = LuaApi::new();
        let config = Config::new(lua_dir.to_string_lossy().to_string());

        // 加载卡片 - 这应该运行而无错误，只是警告
        let result = load_cards(&mut lua_api, &config);
        assert!(result.is_ok());

        // 应该没有卡片被加载
        assert_eq!(lua_api.card_definitions.len(), 0);
    }
}
