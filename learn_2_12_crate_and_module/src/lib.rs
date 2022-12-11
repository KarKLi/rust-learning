// mod 用于声明模块

// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
mod front_of_house {
    // 如果不为pub，那么就只有serving可以访问它了，其他函数/模块都不能通过front_of_house访问hosting
    pub mod hosting {
        // 如果不为pub，那么就只有seat_at_table可以访问它了，其他函数/模块都不能通过hosting访问add_to_waitlist
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}


//     crate
//  └── eat_at_restaurant
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
// 增加一个可导出的函数
pub fn eat_at_restaurant() {
    // 使用绝对路径调用模块
    crate::front_of_house::hosting::add_to_waitlist();

    // 因为eat_at_restaurant和front_of_house都在crate域下，所以可以使用相对路径调用
    front_of_house::hosting::add_to_waitlist();
}

// 一般建议用绝对路径

// 可以使用super来代表父域（这样就不用从crate开始写起了）
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    
    fn cook_order() {}
}

// 当结构体设置为pub的时候，它的所有字段都是私有的
mod School {
    pub struct Student {
        id: i32,
        name: String,
    }
}

fn Read_student_id(stu: &School::Student) {
    // println!("id {}",stu.id); // invalid! Student.id is private
}

// 当enum设置为pub的时候，它的所有成员都是pub的
mod Operation {
    pub enum Opt {
        Empty,
        Forward(i32),
    }
}

fn Execute_opt(o: &crate::Operation::Opt) {
    match o {
        crate::Operation::Opt::Empty => {},
        crate::Operation::Opt::Forward(step) => {println!("forward by step {}",step)},
    }
}