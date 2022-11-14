use std::{fmt::Debug, ops::Add};

fn main() {
    // 关联类型
    {
        // 关联类型是一种能在特征内定义一个待填充的类型的语法
        // 和泛型基本一致，但是代码会比泛型简洁
        {
            trait Iterator {
                type Item;

                fn next(&mut self) -> Option<Self::Item>;
            }

            struct Counter {};

            impl Iterator for Counter {
                type Item = u32;
                fn next(&mut self) -> Option<Self::Item> {
                    Option::None
                }
            }

            let mut c = Counter {};
            // 指定了Item之后，Iterator就不是一个泛型trait了，就可以有Box<dyn Iterator>类型的变量啦
            let a: Box<dyn Iterator<Item = u32>> = Box::new(Counter {});
            c.next();
        }

        // 如果不用泛型，代码就会变成
        {
            trait Iterator<Item> {
                fn next(&mut self) -> bool;
            }
            // 你使用了泛型之后，所有需要用到Iterator的地方都需要写Iterator<Item>，用了关联类型，你只需要写Iterator
            struct Counter {};
            // 这里的Item是u8，还好，如果代码的Item是一个很长的类型呢
            impl Iterator<u8> for Counter {
                fn next(&mut self) -> bool {
                    false
                }
            }
            {
                trait Container<A, B> {
                    fn contains(&self, a: A, b: B) -> bool;
                }

                fn difference<A, B, C>(container: &C) -> i32
                where
                    C: Container<A, B>,
                {
                    1
                }
            }
        }

        // 但是如果你使用了type，你就可以
        {
            trait Container {
                type A;
                type B;
                fn contains(&self, a: Self::A, b: Self::B) -> bool;
                fn hello(&self) -> String {
                    "hello, world!".to_string()
                }
            }
            // hello 不再需要关注Container里面的A,B了
            fn hello<C: Container>(container: &C) {
                container.hello();
            }

            // 当你需要用到带泛型的方法的时候，再尝试补齐Container的A和B类型
            fn difference<T>(container: &T)
            where
                T: Container<A = i32, B = u8>,
            {
                container.contains(8i32, 5u8);
            }

            struct DockerContainer {}
            impl Container for DockerContainer {
                type A = i32;
                type B = u8;
                fn contains(&self, a: Self::A, b: Self::B) -> bool {
                    true
                }
            }
            let container = DockerContainer {};
            difference(&container);
        }
        // 默认泛型类型参数
        {
            // 如果你要使用特征的类型参数，可以指定一个具体的类型
            // 比如标准库里的Add
            {
                trait Add<RHS = Self> {
                    type Output;

                    fn add(self, rhs: RHS) -> Self::Output;
                }
            }
            // 如果你不重定义RHS，RHS就是你实现这个特征的类型
            #[derive(Debug, PartialEq)]
            struct Point {
                x: i32,
                y: i32,
            }

            use std::ops::Add;
            impl Add for Point {
                type Output = Point;

                fn add(self, rhs: Self) -> Self::Output {
                    Point {
                        x: self.x + rhs.x,
                        y: self.y + rhs.y,
                    }
                }
            }
            // 这样你的Point就可以有Add的功能了
            let p = Point { x: 2, y: 3 } + Point { x: 4, y: 5 };
        }

        // 如果你选择不用默认的泛型参数呢？
        {
            use std::ops::Add;
            // 实现两个不同类型的相加
            struct Millmeters(u32);
            struct Meters(u32);

            // 此时的RHS是Meters，不是MillMeters了
            impl Add<Meters> for Millmeters {
                type Output = Millmeters;
                fn add(self, rhs: Meters) -> Self::Output {
                    Millmeters(self.0 + (rhs.0 * 1000))
                }
            }
        }
    }

    // 如何调用同名的方法
    {
        // 不同特征拥有同名方法很正常
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("pilot")
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("wizard")
            }
        }

        impl Human {
            fn fly(&self) {
                println!("human")
            }
        }

        // 如果不加任何限制，默认调用的是方法
        let person = Human;
        person.fly(); // Human.fly()

        // 或者直接调用特征里的方法，传递&self参数
        Pilot::fly(&person);
        Wizard::fly(&person);

        // 但如果特征里的方法没有&self参数呢（比如new）？
        // 这里需要用完全限定语法
        <Human as Pilot>::fly(&person);
        <Human as Wizard>::fly(&person);
        // 语法：<Type as trait>::method(params)
    }

    // 特征定义中的特征约束
    {
        // 你可以在定义特征的时候约束实现这个特征的结构体的前提
        // 比如必须要求这个结构体先实现什么特征
        use std::fmt::Display;

        trait OutlinePrint: Display {
            fn outline_print(&self) {
                let output = self.to_string(); // 因为实现了OutlinePrint特征的结构体必须先实现了Display，进而实现了ToString
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        // 假设你没有实现Display就去实现OutlinePrint
        struct Point {
            x: i32,
            y: i32,
        }

        // impl OutlinePrint for Point {} // invalid!
        use std::fmt;
        impl Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {}", self.x, self.y)
            }
        }
        impl OutlinePrint for Point {} // valid!
    }

    // 如何绕过孤儿规则（不能在非结构体/特征的作用域为某个结构体声明某个特征）
    {
        use std::fmt;

        // struct接个元组把某个类型wrap起来，这样这个struct就在自己的作用域了
        // 因此就可以破坏孤儿规则，在当前的作用域为某个外部类型加上某个外部特征了
        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}
