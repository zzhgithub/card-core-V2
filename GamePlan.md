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
- [ ] 游戏开始时初始化每个玩家有5张手卡。
- [ ] 每个玩家的场上有10个位置，五个前场，五个后场。在Game创建时初始化。每个玩家还有一个墓地。