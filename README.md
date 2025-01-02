# Rust Learning by Exc

# 

## Day 1

Rust 的优点:

- 运行速度
- 内存安全
- 更好利用多处理器



c/c++ 性能好但是类型和内存不太安全

java 拥有GC能保证内存安全，但是性能不行



擅长领域

- 高性能Web service
- WebAssembly
- 命令行工具
- 网络编程
- 嵌入式编成
- 系统编程



总结:

- 性能

- 安全性

- 无所畏惧的并发



编译和运行是两个单独的步骤:

1. Rust 运行前必须编译，命令为： rustc <源文件名>, 如 rustc main.rs
2. 编译成功后, 会生成一个二进制文件, 在 windows 中还会生成一个 `.pdb` 文件, 里面包含调试信息.
3. rust 是 ahead-of-time 编译的语言, 可以先编译程序, 然后把可执行文件交给别人运行 (无需安装Rust)
4. rustc 只适合简单的 Rust 程序, 复杂的程序要使用 Cargo

Cargo

1. Cargo 是 Rust 的构建系统和包管理工具

2. 安装 Rust 的时候会安装 Cargo (cargo --version)

3. cargo new xxxx 创建项目

   1. 项目名称是 xxxx

   2. 会创建一个新的目录 xxxx

      1. Cargo.toml 
      2. src 目录
         1. main.rs
      3. 初始化了一个Git仓库, .gtiignore
         1. 也可以使用其他的 VCS 或者不使用 VCS, cargo new 的时候使用 --vcs 这个 flag

      

Cargo.toml

- TOML 格式, 是cargo的配置格式
- [packge], 是一个区域标题, 表示下方内容是用来配置包的
  - name, 项目名
  - version, 项目版本
  - authors, 项目作者
  - edition, 使用 Rust 版本
- [dependencies]
  - 另一个区域的开始, 它会列出项目的依赖项
- 在 Rust 里面, 代码的包称作 crate



src/main.rs

- cargo 生成的 main.rs 在 src 目录下
- Cargo.toml 在项目的顶层下
- 源代码都应该在 src 目录下
- 顶层目录可以放置: README, 许可信息, 配置文件和其他与程序源码无关的文件
- 如果创建项目时没有使用 cargo, 也可以把项目转化为使用 cargo: 
  - 把源代码移动到 src 下
  - 创建 Cargo.toml 并填写相应的配置



构建 Cargo 项目: cargo build

- cargo build
  - 创建可执行文件: target/debug/hello_cargo 或 target/debug/hello_cargo.exe (Windows)
  - 运行可执行文件: ./target/debug/hello_cargo 或 ./target/debug/hello_cargo.exe (Windows)
- 第一次运行 cargo build 会在顶层目录生成 cargo.lock 文件
  - 该文件负责追踪项目依赖的精确版本
  - 不需要手动修改该文件



构建和运行 Cargo 项目: cargo run

- cargo run, 编译代码+执行结果
  - 如果之前编译成功过, 并且源码没有改变, 那么就会直接运行二进制文件



cargo check

- cargo check, 检查代码, 确保能够通过编译, 但是不产生任何可执行文件
- cargo check 要比 cargo build 快得多
  - 编写代码的时候可以连续反复的使用 cargo check 检查代码, 提高效率



为发布构建

- cargo build --release
  - 编译时会进行优化
    - 代码会运行得更快, 但是编译时间更长
  - 会在 target/release 而不是 target/debug 生成可执行文件
- 两种配置:
  - 开发
  - 正式发布


变量与可变性
- 声明变量使用 let 关键字
- 默认情况下, 变量是不可变的(Immutable)
- 声明变量时, 在变量前面加上 mut, 就可以使变量可变


变量与常量
- 常量(constant), 常量在绑定值以后也是不可变的, 但是它与不可变的变量有很多区别:
  - 不可以使用 mut, 常量永远都是不可变的.
  - 声明常量使用 const 关键字, 他的类型必须被标注.
  - 常量可以在任何作用域内进行声明, 包括全局作用域.
  - 常量只可以绑定到常量表达式, 无法绑定到函数的调用结果或只能在运行时才能计算出的值.
- 在程序运行期间, 常量在其声明的作用域内一直有效.
- 命名规范: Rust 里常量使用全大写字母, 每个单词之间用下划线分开, 例如: MAX_POINTS
- 例子: const MAX_POINTS:u32 = 100_000; (数字的可读性,使用 _ 来区分每三位数字)

Shadowing(隐藏)
- 可以使用相同的名字声明新的变量，新的变量就会 shadow 之前声明的同名变量。
  - 在后续的代码中，这个变量名代表的就是新的变量。

- shadow 和把变量标记为 mut 是不一样的：
  - 如果不使用 let 关键字，那么重新给非 mut 的变量赋值就会导致编译时错误。
  - 而是用 let 声明的同名新变量，也是不可变的。
  - 使用 let 声明的同名新变量，它的类型可以与之前不同。
