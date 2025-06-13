fn main() {
    let mut string = String::from("hello world");
    let word = first_word(&string);
    string.clear();
    println!("{}", word);

    let mut string = String::from("hello world");

    let hello = &string[0..5];
    let world = &string[6..11];
    let s2 = &string;

    let len = string.len();
    let slice = &string[3..len];
    let slice = &string[3..]; // 切到最后

    let slice = &string[0..]; // 从头切到最后
    let slice = &string[..]; // 从头切到最后

    let word = first_word_slice(&string);
    // string.clear(); // 使用切片防止意外行为
    println!("{}", word);

    let mut s = "hello world";
    s = "123";
    let s2 = &mut s;
    *s2 = "12";

    let word = first_word_str(&string);
    let word = first_word_str("hello world");

    array_slice();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // immutable borrow
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 支持String或者字面量
// Rust 的强制解引用转换（Deref coercion）
// 意味着 String 可以被自动转换为 &str。
// 所以当函数接收 &str 参数时，可以传入 &String，编译器会自动进行转换
// 但是反过来不行，因为 str 是一个不定长的字符串切片，无法自动转换为 String（这需要在堆上分配内存）。
// 所以当函数期望 &String 参数时，不能传入 &str。
// 因此，一般推荐使用 &str 作为函数参数类型，这样可以同时接受 String 和字符串字面量，提高代码的灵活性和复用性。
fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
