fn another_function() {
    println!("It works!")
}

fn i32_char_in(value: i32, ch: char) {
    println! {"The value is {}-{}", value, ch};
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn fibonacci_sequence(n: i32) -> i32 {
    let mut cnt = 0;
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    while cnt < n {
        c = a + b;
        a = b;
        b = c;
        cnt += 1;
    }
    c
}

fn main() {
    // ()
    another_function();
    i32_char_in(1, 'N');

    let y = {
        let x = 3;
        x + 1 // 不能添加分号
    };
    println!("The value of y is {}", y);

    let y = plus_one(y);
    println!("The value of y is {}", y);

    println!(
        "Result is {}",
        plus_one({
            let y = 1;
            y + 1
        })
    );

    if y < 5 {
        println!("y < 5");
    } else if y == 5 {
        println!("y is 5");
    } else {
        println!("y > 5");
    }

    let condition = true;
    let _y = if condition {
        5
    } else {
        println!("else");
        6
    };

    let x;
    if condition {
        x = 1;
    } else {
        x = 2;
    }
    println!("x = {}", x);

    let mut count = 0;
    'counting_up: loop {
        count += 1;
        if count == 10 {
            break 'counting_up;
        }
    }

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    while 0 != count {
        println!("a{count}: {}", a[count - 1]);
        count -= 1;
    }

    for elm in a {
        println!("{elm}");
    }

    for number in (0..10).rev() {
        println!("{number}: {}", fibonacci_sequence(number));
    }
}