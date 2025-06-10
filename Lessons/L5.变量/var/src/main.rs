fn main() {
    let x = 5;
    let x = x + 1; // 重新绑定变量 x
    {
        // 内部作用域
        let x = x * 2; // 在新的作用域中重新绑定变量 x
        println!("The value of x in the inner scope is: {}", x); // 输出 12
    }
    println!("The value of x is: {}", x); // 输出 6

    let x: char = 'A'; // 类型可变
    println!("x1: {}", x);

    let t: f64 = 1.11111;
    println!("t.3f: {:.3}", t);
    let t: f32 = t as f32 + 0.1;
    println!("t f32: {}", t);
    let x: bool = true;
    println!("x: {}", x);

    let my_tuple = ('A', 1, 1.2);
    let tup: (i32, f64, u8) = (500, 16.4, my_tuple.1);
    // 模式匹配
    let (a, b, c) = tup;
    println!("a:{}, b:{}, c:{}", a, b, c);
    let five_hundred = tup.0;
    println!("tup0:{}", five_hundred);

    let my_arr = [1, 2, 3];
    let first = my_arr[0];
    println!("first:{}", first);
    let my_arr_typed: [i32; 3] = [1, 2, 3];
    println!("my_arr_typed0: {}", my_arr_typed[0]);
    let a = [3; 5]; // let a = [3,3,3,3,3];
    println!("a0: {}", a[0]);

    let num = 255;
    println!("Decimal: {}", num); // 输出: Decimal: 255
    println!("Binary: {:b}", num); // 输出: Binary: 11111111
    println!("Octal: {:o}", num); // 输出: Octal: 377
    println!("Hex (lowercase): {:x}", num); // 输出: Hex (lowercase): ff
    println!("Hex (uppercase): {:X}", num); // 输出: Hex (uppercase): FF

    println!("|{:5}|", "abc"); // 输出: |  abc| (默认右对齐)
    println!("|{:<5}|", "abc"); // 输出: |abc  | (左对齐)
    println!("|{:>5}|", "abc"); // 输出: |  abc| (右对齐)
    println!("|{:^5}|", "abc"); // 输出: | abc | (居中对齐)
    println!("|{:.<5}|", "abc"); // 输出: |abc..| (左对齐，用 '.' 填充)

    println!("Signed: {:+}", 10); // 输出: Signed: +10
    println!("Signed: {:+}", -10); // 输出: Signed: -10
    println!("Spaced: {: }", 10); // 输出: Spaced:  10
    println!("Spaced: {: }", -10); // 输出: Spaced: -10

    let x = 42;
    let ptr = &x as *const i32;
    println!("Pointer address: {:p}", ptr); // 输出: Pointer address: 0x... (实际地址)

    println!("{0} {1} {0}", "hello", "world"); // 输出: hello world hello

    println!("{greeting}, {name}!", greeting = "Hello", name = "Rust"); // 输出: Hello, Rust!
}
