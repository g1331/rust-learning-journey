use std::collections::HashMap;
fn main() {
    // 创建
    println!("{}创建{}", "=".repeat(10), "=".repeat(10));
    // 像 vector 一样，哈希 map 将它们的数据储存在堆上
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 就像向量一样，哈希映射将其数据存储在堆上。这个 HashMap 有 String 类型的键和 i32 类型的值。
    // 和向量一样，哈希映射也是同构的：所有的键必须具有相同的类型，所有的值必须具有相同的类型。

    // 读取值
    println!("{}读取值{}", "=".repeat(10), "=".repeat(10));
    // 使用get
    let s = "Green";
    let team_name = String::from(s);
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("{s}'s score is {}", score),
        None => println!("{s}'s score is not found"),
    }
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue's score is {}", score);
    // 使用for
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // 所有权
    // 对于实现 Copytrait 的类型，如 i32，值被复制到散列映射中。
    // 对于像 String 这样的拥有值，这些值将被移动，哈希映射将成为这些值的所有者
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name.clone(), field_value.clone());
    println!("{:?} - {:?}", field_name, field_value);
    map.insert(field_name, field_value);
    // println!("{:?}", field_name);
    //                  ^^^^^^^^^^ value borrowed here after move

    // 更新
    println!("{}更新{}", "=".repeat(10), "=".repeat(10));
    // 覆盖
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    // 使用 entry 方法只在键还没有值的情况下插入
    // entry 方法的返回值是一个名为 Entry 的枚举，它表示一个可能存在也可能不存在的值
    // 如果该键确实存在于哈希映射中，则现有值应保持不变;如果该键不存在，则插入它和它的值
    // or_insert 方法被定义为返回一个对相应 Entry 键的值的可变引用（如果该键存在），
    // 如果不存在，则插入参数作为该键的新值，并返回一个对新值的可变引用
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("{:?}", scores);
    // 根据旧值更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // split_whitespace 方法返回一个迭代器，遍历文本中的值的子切片，子切片之间用空格分隔。
    // or_insert 方法返回指定键的值的可变引用（&mut V）。
    // 在这里，我们将该可变引用存储在 count 变量中，因此为了分配该值，
    // 我们必须首先使用星号（*）解引用 count。
    // 可变引用在 for 循环结束时超出范围，因此所有这些更改都是安全的，并且被借用规则所允许。
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
