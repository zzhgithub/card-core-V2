use crate::cards::card_def::Card;
use crate::effect::effect_def::Effect;
use mlua::Lua;
use std::collections::HashMap;

pub struct LuaApi {
    /// 保存卡片ID到卡片定义的映射
    pub card_definitions: HashMap<String, Card>,
}

impl LuaApi {
    /// 初始化LuaApi结构体
    pub fn new() -> Self {
        Self {
            card_definitions: HashMap::new(),
        }
    }

    /// 安装Lua API函数到Lua实例
    pub fn install(&mut self, lua: &Lua) -> mlua::Result<()> {
        let mut api_ptr = std::ptr::NonNull::from(self);

        let card_define = lua.create_function_mut(
            move |lua_ctx, (id, callback): (String, mlua::Function)| -> mlua::Result<()> {
                let api = unsafe { api_ptr.as_mut() };
                let builder = crate::cards::card_def::CardBuilder::new().id(id.clone());

                // 创建CardBuilder的AnyUserdata，并传给回调函数
                let builder_ud = lua_ctx.create_userdata(builder)?;

                // 执行回调函数，不期待返回值
                callback.call::<()>(builder_ud.clone())?;

                // 从userdata获取builder
                let builder = builder_ud.take::<crate::cards::card_def::CardBuilder>()?;

                // 调用builder的build方法获取卡片
                let card = builder
                    .build()
                    .map_err(|e| mlua::Error::external(format!("Failed to build card: {}", e)))?;

                // 插入到共享的map中
                api.card_definitions.insert(id, card);

                Ok(())
            },
        )?;

        // 在全局注册创建Effect的函数
        let create_effect_func =
            lua.create_function(|lua_ctx, _: ()| -> mlua::Result<mlua::Value> {
                let effect = Effect::new("Default Description".to_string(), "Custom".to_string());
                let ud = lua_ctx.create_userdata(effect)?;
                Ok(mlua::Value::UserData(ud))
            })?;

        lua.globals().set("card_define", card_define)?;
        lua.globals().set("createEffect", create_effect_func)?;

        Ok(())
    }
}
