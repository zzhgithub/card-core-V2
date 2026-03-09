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