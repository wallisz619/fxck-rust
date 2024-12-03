# Rust Learning by Exc

## Day 1
编译和运行是两个单独的步骤:
1. Rust 运行前必须编译，命令为： rustc <源文件名>, 如 rustc main.rs
2. 编译成功后,会生成一个二进制文件, 在windows中还会生成一个.pdb文件,里面包含调试信息.
3. rust是ahead-of-time编译的语言, 可以先编译程序,然后把可执行文件交给别人运行(无需安装Rust)
4. rustc 只适合简单的Rust程序, 复杂的程序要使用Cargo

