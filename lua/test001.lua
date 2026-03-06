card_define("TEST001", function(builder)
    builder:id("TEST001")
    builder:name("测试卡")
    builder:card_type("character")  -- 人物卡
    builder:attribute("rationality") -- 理性属性 
    builder:category("science")      -- 科学范畴
    builder:cost(3)                -- 花费3点
    builder:attack(100)             -- 攻击力100
    builder:effect("e1", "随机抽取一张策略牌", "draw_random_strategy")
    builder:field("special_ability", "可以跳过一次攻击")
end)