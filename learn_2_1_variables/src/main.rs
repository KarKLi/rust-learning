struct Struct {
    e: i32,
}
fn main() {
    // 你不能修改一个不可变变量
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // 可以将其设置为mut
    {
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }

    // 使用下划线避免rust警告未使用变量。
    {
        let _x = 5;
        let y = 5; // warn
    }

    // 变量可解构
    {
        let (a, mut b): (bool, bool) = (true, false);
        println!("a = {:?}, b = {:?}", a, b);
        b = true;
        assert_eq!(a, b);
    }

    // 还可以解构式赋值（看起来很难）
    {
        let (a, b, c, d, e);
        (a, b) = (1, 2);
        [c, .., d, _] = [1, 2, 3, 4, 5];
        Struct { e, .. } = Struct { e: 5 };
        assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    }

    // 还可以声明常量
    {
        const MAX_POINTS: i32 = 100_000; // 下划线只是用于分隔位数的而已，没有实际影响
    }

    // 变量可以被shadow
    {
        let x = 5;
        // 在main函数的作用域内对之前的x进行遮蔽
        let x = x + 1;
    
        {
            // 在当前的花括号作用域内，对之前的x进行遮蔽
            let x = x * 2;
            println!("The value of x in the inner scope is: {}", x);
        }
        println!("The value of x is: {}", x);
    }
}
