fn main() {
    // 枚举是一种将相同类别东西集合起来的一种数据结构
    {
        // 扑克牌花色
        #[derive(Debug)]
        enum PokeSuit {
            Clubs,
            Spades,
            Diamonds,
            Hearts
        }
        // 使用::读取出某个枚举的成员
        let heart = PokeSuit::Hearts;
        let diamond = PokeSuit::Diamonds;
        // 枚举没有常规的显示方法，需要使用{:?}来格式化
        println!("{heart:?}");
        println!("{diamond:?}");
    }
    // 枚举成员还可以带上一个值
    {
        #[derive(Debug)]
        enum PokerCard {
            Clubs(u8),
            Spades(u8),
            Diamonds(u8),
            Hearts(u8),
        }
        // 初始化的时候就要把这个值带上
        let c1 = PokerCard::Spades(5);
        let c2 = PokerCard::Diamonds(13);
        println!("{c1:?}");
        println!("{c2:?}");
    }
    // 还可以是不同的类型的值
    {
        #[derive(Debug)]
        enum PokerCard {
            Clubs(u8),
            Spades(u8),
            Diamonds(char),
            Hearts(char),
        }
        // 初始化的时候就要把这个值带上
        let c1 = PokerCard::Spades(5);
        let c2 = PokerCard::Diamonds('A');
        println!("{c1:?}");
        println!("{c2:?}");
    }
    // 甚至可以是匿名结构体
    {
        enum Message {
            Quit,
            Move {x: i32, y:i32},
            Write(String),
            ChangeColor(i32,i32,i32),
        }
        let m1 = Message::Quit;
        let m2 = Message::Move { x: 1, y: 1 };
        let m3 = Message::ChangeColor(255, 255, 0);
    }
    // 大名鼎鼎的Option就是个enum类型
    {
        // 可省略Option前缀
        let some_number = Some(5);
        let some_string = Some("a string");
        // None不可省略，或要显式实例化类型
        let absent_number = Option::<i32>::None;
        let absent_number: Option<i32> = None;
    }
}
