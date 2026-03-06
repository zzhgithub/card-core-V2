# Game系统的计划
- [x] 创建一个独立的player模块
  - [x] 创建类型别名PlayerId 为usize
  - [x] 在player模块下单独文件创建AskingPlayer枚举
  - [x] 在player模块下单独文件创建PlayerAction枚举
  - [x] 在player模块中定义一个PlayerController的trait。内部只有一个方法。 fn ask_alayer_action(AskingPlayer)->PlayerAction
- [x] 在Game结构体中，添加游戏阶段字段，和 Vec<Box<PlayerController>> 名称是players

- [x] 修改AskingPlayer为AskAction 注释为，请求用户动作的描述