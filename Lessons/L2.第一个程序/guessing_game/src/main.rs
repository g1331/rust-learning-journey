use rand::Rng;
use std::cmp::Ordering;
use std::io;
// pub trait Rng
// trait
// 在 Rust 中，trait（特征）是一种定义共享行为的机制。它类似于其他语言中的“接口”，用于规定类型必须实现哪些方法。通过 trait，可以实现多态和代码复用。
//
// 简单例子：
//
// ```rust
// trait Speak {
//     fn speak(&self);
// }
//
// struct Dog;
// impl Speak for Dog {
//     fn speak(&self) {
//         println!("汪汪！");
//     }
// }
// ```
//
// 这样，任何实现了 `Speak` trait 的类型都可以调用 `speak` 方法。

fn main() {
    // println! - 宏（macros）
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // 等价于1..101
    // println!("The secret number is {}.", secret_number);

    loop {
        println!("Please input your guess.");

        // mut - 可变变量
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // &mut - 可变引用
            // Result - 结果类型
            // --- OK(u) - 成功
            // --- Err - 错误
            .expect("Failed to read line");
        // {} - 占位符
        println!("You guessed: {}", guess);

        // 转换为u32
        // 遮蔽：这里使用了相同的变量名，但是实际上是不同的变量 shadowing
        // 显式指定类型 :u32 ，此时secret_number会自动推断为u32，因为下方在做比较（强大的推断系统）
        // trim：返回删除了前导和尾随空格的字符串切片。
        // parse：将此字符串切片解析为另一种类型。
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue; // 继续下一次循环
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
