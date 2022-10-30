fn main() {
    // 元组是一种可以组合多种类型数据的数据结构，和python很像
    {
        // 创建一个元组
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        // 类型可自动推导
        let tup = (500, 6.4, 1);
    }
    // 可以用模式匹配解构元组
    {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("The value of y is: {y}");
    }
    // 也可以直接通过.号访问的方式访问其中的元素
    {
        let x = (500, 6.4, "中国人");
        let s = x.2;
        println!("x is {x:?}, x.2 is {s}");
    }
    // 元组可以作为函数的返回值，就像go一样
    {
        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len();
            (s, length)
        }
        println!("{:?}", calculate_length("hello, world".to_string()));
    }
}
