fn main() {
    // 先看一个泛型的例子
    {
        // 暂时还不能通过编译
        // fn add<T>(a: T, b: T) -> T {
        //     a + b
        // }

        // 也是不能通过编译的
        // fn largest<T>(list: &[T]) -> T {
        //     let mut largest = list[0];
        //     for &item in list.iter() {
        //         if item > largest {
        //             largest = item
        //         }
        //     }
        //     largest
        // }

        // 上述两个例子不能通过编译的原因：编译器并不知道实例化的时候，T是不是可加的（example 1）/T是不是可比较的（example 2）
        // 修改一下，限定T需要满足的trait（后面会讲到），就可以通过编译了
        fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }
        fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];
            for &item in list.iter() {
                if item > largest {
                    largest = item
                }
            }
            largest
        }

        let a = add(2, 3);
        let l = largest(&vec![1, 2, 3]);
    }

    // 结构体也可以使用泛型
    {
        struct Point<T> {
            x: T,
            y: T,
        }
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        // let what = Point { x: 1, y: 1.1 }; // wrong!
    }

    // 可以有多个泛型类型
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }
        let p = Point { x: 1, y: 1.1 };
    }

    // 还可以为泛型结构体定义方法
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
    }

    // 还可以为泛型结构体方法声明单独的泛型类型
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
    }

    // 还可以为泛型结构体中的特定类型定义方法ƒ
    {
        struct Point<T> {
            x: T,
            y: T,
        }
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        let p1 = Point {
            x: 3.0f32,
            y: 3.0f32,
        };
        let p2 = Point { x: 2, y: 5 };
        p1.distance_from_origin();
        // p2.distance_from_origin(); // wrong!
    }

    // const泛型（Rust 1.51引入）
    // 数组类型并不只根据类型决定，还由长度决定，因此，[i32;3]和[i32;2]是两种不同的类型
    // 如果需要在把数组传递给函数的时候，要么函数形参是丢失了长度信息的（如fn sum <T: std::ops::add>(arr: &[T])
    // 要么就只能限定死长度（如fn sum_3(arr: &[i32;3])
    // 现在，长度也可以作为一种确定类型的const泛型了
    {
        fn display_array<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
            println!("arr {:?} with size {}", arr, N);
        }
        display_array(&[1,2,3]);
        display_array(&[1,2,3,4]);
    }
}
