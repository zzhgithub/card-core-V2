# AGENTS.MD - 代理编码规范

## 构建/测试/Lint 命令

### Rust 项目命令:
```bash
# 编译项目
cargo build

# 编译并检查所有代码
cargo check

# 格式化代码
cargo fmt
rustfmt src/**/*.rs --edition 2021

# 检查代码错误和警告
cargo clippy
cargo clippy --all-targets --all-features

# 运行所有测试
cargo test

# 运行单个测试文件
cargo test -- --test-threads=1

# 运行特定测试 (替换 test_function_name 为你想要的测试名)
cargo test test_function_name

# 运行单个测试文件
cargo test -p package_name -- test_function_name
```

## 代码风格与约定

### 通用规范:
- 使用标准的 Rust 代码格式化工具 (`cargo fmt`) 自动格式化
- 遵循 Rust 命名约定 (snake_case 用于函数和变量，PascalCase 用于类型和特质)
- 注释遵循 Rust 的文档约定
- 避免冗余代码，尽量复用现有功能

### Imports 导入约定:
- 总是使用 `use` 语句在模块开头导入依赖项
- 按字典顺序排序导入
- 避免使用 `use *`; 尽可能精确地导入特定项目
- 分离标准库、外部库和内部模块的导入块

### 类型定义:
- 优先使用描述性的、有意义的类型名
- 利用枚举（enum）表达互斥选项
- 优先创建新的类型而非简单地传递原生类型（例如，使用 newtype 模式）

### 命名规则:
- 函数和变量名使用 snake_case
- 结构体、枚举和特质名使用 PascalCase
- 常量名使用 SCREAMING_SNAKE_CASE
- 测试函数保持有意义但简洁的命名

### 代码结构和组织:
- 保持函数简短且专注于单一功能
- 将相关结构和功能组合到模块中
- 文件名通常应与模块名相同
- 使用有意义的模块层次结构

### 错误处理:
- 在适当的场合使用 Result 和 Option 类型
- 提供清晰的错误消息
- 实现适当的错误传播（通过 ? 操作符）
- 避免使用 panic! 和 unwrap() 在可恢复错误情况下

### 特殊说明:
- 对于卡片相关的数据结构，请遵循卡片项目的定义:
  - `CardType`: 包括人物卡、策略卡、物品卡、传奇卡
  - `CardAttribute`: 包括理性、神性、灵性三类属性
  - `CardCategory`: 分为数学、科学、文艺、哲学、神秘五类
- 遵循 README 中描述的游戏机制和卡片设计原则
- 所有卡片效果应存储在 HashMap 中，键名为 e1, e2 依次类推，值为 Effect 对象

### 性能最佳实践:
- 优先使用迭代器适配器链而非多步操作
- 避免不必要的克隆，特别是对于大的数据结构
- 考虑使用 `Cow<str>` 或其他智能指针提高性能

### 安全考虑:
- 避免使用 unsafe 语句，除非经过严格的代码审查
- 验证用户输入，防止无效数据破坏游戏状态
- 确保访问集合元素时不会出现越界错误

### 提交注意事项:
- 提交消息应清晰描述更改内容及其原因
- 确保提交的代码可通过所有现有的 CI 检查和测试
- 如果进行了重大重构，需确保更新相应的单元测试