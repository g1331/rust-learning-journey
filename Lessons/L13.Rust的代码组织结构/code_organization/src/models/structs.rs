use crate::models::enums::YesNo; // 方便模块内复用

// 字段不加pub就是私有的
pub struct HousePrice {
    pub price: u32,
    pub area: String,
    pub bed_rooms: u32,
    pub main_road: YesNo,
}
