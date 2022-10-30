fn main() {
    // 有些变量是一定要指定类型的，不能自动推导的
    {
        // let guess = "42".parse().expect("not a number!"); // wrong!
        let guess = "42".parse::<i32>().expect("not a number!"); // right!
        println!("guess is {}", guess);
        let guess: i32 = "42".parse().expect("not a number!"); //right!
        println!("guess is {}", guess);
        // 类型可以通过常量后面加下划线和类型指定类型
        let guess = 42_u8;
        println!("guess is {}", guess);
        // 也可以不加下划线
        let guess = 42u8;
        println!("guess is {}", guess);
        // 也可以用as指定类型
        let guess = 42 as u8;
        println!("guess is {}", guess);
    }
    // 必须显式转换类型
    {
        // let v: u16 = 38_u8; // invalid
        let v: u16 = 38_u8 as u16; // valid
    }
    // 不要随便比较浮点数
    {
        //assert!(0.1 + 0.2 == 0.3); // panic
    }
    // 一些浮点数的操蛋事（高精度下，相加也不一定等于字面值）
    {
        let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
        let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
        println!("abc (f32)");
        println!("0.1+0.2: {:x}", (abc.0 + abc.1).to_bits());
        println!("0.3: {:x}", abc.2.to_bits());
        println!();

        println!("xyz (f64)");
        println!("0.1+0.2: {:x}", (xyz.0 + xyz.1).to_bits());
        println!("0.3: {:x}", xyz.2.to_bits());
        println!();

        assert!(abc.0 + abc.1 == abc.2); // ok

        // assert!(xyz.0 + xyz.1 == xyz.2); // panic
    }

    // 非法的数学运算可以返回NaN
    {
        let x = (-42.0_f32).sqrt();
        println!("{}", x); // ok, output NaN

        // assert_eq!(x, x); // panic
        // 可以判断NaN
        if x.is_nan() {
            println!("未定义的数学行为")
        }
    }

    // 一些定义变量时确定类型的方法
    {
        let twenty_one: i32 = 21;
        let twenty_two = 22i32;

        // 同样类型才能运算
        let addition = twenty_one + twenty_two;
        println!("{} + {} = {}", twenty_one, twenty_two, addition);

        // 可以用_分割较长类型
        let one_million: i64 = 1_000_000;
        println!("{}", one_million.pow(2));

        // 定义一个数组，数组里的成员需要为同类型。
        let forty_twos = [42.0, 42f32, 42.0_f32];

        // 打印数组中第一个值，并控制小数位为2位
        println!("{:.2}", forty_twos[0]);
    }
    // 位运算
    {
        let a: i32 = 2;
        let b: i32 = 3;
        println!("(a & b) value is {}", a & b);
        println!("(a | b) value is {}", a | b);
        println!("(a ^ b) value is {}", a ^ b);
        println!("(!b) value is {}", !b);
        println!("(a << b) value is {}", a << b);
        println!("(a >> b) value is {}", a >> b);
        let mut c = a;
        // 除了!以外，都可以使用自赋值
        c <<= b;
        println!("a<<=b value is {}", c);
    }

    // 可以使用序列快速生成连续的数值
    {
        // 遍历[1,5]
        for i in 1..=5 {
            println!("{}", i);
        }
        // 遍历[1,5)
        for i in 1..5 {
            println!("{}", i);
        }
    }
}
