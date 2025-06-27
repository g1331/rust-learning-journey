use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::num::ParseIntError;
use std::{fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");
    // 成功：返回 Ok(File)
    // 失败：返回 Err(错误信息)
    println!("{greeting_file_result:?}");

    // 处理错误，如果文件不存在则创建
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // 一般用expect
    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file = File::open("hello.txt").expect("File not found");

    // panic!("Errors!");

    read_username_from_file();
    read_username_from_file_2();
    read_username_from_file_3();
    read_username_from_file_4();

    Ok(())
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, std::io::Error> {
    let mut username_file = File::open("hello.txt")?; // 成功就往下走，失败则返回错误
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, std::io::Error> {
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // 利用标准库书写为更简洁的方式
    fs::read_to_string("hello.txt")
}

// 返回自定义错误
#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    ParseInt(std::num::ParseIntError),
    // 其他错误类型
    Other(String),
}

impl From<io::Error> for MyError {
    fn from(value: io::Error) -> MyError {
        MyError::Io(value)
    }
}

impl From<ParseIntError> for MyError {
    fn from(value: ParseIntError) -> MyError {
        MyError::ParseInt(value)
    }
}

fn read_username_from_file_4() -> Result<String, MyError> {
    let mut username = String::new();
    let file = File::open("hello.txt")?.read_to_string(&mut username)?;
    let num: i32 = "55".parse()?;
    Ok(username)
}

fn last_char_of_first_lint(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
