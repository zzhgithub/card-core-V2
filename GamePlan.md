# Game系统的计划
- [ ] 创建一个独立的player模块
  - [ ] 创建类型别名PlayerId 为usize
  - [ ] 在player模块下单独文件创建AskingPlayer枚举
  - [ ] 在player模块下单独文件创建PlayerAction枚举
  - [ ] 在player模块中定义一个PlayerController的trait。内部只有一个方法。 fn ask_alayer_action(AskingPlayer)->PlayerAction
- [ ] 在Game结构体中，添加游戏阶段字段，和 Vec<Box<PlayerController>> 名称是players