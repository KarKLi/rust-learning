use std::{fmt::format, path::Component};

fn main() {
    // &dyn some_trait 和 Box<dyn some_trait> 都可以被称为特征对象
    trait Draw {
        fn draw(&self) -> String;
    }

    struct Button {
        width: u32,
        height: u32,
        label: String,
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    // 如何解决函数中需要返回多个不同类型的对象作为同一种特征的问题？
    {
        impl Draw for Button {
            fn draw(&self) -> String {
                "This is a button".to_string()
            }
        }

        impl Draw for SelectBox {
            fn draw(&self) -> String {
                "This is a SelectBox".to_string()
            }
        }

        // 现在我们需要有个Vector来存放所有支持Draw trait的对象
        struct Screen {
            // 使用Box<dyn Draw>
            pub components: Vec<Box<dyn Draw>>,
        }
    }

    // 如果函数里面要使用trait的参数，可以传入Box<dyn Draw>，也可以传入&dyn Draw
    {
        impl Draw for u8 {
            fn draw(&self) -> String {
                format!("u8: {}", *self)
            }
        }

        impl Draw for f64 {
            fn draw(&self) -> String {
                format!("f64: {}", *self)
            }
        }
        // 这两者都会传在堆上的对象，因为是动态对象
        fn draw1(x: Box<dyn Draw>) {
            println!("{} with Box<dyn Draw>", x.draw())
        }

        fn draw2(x: &dyn Draw) {
            println!("{} with &dyn Draw", x.draw())
        }

        let x = 1.1f64;
        let y = 8u8;

        draw1(Box::new(x));
        draw2(&x);

        draw1(Box::new(y));
        draw2(&y);
    }

    // 如果你确实就只想让Vector里面的数据都是同类型的数据呢？（比如都是Button或者都是SelectBox）
    {
        // 这样就可以了（但是这里涉及到生命周期的问题，所以还是先使用Box<dyn Draw>作为T吧
        struct Screen<T: Draw> {
            pub components: Vec<T>,
        }
        impl<T> Screen<T>
        where
            T: Draw,
        {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
    }

    // 但是不是所有trait都能拥有特征对象的
    // 如果一个特征要返回Self，或者其中的方法有泛型参数，就不能拥有dyn打头的特征对象
    {
        // TA 不能有特征对象
        trait TA {
            fn new(&self) -> Self;
        }
        
        // invalid!
        // fn new_a(a: &dyn TA ) {

        // }

        // invalid too
        // fn new_a(a: Box<dyn TA>) {

        // }

        // TB 也不能有特征对象
        trait TB {
            fn new<T> (x: &T) -> i32;
        }

        // invalid
        // fn new_b(b: &dyn TB) {

        // }

        // invalid too
        // fn new_b(b: Box<dyn TB>) {

        // }
    }
}
