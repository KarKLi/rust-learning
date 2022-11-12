fn main() {
    // rust使用impl定义方法
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }
        impl Circle {
            // 一般会使用new来作为构建函数的名字
            fn new(x: f64, y: f64, radius: f64) -> Circle {
                Circle {
                    x: x,
                    y: y,
                    radius: radius,
                }
            }
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }
        // 当然，也可以没有构造函数，这种情况下就直接使用结构体名称生成结构体即可
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        )
    }

    // 对于结构体的方法来说，第一个参数可以有三种取值：self, &self, &mut self
    // 第一种比较少用，因为这表明你把当前结构体的所有权转移给了该方法
    // 第二种相当于C++中的const方法，是对self的不可变借用，self里的所有成员都是只读
    // 第三种就是普通方法，是对self的可变借用，self的所有成员都是可RW的
    {
        struct Coor {
            x: f64,
            y: f64,
        }
        impl Coor {
            fn new(x: f64, y: f64) -> Coor {
                Coor { x: x, y: y }
            }
            // 方法名可与变量相同，常用这种方法来作为getter方法
            fn x(&self) -> f64 {
                self.x
            }
            fn set_x(&mut self, x: f64) {
                self.x = x
            }
            fn clear(self) -> Coor {
                Coor {
                    x: 0.0f64,
                    y: 0.0f64,
                }
            }
        }
    }

    // 你可以写多个impl块，不一定要在一个impl块中实现所有的方法
    {
        struct s {
            a: i32,
            b: i32,
        }
        impl s {
            fn new(a: i32, b: i32) -> s {
                s { a: a, b: b }
            }
        }
        impl s {
            fn a(&self) -> i32 {
                self.a
            }
        }
        impl s {
            fn b(&self) -> i32 {
                self.b
            }
        }
    }

    // 还可以给enum实现方法
    {
        #[allow(unused)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                if let Message::Write(s) = self {
                    println!("message is {}", s)
                }
            }
        }
        let msg1 = Message::Write("hello".to_string());
        msg1.call();
        let msg2 = Message::Quit;
        msg2.call();
    }
}
