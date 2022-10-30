fn main() {
    // 结构体的定义和C差不多，每个成员也都要有明确的类型
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    // 初始化的时候，每个字段都要赋值，但顺序可以随便
    {
        let user1 = User {
            username: String::from("kark"),
            active: false,
            email: String::from("kark_li@qq.com"),
            sign_in_count: 1,
        };
        println!("{:?}", user1);
    }
}
