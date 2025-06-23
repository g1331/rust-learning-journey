pub mod functions;
pub mod models;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting; // 重导出

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
