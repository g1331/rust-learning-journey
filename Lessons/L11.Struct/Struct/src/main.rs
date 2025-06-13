// Tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Point2 {
    x: i32,
    y: i32,
}

// Unit-Like Struct
struct AlwaysEqual; // 可以实现trait，简单理解为他们可以有一些行为，但是本身不存数据

#[derive(Debug)] // Copy, Clone
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法定义在结构体的上下文当中，并且第一个参数永远是self（不一定是引用）
    fn area(&self) -> u32 {
        // &self -> self: &Self,Self相当于Rectangle类型的别名
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 第一个参数非self的是关联函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // 添加了 Copy, Clone 后，self会进行复制
    fn max(self, other: Self) -> Self {
        // 获得所有权
        let width = self.width.max(other.width);
        let height = self.height.max(other.height);
        Self { width, height }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        // *self = self.max(other); // 无法通过引用获得所有权，除非添加Copy, Clone
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("Caleb"),
        email: String::from("email@email.com"),
        active: true,
        sign_in_count: 1,
    };
    print_user(&user1);
    user1.sign_in_count = 2;
    print_user(&user1);

    let mut user2 = build_user(String::from("123"), "123");
    print_user(&user2);

    let mut user3 = User {
        username: String::from("test"),
        ..user2
    };
    // 当使用结构体更新语法 ..user2 时，
    // 对于实现了 Copy trait 的字段（如 active 和 sign_in_count），会直接复制值。
    // 但对于没有实现 Copy trait 的字段（如 String 类型的 email），则会发生所有权转移。
    user3.active = true;
    print_user(&user3);
    user2.active = false;
    // print_user(&user2); // 所有权移动，导致此处不能使用

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let mut p = Point2 { x: 0, y: 0 };
    let x = &mut p.x; // 创建对 p.x 的可变借用
    // print_point(&p); // p 并没有失去所有权，只是暂时不能整体使用;print_point(&p) 失败的原因是借用规则冲突
    p.y += 1; // 但是另一个字段不影响
    *x = 1;
    print_point(&p); // 直到x用完

    let rect1 = Rectangle {
        width: 30,
        height: 20,
    };
    println!("{rect1:#?}");

    let rect2 = Rectangle {
        width: 40,
        height: 30,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // 语法糖
    println!(
        "Can rect1 hold rect2? {}",
        Rectangle::can_hold(&rect1, &rect2)
    );
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    let rect3 = Rectangle::square(16);

    let max_rect = rect3.max(rect2);
    // println!("Rect3 is {:#?}", rect3);
}

fn print_point(p: &Point2) {
    println!("{}, {}", p.x, p.y);
}

fn print_user(user: &User) {
    println!("-----------------------------");
    println!("UserName: {}", user.username);
    println!("Email: {}", user.email);
    println!("Active: {}", user.active);
    println!("SignInCount: {}", user.sign_in_count);
    println!("-----------------------------");
}

fn build_user(email: String, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email,
        sign_in_count: 1,
    }
}
struct User {
    active: bool,     // stack
    username: String, // heap
    email: String,    // heap
    sign_in_count: u64, // stack
                      // 引用涉及生命周期，前面的区域以后再来探索吧
                      // username: &String,   // heap
}
