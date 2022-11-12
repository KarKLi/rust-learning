fn main() {
    // match是可以实现C中switch的一种关键字
    {
        enum Direction {
            East,
            West,
            North,
            South,
        }
        let dire = Direction::South;
        // match 必须概括所有的情况，如果不能列举出所有的情况，用_表示剩余的情况
        // 每种情况使用逗号分隔
        match dire {
            Direction::East => println!("east"),
            // 使用|可以组合起来，=>可以跟一条语句，也可以跟一个语句块
            Direction::North | Direction::South => {
                println!("south or north")
            }
            _ => println!("west"),
        }
    }

    // match是一个表达式，且返回值为每个模式的语句块或语句的结果
    // 因此每个模式的返回值必须相同
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        // match当然也可以直接赋值给变量
        enum IpAddr {
            Ipv4,
            Ipv6,
        }
        let ip1 = IpAddr::Ipv6;
        let ip_str = match ip1 {
            IpAddr::Ipv4 => "127.0.0.1",
            _ => "::1",
        };
        println!("{}", ip_str);
    }

    // match还有一个作用，就是从enum枚举中取出它的值
    {
        enum Bool {
            True(String),
            False,
        }
        let cond = Bool::True("True!".to_string());
        match cond {
            Bool::True(s) => {
                println!("{}", s);
            }
            Bool::False => {}
        }
        // 更复杂的一个match例子
        enum Action {
            Say(String),
            MoveTo(i32, i32),
            ChangeColorRGB(u16, u16, u16),
        }
        let actions = [
            Action::Say("Hello Rust".to_string()),
            Action::MoveTo(1, 2),
            Action::ChangeColorRGB(255, 255, 0),
        ];
        for action in actions {
            match action {
                Action::Say(s) => {
                    println!("{}", s);
                }
                Action::MoveTo(x, y) => {
                    println!("from (0, 0) move to ({}, {})", x, y);
                }
                Action::ChangeColorRGB(r, g, _) => {
                    println!("change color into '(r:{}, g:{}, b:0)", r, g);
                }
            }
        }
    }

    // 如果你只想关心一个值，那么使用match会导致你需要多写一个 _ => ()
    // 太啰嗦了
    // 我们使用if let代替
    {
        let v = Some(3u8);
        if let Some(3) = v {
            println!("3");
        }
    }

    // Rust中提供了一种叫matches!的宏，用于把表达式和一个模式进行匹配，并返回true或false
    {
        enum MyEnum {
            Foo,
            Bar,
        }
        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        // 如果对v进行过滤，可以调用这样的链式函数
        // v.iter().filter(|x| x == MyEnum::Foo); // wrong!
        // 这里错的原因是：x不能直接和枚举成员进行比较，需要进行模式匹配
        v.iter().filter(|x| matches!(x, MyEnum::Foo));

        // 还有一些高级的例子
        let foo = 'f';
        // 还记得我们模式匹配时候说的吗，|是一种组合符，将模式组合了起来
        assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
        // 下面这个例子有点if let的变种
        let bar = Some(4);
        // 如果枚举成员是带有值的，后面就可以跟if
        assert!(matches!(bar, Some(x) if x > 2));
    }

    // 无论是match还是if let，都可以在模式匹配的时候用新值覆盖旧值
    {
        let age = Some(30);
        println!("before match, age is {:?}", age); // age is Some(30)
        if let Some(age) = age {
            println!("after match, age is {}", age); // age is 30
        }
        println!("when exit match, age is {:?}", age); // age is Some(30)
        match age {
            Some(age) => println!("after match, age is {}", age), // age is 30
            _ => (),
        }
    } // 但是这种做法有风险，容易在模式语句块里面使用了错误的age（一种variable shadowing）
}
