fn read(x: bool) {
    if x {
        println!("x is true")
    }
}

fn main() {
    let x = true;
    read(x);

    let a = 5;
    let mut b = a;
    b += 1;

    // let a = [0; 1_000_000];
    // let b = a; // 浪费内存，因为b的内存是拷贝的a的

    // Box，数据在heap上
    // let a = Box::new([0; 1_000_000]);
    // let b = a;
    make_and_drop();

    let first = String::from("Ferris");
    // let full = add_suffix(first); // push_str创建的内存所有权转移到full上
    // println!("{full}, originally {first}", full = full, first = first);
    // 变量在被移动（move）后不可使用

    let first_clone = first.clone(); // 防止所有权转移，clone会复制一个相同数据的内存
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}", full = full, first = first);

    let s = String::from("hello");
    let s2 = add_suffix2(s);
    println!("{s2}");

    // let s = String::from("hello");
    // let b = false;
    // let s2;
    // if b {
    //     s2 = s;      // 编译器认为所有权可能发生转移，因此编译不通过
    // }
    // println!("{s}");
}

fn make_and_drop() {
    let a_box = Box::new(5);
    let b_box = a_box; // double free
}

fn add_suffix(mut name: String) -> String {
    name.push_str("Jr.");
    // push_str 会尝试在heap创建一个新的内存并且释放原来的内存
    // 然后将组装后的数据放在新的内存中，此时原变量发生了move
    name // 当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
}

fn add_suffix2(mut name: String) -> String {
    name.push_str(" world");
    name
}
