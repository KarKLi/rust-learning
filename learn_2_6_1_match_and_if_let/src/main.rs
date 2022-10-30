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
            Bool::False => {},
        }
        // 更复杂的一个match例子
        enum Action {
            Say(String),
            MoveTo(i32,i32),
            ChangeColorRGB(u16,u16,u16),
        }
        
    }
}
