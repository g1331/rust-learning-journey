use std::rc::Rc;

fn main() {
    // Fixing an Unsafe Program: Returning a Reference to th Stack
    let value = return_a_string();
    println!("{}", value);
    let value = return_a_string2();
    println!("{}", value);
    let value = return_a_string3();
    println!("{}", value);
    // 用完后计数器为0，此时会释放
    let mut s = String::new();
    return_a_string4(&mut s);
    println!("{}", s);

    // Fixing an Unsafe Program: Not Enough Permissions
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    let full = stringify_name_with_title(&name);
    println!("{}", first);
    print!("{}", full);

    // Fixing an Unsafe Program: Copying vs. Moving Out of a Collection
    // 如果一个值不拥有堆数据，那么它可以在不移动的情况下被复制
    // 一个 i32 不拥有堆数据，因此可以在不移动的情况下被复制
    // 一个 String 拥有堆数据，因此不能再不移动的情况下被复制
    // 一个 &String 不拥有堆数据，因此可以再不移动的情况下被复制
    let v = vec![1, 2, 3];
    let n_ref = &v[0];
    let n = *n_ref;
    println!("{}", n);
    let v = vec![String::from("hello"), String::from("world")];
    // let s_ref = &v[0];
    // let s = *s_ref;
    // 直接使用clone吧
    let s = &v[0].clone();
    print!("{}", s);

    // Fixing an Unsafe Program: Mutating Different Tuple Fields
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = &name.0; // 0不可写了
    // let first = get_first(&name); // 由于get_first函数签名是name其中一个元素的引用，Rust目前还不知道你可能会修改谁，所以不能编译通过
    name.1.push_str(", Esq."); // 1还可以
    println!("{first}, {}", name.1);

    // Fixing an Unsafe Program: Mutating Different Array Elements
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];

    *x += 1;

    let y = &a[2]; // x拥有了a的所有权限，如果不注释下行的代码，此处编译无法通过
    // *x += y;

    println!("{:?}, {y}", a);
}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}

// 不能返回引用，可以直接返回s本身将所有权移交给返回值
fn return_a_string() -> String {
    let s = String::from("hello world");
    s
}

// 返回一个字符串字面量的引用
// 函数签名 -> &'static str 中的 'static 生命周期表示这是一个静态引用，它在整个程序运行期间都有效。
// 字符串字面量 "hello world" 是被直接存储在程序的只读数据段中，所以它具有 'static 生命周期。
fn return_a_string2() -> &'static str {
    "hello world"
}

// 使用 Rust 的引用计数智能指针 Rc 来共享字符串数据
// 这种方式的特点是允许多个所有者共享同一个数据。每次调用 Rc::clone() 时，
// 引用计数会增加，当所有 Rc 指针都被销毁时，数据才会被清理。
// 这对于需要在多个地方共享只读数据的场景特别有用。
// 注意这里返回类型是 Rc<String>，表明返回的是一个引用计数的智能指针，而不是普通的 String 或字符串引用。
fn return_a_string3() -> Rc<String> {
    // 函数通过 Rc::new() 创建了一个新的引用计数指针，它包装了一个堆分配的字符串
    let s = Rc::new(String::from("hello world"));
    // 然后使用 Rc::clone() 来增加引用计数并返回一个新的 Rc 指针，指向相同的数据
    Rc::clone(&s)
}

// 通过可变引用来修改字符串内容，而不是创建新的字符串或返回值
// 函数接收一个可变字符串引用参数 output: &mut String，这意味着函数可以直接修改调用者提供的字符串。
// 使用 replace_range(..) 方法将整个字符串内容替换为 "hello world"
fn return_a_string4(output: &mut String) {
    output.replace_range(.., "hello world");
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

// Fixing an Unsafe Program: Aliasing adn Mutating a Data Structure
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap();
    // let largest = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    // for s in src {
    //     if s.len() > largest.len() {
    //         dst.push(s.clone()); // largest使dst失去了写权限，两种方法解决
    //                              // 要么让largest指向clone后的，要么用一个新变量临时保存
    //                              // 或者直接读len来使用
    //     }
    // }
    // let to_add = src
    //     .iter()
    //     .filter(|s| s.len() > largest.len())
    //     .cloned()
    //     .collect();
    // dst.push(to_add);

    let largest_len = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
