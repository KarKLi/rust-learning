fn main() {
    // 在rust中，&str和String都代表字符串
    // &str是对字面值字符串的借用，是在栈上的。而String是动态字符串，是在堆上的
    // &str是不可以隐式转换为String的。
    {
        fn greet(name: String) {
            println!("Hello, {name}")
        }
        let my_name = "Pascal";
        // greet(my_name); // invalid!
        greet(String::from(my_name)); // valid
        greet(my_name.to_string()) // valid
    }
    // 对于字符串来说，字符串的切片就是对String某一部分或&str的引用
    {
        let my_name = "Pascal";
        let pas = &my_name[..3];
        println!("{pas}");
        let my_name_string = my_name.to_string();
        // 语法和for里面的range是一样的
        let pas = &my_name_string[..3];
        let asc = &my_name_string[1..4];
        let asca = &my_name_string[1..=4];
        println!("{pas}, {asc}, {asca}");
        // 也可以截取完整的切片
        let pascal = &my_name_string[..];
        println!("{pascal}");
        // 切片对UTF-8字符来说是很危险的
        let chinese = "中国人";
        // let zhong = &chinese[0..2]; // invalid! 中 has 3 bytes
        // println!("{zhong}");
    }
    // 无论是切&str还是切String，返回的都是&str
    {
        fn first_word(s: &String) -> &str {
            &s[..1]
        }
        let s = String::from("hello, world");
        let word = first_word(&s);
        // s.clear(); // wrong! word is borrowed from s, and no variable borrow can be created when using clear()
        println!("The first word is {word}");
    }
    // 其他集合类型也可以切
    {
        let a = [1,2,3,4,5];
        let slice = &a[1..3];
        assert_eq!(slice,&[2,3]);
    }
    // String和&str是可以互相转换的
    {
        fn say_hello(s: &str) {
            println!("{}",s);
        }
        let s = String::from("hello, world");
        // String可以隐式转为&str
        say_hello(&s);
        say_hello(&s[..]);
        // 也可以显式转换
        say_hello(s.as_str())
    }
    // 一些String的其他特性
    {
        // String不允许索引取值
        let s = String::from("hello");
        // let c = s[0]; // invalid
        // 切片也很危险
        // let s = String::from("नमस्ते");
        // println!("{}",&s[..2]); // invalid!
        // String可以任意地往里面塞字符或字符串，是mut的就行了
        let mut s = String::from("hello ");
        s.push('r');
        println!("{s}");
        s.push_str("ust!");
        println!("{s}");

        // String可以insert，但是对于utf-8字符串来说，是比较危险的
        let mut s = String::from("Hello rust!");
        s.insert(5, ',');
        println!("插入字符 insert() -> {}", s);
        s.insert_str(6, " I like");
        println!("插入字符串 insert_str() -> {}", s);
        println!("{s}");
        // let mut s = "中国人".to_string();
        // s.insert(1, '1'); // panic!
        // println!("{s}");

        // 无论是String还是&str，都可以执行replace操作，replace会返回一个新的String
        let s = String::from("hello, world");
        let new = s.replace("hello", "HELLO");
        println!("{new}");
        let new = "hello, world".replace("hello", "HELLO");
        println!("{new}");
        // replacen是替换n个符合模式的字符串，不赘述

        // replace_range可以范围替换字符串，不过要注意，这也不是utf-8安全的
        // replace_range是原地替换，所以只能作用于mut String
        let mut s = String::from("hello");
        s.replace_range(0..1, "H");
        println!("{s}");
        // let mut s = String::from("中国");
        // s.replace_range(0..1, "美"); // panic!
        // println!("{s}");

        // String的所有删除操作都是原地的，因此必须是mut

        // pop会返回一个Option类型，如果字符串已经为空了，这个Option就会是None
        // 这个操作是utf8安全的
        let mut s = "pop 哈!".to_string();
        let p1 = s.pop();
        println!("{p1:?}"); // 因为p1是Option类型，所以不能直接打印
        let p1 = s.pop();
        println!("{p1:?}");
        let mut s = "".to_string();
        let p1 = s.pop();
        println!("{p1:?}"); // should be None

        // remove，删除并返回字符串指定位置的字符，起始位置必须是合法的utf-8字符边界，因此也是一个utf-8不安全的函数
        let mut s = "测试remove".to_string();
        let c = s.remove(0); // 这里会返回一个utf-8字符
        println!("{c}");
        // let c = s.remove(1); // panic!
        // println!("{c}");

        // clear，清空字符串
        let mut s = "你好".to_string();
        s.clear();
        println!("{s}");

        // 连接字符串，有很多有意思的方法

        // 可以用+号
        let s1 = String::from("hello ");
        let s2 = String::from("world");
        let result = s1 + &s2; // 必须是s2的不可变借用，此时s1已经无效了
        println!("{result}"); // ok
        // println!("{s1}"); // invalid

        // 可以基于mut String直接+=
        let mut s = String::from("result");
        s += ".";
        println!("{s}");

        // 如果不想转移走任何东西的所有权，又不想用mut，可以使用format!
        let s1 = String::from("hello");
        let s2 = " world.";
        let s3 = format!("{}{}",s1,s2);
        println!("{s1}+{s2}={s3}");
    }

    // 字符串包含双引号的解决方法
    {
        // "android", a new tech.
        // 使用r#作为带有双引号字符串的开头，#结尾，就可以了
        let s = r#""android", a new tech."#;
        // 如果字符串里面本身有#，那么只需要保证外面的首尾#比里面的多就行了
        let s = r###""hello"##world"###;
        println!("{s}");
        // 还不如直接反斜杠转义呢
        let s = "\"android\", a new tech";
        println!("{s}");
    }

    // 遍历字符串中的utf-8字符可以使用chars方法（String和&str均适用）
    {
        let s = "中国人नमस्ते";
        for c in s.chars() {
            println!("{c}");
        }
    }
    // 遍历字节可以使用bytes方法
    {
        let s = "中国人नमस्ते";
        for c in s.bytes() {
            println!("{c}");
        } 
    }
}
