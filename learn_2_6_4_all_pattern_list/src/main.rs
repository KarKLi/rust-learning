fn main() {
    // 你完全可以在模式匹配中使用一些酷炫的语法

    // 在match中使用序列（仅数字/字符类型可用）
    {
        let x = 5;
        match x {
            1..=5 => println!("one through five"), // 只可以使用字符和数字类型作为序列匹配，因为这有这些类型能在编译期确定序列里面的全部值
            _ => println!("something else"),
        }
        let x = 'c';
        match x {
            'a'..='j' => println!("early ascii letter"),
            'k'..='z' => println!("late ascii letter"),
            _ => println!("something else"),
        }
    }

    // 解构结构体
    {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p; // a = 0,b = 7
        println!("({},{})", a, b);
        // 也可以进行同名解构
        let Point { x, y } = p;
        println!("({},{})", x, y);
    }

    // 可以解构嵌套结构
    {
        // 解构嵌套enum
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 166, 255));
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b);
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!(
                    "Change the color to hue {}, saturation {}, and value {}",
                    h, s, v
                );
            }
            _ => (),
        }
        // 解构嵌套元组
        struct Point {
            x: i32,
            y: i32,
        }
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }

    // 和Go一样，使用_忽略你不用的函数参数
    {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }
        foo(3, 4);
    }

    // 使用_在match中忽略你不要的值
    {
        let numbers = (1, 2, 3, 4, 5);
        match numbers {
            (first, _, third, _, fifth) => {
                println!("{},_,{},_,{}", first, third, fifth);
            }
        }
    }

    // 在if let中使用_会导致变量不会被移动
    {
        let s = Some(String::from("hello"));
        if let Some(_) = s {
            println!("a string!");
        }
        println!("s: {:?}", s);
    }

    // 用..忽略剩下的值（这样就不用写多个_了）
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point { x: 0, y: 0, z: 0 };
        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
        // 还可以忽略部分值（但不能有歧义，只能是忽略除首、尾、首尾这三种情况）
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("first {}, last {}", first, last);
            }
        }
    }

    // match guard是一种可以在模式匹配表达式里带上if的语法
    {
        let num = Some(4);
        match num {
            // 次序很重要，match是从上到下进行匹配的，所以第二个match语句块并不会生效
            Some(x) => println!("{}", x),
            Some(x) if x < 5 => println!("less than 5: {}", x),
            None => (),
        }
        // match guard还可以解决Some里面会导致变量移动的问题
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("matched Some({})", n),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("x {:?}, y {}", x, y);
        // match里面使用match guard的时候，还可以配合上|使用
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"), // 优先级为(4|5|6) if y
            _ => println!("no"),
        }
    }
}
