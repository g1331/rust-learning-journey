fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    println!("m1的地址: {:p}", &m1);
    greet(&m1, &m2);
    let s = format!("{}, {}", m1, m2);
    println!("{}", s);

    let m1 = Box::new("Hello");
    println!("m1: {}", m1);
    println!("m1: {}", &m1); // 打印出来相同，为何？自动解引用了

    let x = Box::new(-1);
    let x_abs1 = i32::abs(*x); // 显式解引用
    let x_abs2 = x.abs(); // 隐式解引用
    assert_eq!(x_abs1, x_abs2);

    let r = &x;
    let r_abs1 = i32::abs(**r); // 显式解引用2次
    let r_abs2 = r.abs(); // 隐式解引用2次
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // 显式引用
    let s_len2 = s.len(); // 隐式引用
    assert_eq!(s_len1, s_len2);

    permission_transfer_process();
    can_t_alias_eg();
    variable_references();

    let strings: Vec<String> = vec![String::from("Hello"), String::from("World")];
    let first_string = get_first_string(&strings);
    println!("first_string: {}", first_string);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}", g1, g2);
    // 当Rust释放这个变量所在的stack frame时，会释放box在heap上的数据
    // 这里的g1和g2并不拥有数据，所以外部在heap上的数据不会被释放
    let address_in_g1 = g1 as *const String;
    println!("g1存的地址: {:p}", address_in_g1); // 应该和 &m1 相同
    println!("g1的地址: {:p}", &g1);
}

fn permission_transfer_process() {
    let mut v = vec![1, 2, 3]; // v: RWO
    let num = &v[2]; // v: R ,num: RO, *num: R
    println!("num: {}", *num);
    v.push(4); // push会分配新的内存，将原来的数据拷贝到新内存中再添加新的数据4
    // 此时num指向的地址已经释放了，所以不能再使用num
    // println!("num: {}", *num);

    let x = 0; // x: R O 
    let mut x_ref = &x; // x: R x_ref: R W O 不可变引用
    println!("x_ref: {x_ref}");
    // *x_ref += 1;
    let y = 1;
    x_ref = &y;
    println!("x_ref: {x_ref}");
}

fn can_t_alias_eg() {
    let x = Box::new(1);
    let y = x;
    // println!("x: {}", x); 所有权已经转移
    println!("y: {}", y);

    let r1 = &y;
    let r2: &Box<i32> = &y;
    println!("r1: {r1}, r2: {r2}");
    // &Box<i32> 是对 Box 的引用，指向栈上的 Box 结构体（其中包含了指向堆上 i32 的指针）
    // &Box<i32> 多了一层间接性，需要先访问 Box，再通过 Box 访问实际的 i32 值

    // &i32 直接指向一个 i32 值（在这个情况下是指向堆上的 i32）
    // &i32 直接指向整数值，访问更直接
    let r3: &i32 = &*y; // *y 得到堆上的 i32，&*y 就是 &i32，即指向堆上整数的引用
    println!(
        "r3_p: {:p}, r3_v_p: {:p}, y_p: {:p}, y_v_p: {:p}, value: {}(r3),{}(y)",
        &r3,
        r3,
        &y,
        &*y, // 打印 &*y 等价 y，
        // 因为对 Box 类型使用指针格式说明符 {:p} 时，会自动调用 Box 的 Deref 实现，返回指向其内部数据的指针
        *r3,
        *y // 访问heap上的值，此处*y等价y，Box 类型实现了 Display trait，而在实现时它自动解引用到内部的值
    );

    // 如果函数需要操作 Box 本身，用 &Box<i32>
    // 如果只需要使用整数值，用 &i32 更合适
}

fn variable_references() {
    let mut v = vec![1, 2, 3, 4, 5];
    let num = &mut v[2];
    // let num2 = &mut v[1]; // 可变引用唯一
    let num2 = &*num;
    // *num += 1;  // 被不可变引用借用后，在使用num2期间，num不可改变
    println!("num: {}, num2: {}", num, num2);
    *num += 1;
    println!("num: {}", num);
    println!("vec {:?}", v);
}

fn get_first_string(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    s_ref
}

// 这里不确定返回的引用究竟时谁的，所以无法通过编译，需要生命周期标注
// fn get_first_string_or(strings: &Vec<String>, default: &String) -> &String {
//     if strings.len() > 0 {
//         &strings[0]
//     } else {
//         default
//     }
// }

// 返回无效引用，无F权限
// fn return_a_string() -> &String {
//     let s = String::from("hello");
//     let s_ref = &s;
//     s_ref
// }
