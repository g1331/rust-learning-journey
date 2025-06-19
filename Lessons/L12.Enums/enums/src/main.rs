fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("::1"),
    };

    easy_way();

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    // 如何防止类似空指针使用的报错:
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // error[E0277]: cannot add `Option<i8>` to `i8`
    let sum = x + y.unwrap_or(0); // 非空时能相加，否则使用默认值

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // 使用一个变量表示其他情况，如果不适用变量，可以使用_占位
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max is {}", max);
    }
}

fn easy_way() {
    // 枚举的变体可以存储值
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}

fn route(_ip_type: IpAddrKind) {}

enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    } // 分支中表达式的结果就是match表达式的结果，多行时使用{}
}
