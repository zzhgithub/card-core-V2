# Game系统的计划
- [x] 创建一个独立的player模块
  - [x] 创建类型别名PlayerId 为usize
  - [x] 在player模块下单独文件创建AskingPlayer枚举
  - [x] 在player模块下单独文件创建PlayerAction枚举
  - [x] 在player模块中定义一个PlayerController的trait。内部只有一个方法。 fn ask_alayer_action(AskingPlayer)->PlayerAction
- [x] 在Game结构体中，添加游戏阶段字段，和 Vec<Box<PlayerController>> 名称是players

- [x] 修改AskingPlayer为AskAction 注释为，请求用户动作的描述
- [x] 在Game 的impl内创建new方法。传入两个用户的卡组信息（为卡片的id列表。调用检查方法检查卡组是否合法，这些内容在单独的desk模块内），new时需要LuaApi将开组信息的定义复制一份。在Game中存一个对战中的卡片实体，每个实体要有一个独立的id.(需要在new时传递一个id生成器，在必要时生产实体ID)。卡片的实体对象，应该卡片的原始定义和一当前的卡片定义。
- [x] 在confg.toml 中太难desk检查的配置项。卡组默认40张。无核心卡要求，一张卡最对三个重复
- [x] 在src的lua目录中随机创建100张测试用卡的定义，每个卡一个文件，id不同。要求符合规范
- [x] 在src的同级创建desks目录，在desk模块中创建load_desks方法。可以加载定义的全部desk的hashmap key是文件名称，值是desk的内容。在desk定义文件中没有卡片id一行，使用纯文本格式。读取时如果卡片id不存给出报错。提供通过desk名称获取desk的方法。然后Result并给出报错信息
- [x] 在player模块内，以新建的文件实现PlayerController，一个是命令行的实现，等待命令行的输入。令一个是AI的实现，自动回复内容。（内部的具体实现可以先TODO,我将在后面详细描述）
- [x] 在Game创建时传入两个实现，一个命令行实现，一个AI实现。在game中创建run方法。先随机生成先行玩家，然后进行游戏流程。
- [x] 游戏开始时初始化每个玩家有5张手卡。
- [x] 每个玩家的场上有10个位置，五个前场，五个后场。在Game创建时初始化。每个玩家还有一个墓地。


# Effect计划
- [x] 重新设计Effect
pub struct Effect {
  pub trigger: Trigger,              // 时点
  pub optional: bool,                // 是否“可以发动” 
  pub activation_limit: Option<ActivationLimit>, //  触发的限制
  pub conditions: Option<Condition>,    // 触发条件
  pub choices: Vec<Choice>,       // 选择目标
  pub actions: Vec<Action>,          // 实际效果
  pub costs: Option<CostChoice>,       // 费用要求
  }

时点：回合开始时，自己主要阶段， 对方主要阶段，双方主要阶段， 攻击时， 暴露时，破坏时，登场时，回合结束阶段
是否可以发动。如果不是的话，不询问玩家，效果直接生效
触发的限制 一回合一次 同名卡一回合一次
触发条件，需要设计。满足可以在lua内自由组合定义，并可以or和and的逻辑。生成语法树，并可以解析执行。比如：
1.对手手卡大于自己的手卡时才能发动 
2.自己Cost区内属性理性的卡数目（简称理性值）大于3才能发动（登场）

Action 为实际执行的效果。配合好Choice选择目标使用。比如
1.自己抽一张卡。
2. 选择一名玩家增加一点RealPoint
3. 破坏对手场上的理性属性的卡

费用要求。只发动效果除了支付卡面信息的费用外，还需要支付的类似费用的内容。是一个定义，需要用户选择比如：
1.支付场上一张卡发动
2.支付两张手卡发动
3.将两张Cost区的卡丢入墓地发动。