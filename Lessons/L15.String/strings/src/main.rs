fn main() {
    // 创建
    let s = String::new();
    let data = "initial content"; // 字符串字面量
    let s = data.to_string(); // 实现了display的trait具有to_string方法
    let s = "initial content".to_string();
    let s = String::from("initial content");

    // 更新
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str不会获取参数的所有权，它添加的是字符串切片
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {}", s2);
    s.push('!'); // push会获取参数的所有权，它添加的是字符
    println!("{}", s);

    // + 或者 format!
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("{}", s3);
    // println!("{s2}");
    // + 实际上调用
    // fn add(self, s: &str) -> String {
    // 首先会获得s1的所有权，然后将s2的内容拷贝一份并附加到s1后面
    // 如果s1的空间足够容纳s2，则不会发生新的内存分配
    // 如果s1的空间不足，则会申请新的内存，将s1和s2的内容拷贝到新内存中，然后释放旧内存，并将所有权返回给s3

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    // format! 宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    // 切片
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 必须保证切片范围在字符的边界上
    println!("{}", s);
    
    // 遍历
    // 获取Unicode标量值
    for c in "Зд".chars() {
        println!("{c}");
    }
    // 获取原始字节
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
