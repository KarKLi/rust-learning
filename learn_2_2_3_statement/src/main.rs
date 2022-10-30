fn main() {
    // 函数中的最后一句不带分号的语句为表达式，也是这个函数的返回值
    {
        fn add_with_extra(x: i32, y: i32) -> i32 {
            let x = x + 1;
            let y = y + 1;
            x + y // 表达式
        }
    }
    // 下列这些都是语句，因此不能把语句赋值给变量，只能把表达式赋值给变量
    // let是一个语句，不是一个表达式
    {
        let a = 8;
        let b: Vec<f64> = Vec::new();
        let (a, c) = ("hi", false);
        // let b = (let a =8); // wrong in stable, but OK in unstable
    }
    // 能返回值的，就是一个表达式
    {
        // 下面花括号里的就是一个表达式（或者表达块）
        let y = {
            let x = 3;
            x + 1
        };
        println!("The value of y is: {}", y);
    }
    // 如果表达式不返回任何值，会隐式地返回一个()
    {
        fn ret_unit_type() {
            let x = 1;
            // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
            // 类似三元运算符，在Rust里我们可以这样写
            let y = if x % 2 == 1 {
                "odd"
            } else {
                "even"
            };
            // 或者写成一行
            let z = if x % 2 == 1 { "odd" } else { "even" };
        }
        assert_eq!((),ret_unit_type());
    }
}
