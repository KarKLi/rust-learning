// mod front_of_house; => 告诉Rust从和front_of_house同名的rust文件中引入这个模块
mod front_of_house;
// 使用crate::front_of_house::hosting，使其可以在代码中直接成为hosting::xxx调用
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}