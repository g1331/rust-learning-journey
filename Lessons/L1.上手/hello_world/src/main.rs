use std::io::stdin;
// The Rust Prelude
// Prelude是常用功能的集合，在Prelude里的东西会被自动的导入到每个rust程序中，无需显式的引入
// 标准库的其他部分需要显式导入，如 std::io::stdin
// `::` 用于访问某个模块中公开可用的api

// 全局作用域

// fn main 是 binary-crate 的入口
fn main() {
    let mut msg = String::new(); // ; 用于结束语句
    println!("Please enter message:");
    stdin().read_line(&mut msg).unwrap();
    println!("Message is {}", msg);
}
