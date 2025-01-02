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

数据类型儿
- 标量和复合类型
- Rust 是静态编译语言，在编译时必须知道所有变量的类型
  - 基于使用的值，编译器通常能够推断出他的具体类型
  - 但如果可能得类型比较多，就必须添加类型的标注，否则编译会报错

标量类型
- 一个标量类型代表一个单个的值
- Rust 有四个主要的标量类型：
  - 整数类型
  - 浮点类型
  - 布尔类型
  - 字符类型

整数类型
- 整数类型没有小数部分，每种都分 i 和 u，以及固定的位数
- 无符号类型 u
- 有符号类型 i
- 取值范围：
  - 有符号范围 -2^(n-1) 到 2^(n-1)-1
  - 无符号范围 0 到 2^n-1

浮点类型
- Rust 有两种基础的浮点类型
  - f32, 32位，单精度
  - f64，64位，双精度
- Rust 的浮点类型使用了 IEEE-754 标准来表述
- f64 是默认类型，因为现代 CPU 上 f64 和 f32 的速度差不多，而且精度更高。

布尔类型
- true false

字符类型
- char 代表单个字符
- 字符类型的字面值使用单引号
- 占用 4 个字节大小
- 是 Unicode 标量值，可以表示比 ASCII更多的字符内容：拼音、中日韩文、零长度空白字符、emoji 表情等

复合类型
- 复合类型可以将多个值放在一个类型里
- Rust 提供了两种基础的复合类型；元组（Tuple）、数组

Tuple
- Tuple 可以将多个类型的多个值放在一个类型里
- Tuple 的长度是固定的，一旦声明就无法更改
- 访问 Tuple 的元素
  - 在Tuple变量使用点标记法,后街元素的索引号

数组
- 声明数组
  - 在中括号里, 各值用逗号分开
- 数组的用处
  - 如果想让数组存放在 stack(栈) 上而不是 heap(堆) 上, 或者想保证有固定数量的元素, 这时使用数组更有好处
- 数组没有 Vector 灵活
  - Vector 和数组类似, 它由标准库提供
- 访问数组的元素
  - 数组是 stack 上分配的单个块的内存
  - 可以使用索引来访问数组的元素
  - 如果访问的索引超出了数组的范围, 那么 编译不会报错, 但是运行时会报错. (现在编译时也会报错了(rustc v1.83). error: this operation will panic at runtime)


函数
- 声明函数使用 fn 关键字
- 惯例, 针对函数和变量名, Rust 使用 snake case 命名规范:
  - 所有字母都是小写的
- 函数的参数
  - parameter, argument
- let 不返回值
- 函数的返回值
  - 在 -> 符号后边声明函数返回值的类型, 但是不可以为返回值命名
  - 在 Rust 里面, 返回值就是函数体里面最后一个表达式的值
  - 不加 return 会报错

控制流
- if 表达式
- 与条件相关联的代码块叫做分支 (arm)
- 可选的, 在后边可以加上一个 else 表达式
- 使用 else if 处理多重条件
- 如果使用了多于一个 else if, 那么最好使用 match 来重构代码

循环
- loop 
- while 
- 使用 loop 或 while 来遍历集合, 但是易错且低效
- 使用 for 循环更简洁紧凑, for 遍历可以针对集合中的每个元素来执行一些代码
- 由于 for 循环的安全 简洁性, 是在 Rust 中用的最多的循环方式
- Range
 - 标准库提供
 - 指定一个开始数字和一个结束数字, Range 可以生成它们之间的数字 (不含结束)
 - for number in (1..4).rev() {}
 - rev 方法可以反转 Range
 