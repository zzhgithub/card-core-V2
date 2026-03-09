#[cfg(test)]
mod tests {
    use card_ai::card_def::CardType; // Access via cards module path that was glob re-exported
    use card_ai::lua_api::lua_api::LuaApi; // Access via lua_api module path that was glob re-exported
    use mlua::Lua;

    #[test]
    fn test_character_card_with_summon_effect() {
        // Create a new Lua API instance
        let mut lua_api = LuaApi::new();

        // Create a Lua context
        let lua = Lua::new();

        // Install the Lua API into the Lua context
        lua_api.install(&lua).expect("Failed to install Lua API");

        // Load the card definition code for a character card with summon effect
        let lua_code = r#"
            card_define("S002-C-001", function(card)
                card:name("英雄卡")
                card:card_type("character") 
                card:attribute("rationality")
                card:category("math")
                card:cost(1)
                
                -- Define the effect when summoned - player draws a card
                local effect = createEffect()
                effect:name("登场效果")
                effect:description("登场时自己抽一张卡")
                effect:trigger("summoned")  -- Triggered when card is summoned
                effect:addDrawCardAction(0, 1)  -- Player 0 draws 1 card
                
                -- Add the effect to the card
                card:effect_from_user_data("e1", effect)  -- Key "e1" stores this effect
            end)
        "#;

        // Execute the Lua card definition code
        assert!(
            lua.load(lua_code).exec().is_ok(),
            "Failed to execute Lua card definition"
        );

        // Check if the card was registered
        assert!(
            lua_api.card_definitions.contains_key("S002-C-001"),
            "Card S002-C-001 was not registered"
        );

        let card = lua_api.card_definitions.get("S002-C-001").unwrap();
        assert_eq!(card.name, "英雄卡");
        // Match against the CardType enum
        match card.card_type {
            CardType::Character => assert!(true),
            _ => panic!("Expected card_type to be Character"),
        }
        assert_eq!(card.cost, 1);

        // Verify the card has the expected effect
        assert!(
            card.effects.contains_key("e1"),
            "Card does not contain e1 effect"
        );

        let effect = card.effects.get("e1").unwrap();
        assert_eq!(effect.name, "登场效果");
        assert_eq!(effect.description, "登场时自己抽一张卡");
    }
}
