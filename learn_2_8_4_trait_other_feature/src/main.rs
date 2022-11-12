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
}
