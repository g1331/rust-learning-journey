use code_organization::functions::read_csv;
use code_organization::models::structs::HousePrice; // 引用路径
// as 关键字，给引用起别名
use code_organization::models::structs::HousePrice as HP;

fn main() {
    crate::m1::m2::method1(); // 绝对路径的方式

    let y = code_organization::models::enums::YesNo::Yes;
    let house_price = HousePrice {
        price: 1000000,
        area: String::from("1000 sqft"),
        bed_rooms: 3,
        main_road: y,
    };
    println!("price: {}", house_price.price);

    let house_price2 = HP {
        price: 1000000,
        area: String::from("1000 sqft"),
        bed_rooms: 3,
        main_road: code_organization::models::enums::YesNo::Yes,
    };
    println!("price: {}", house_price2.price);

    // read_csv(String::from("house_price.csv"));

    // 使用重导出
    code_organization::hosting::add_to_waitlist();
}

mod m1 {
    pub mod m2 {
        // pub使项公开
        pub fn method1() {
            println!("Method 1")
        }
    }
}

mod x1 {
    fn method3() {
        x2::method2();
        self::x2::method2();
    }
    mod x2 {
        pub fn method2() {
            super::super::m1::m2::method1(); // 相对路径
        }
    }
}
