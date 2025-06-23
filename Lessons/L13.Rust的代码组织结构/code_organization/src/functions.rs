use crate::models::structs::HousePrice;
// use csv::{ReaderBuilder, Writer}; // 同一级的可以放在一个花括号中
// 引入所有
use csv::*;

pub fn read_csv(path: String) -> Vec<HousePrice> {
    let mut rdr = Reader::from_path(path).unwrap();
    vec![]
}
