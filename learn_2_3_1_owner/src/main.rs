fn main() {
    // 一个变量在离开它的作用域之后就会被释放
    {
        {
            let s = "hello";
        } // 这里s已经无效了

        // println!("{}",s) // invalid!
        // 如果希望使用不定长字符串，使用String类型
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    }
    // 非基本类型赋值即转移所有权（移动语义）
    {
        let s1 = String::from("hello");
        let s2 = s1; // s1 moved and dropped

        // println!("{}", s1); // invalid!
        println!("{}", s2); // valid

        let s3 = s2.clone(); // explicit copy, s2 still valid.
        println!("{}, {}", s2, s3);
    } // s3 dropped, s2 dropped

    // 基本类型的赋值不是转移所有权（移动语义），而是复制（深拷贝）
    {
        // 所有整数类型都是基本类型，比如i32, u32
        let a: i32 = 5;
        let b = a;
        println!("{a},{b}");

        // bool类型也是
        let a: bool = true;
        let b = true;
        println!("{a},{b}");

        // 浮点类型也是
        let a: f32 = 3.0;
        let b = a;
        println!("{a},{b}");

        // 字符类型也是
        let a: char = 'a';
        let b = a;
        println!("{a},{b}");

        // 元组，当且仅当元组中所有元素都是基本类型的时候是
        let a: (i32, f32) = (1, 3.0);
        let b = a;
        println!("{a:?},{b:?}"); // tuple not implement std::fmt::Display trait

        // 不可变引用T
        let a: &str = "hello";
        let b = a;
        println!("{a},{b}");
    }

    // 函数传非引用参会夺走变量的所有权
    {
        fn takes_ownership(s: String) {}
        let s = String::from("haha");
        takes_ownership(s);
        // println!("{}",s); // invalid! s has moved when call takes_ownership() 
    }

    // 但函数当然也可以返回变量的所有权
    {
        fn transfer_ownership(s: String) -> String {
            s
        }
        let s1 = String::from("haha");
        let s2 = transfer_ownership(s1);
        // println!("{}",s1); // invalid!
        println!("{}",s2); // ok
    }
}
