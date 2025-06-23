fn main() {
    let v: Vec<i32> = Vec::new(); // 创建一个空的vector，必须显式指定类型
    let v = vec![1, 2, 3]; // 创建一个持有值的vector

    let mut v = Vec::new();
    // push：为vector添加元素
    v.push(1);
    v.push(2);
    v.push(3); // 添加元素后类型就确认了
    println!("{:?}", v);

    // 使用值
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // 索引的方式
    println!("The third element is {}", third);
    match v.get(2) {
        // get方法的方式，更安全
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    // let does_not_exist = &v[100]; // panic

    // 遍历修改
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // 遍历的几种方式
    // 1. range
    for i in 0..v.len() {
        println!("{}", v[i]);
    }
    // 2. iter
    for i in v.iter() {
        println!("{}", i);
    }
    // 3. iter_mut
    for i in v.iter_mut() {
        *i += 50;
    }
    println!("{:?}", v);

    // 使用枚举作为元素，这样能够存储不同类型的元素
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
