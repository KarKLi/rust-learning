fn main() {
    // 常规的引用是一种指针类型
    {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y); // 需要用*进行解引用
    }
    // 不可变引用相当于一种读者
    {
        fn cal_length(s: &String) -> usize {
            s.len()
        }
        let s1 = String::from("hello");
        let len = cal_length(&s1); // 创建一个s1的不可变引用
        println!("{s1} length is {len}");

        // 不可变引用当然不可以更改值
        // fn change(s: &String) {
        //     s.push_str(", world"); // invalid!
        // }
    }
    // 可变引用相当于一种可读可写的角色，只能同时存在一个可变引用
    {
        fn change(s: &mut String) {
            s.push_str(", world"); // valid!
        }
        let mut s1 = String::from("hello");
        let s2 = &mut s1;
        change(s2); // ok!

        // 同一作用域不可以有多个可变引用
        // change(&mut s1); // invalid!
        // let s3 = &mut s1;
        // println!("{s2},{s3}"); // invalid!

        // 不同作用域下可以有多个可变引用（比如s5和s6）
        let mut s4 = String::from("1");
        {
            let s5 = &mut s4;
            change(s5); // ok!
        }
        let s6 = &mut s4;
        change(s6); // ok! s5 has dropped
    }
    // 不可以同时存在可变引用和不可变引用
    {
        let mut s = String::from("hello");
        let r1 = &s; // ok
        let r2 = &s; // ok
                     // let r3 = &mut s; // no!
                     // println!("{r1},{r2},{r3}");
    }
    // 但有个例外，在不可变引用不用了之后，你可以申请一个可变引用
    {
        let mut s = String::from("hello");
        let r1 = &s; // ok
        let r2 = &s; // ok
        println!("{r1},{r2}");
        let r3 = &mut s; // ok if r1 and r2 not use anymore
        println!("{r3}");
    }
    // 不可以创建悬垂引用
    {
        // 非法，s的生命周期在dangle结束之后就结束被drop了，不可以在没有特殊注解的前提下被延长
        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // }
        // let s = dangle();
        // println!(s);

        // 但你可以转移所有权
        fn not_dangle() -> String {
            let s = String::from("hello");
            s
        }
        let s = not_dangle();
        println!("{}", s);
    }
}
