use std::fmt::Display;

fn main() {
    // 特征是一种实现【接口】概念的方式，和go的interface很像
    // 特征使用trait关键字定义
    // 给Post和Weibo都实现summary特征
    struct Post {
        title: String,
        author: String,
        content: String,
    }
    struct Weibo {
        username: String,
        content: String,
    }

    {
        // 假设有一种特征叫summary
        trait Summary {
            fn summarize(&self) -> String;
        }

        // 使用impl来给结构体实现特征
        impl Summary for Post {
            fn summarize(&self) -> String {
                format!("文章 {}, 作者是{}", self.title, self.author)
            }
        }

        impl Summary for Weibo {
            fn summarize(&self) -> String {
                format!("{}发表了微博{}", self.username, self.content)
            }
        }
        // 调用一下这两个方法（虽然这个做法不能体现出【特征】的特性）
        let post = Post {
            title: "Rust语言简介".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了!".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "好像微博没Tweet好用".to_string(),
        };
        println!("{}", post.summarize());
        println!("{}", weibo.summarize());
    }

    // 特征可以有默认方法
    {
        trait Summary {
            fn summarize(&self) -> String {
                String::from("(read more...)")
            }
        }

        impl Summary for Post {} // 使用默认方法
        impl Summary for Weibo {
            fn summarize(&self) -> String {
                format!("{}发表了微博{}", self.username, self.content)
            }
        }
        let post = Post {
            title: "Rust语言简介".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了!".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "好像微博没Tweet好用".to_string(),
        };
        println!("{}", post.summarize());
        println!("{}", weibo.summarize());
    }

    // 真正可以用特征的地方：使用在函数参数内
    {
        trait Summary {
            fn summarize(&self) -> String {
                String::from("(read more...)")
            }
        }

        impl Summary for Post {}
        fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize())
        }
        // 上述语句等价于
        fn notify2<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize())
        }
        let post = Post {
            title: "Elon Mask bought Twitter".to_string(),
            author: "karkli".to_string(),
            content: "Elon Mask will fire 90% twitter employee".to_string(),
        };
        notify(&post);
        // 还可以让满足多种特征的数据传递进来（只需要使用+号）
        fn notify3(item: &(impl Summary + Display)) {}
        // 上述语句同样等价于
        fn notify4<T: Summary + Display>(item: &T) {}
        // 当特征很多的时候，可以使用where语句分行写特征
        // 如果你没有这一句use，你就不能直接使用Debug这个trait了，必须输入全名
        use std::fmt::Debug;
        fn some_function<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            1
        }
    }

    // 既然函数参数都可以用特征来做，那自然也可以把特征作为返回值
    {
        trait Summary {
            fn summarize(&self) -> String;
        }

        impl Summary for Weibo {
            fn summarize(&self) -> String {
                format!("{}发表了微博{}", self.username, self.content)
            }
        }

        fn returns_summarizable() -> impl Summary {
            Weibo {
                content: "I'm a tencent employee".to_string(),
                username: "karkli".to_string(),
            }
        }

        // 但是函数不能返回两个不同的变量（哪怕返回值都是impl Summary！）
        impl Summary for Post {
            fn summarize(&self) -> String {
                format!("文章 {}, 作者是{}", self.title, self.author)
            }
        }
        fn returns_summarizable_v2(switch: bool) -> impl Summary {
            // wrong! `if` and `else` have incompatible types

            // if switch {
            //     Post {
            //         title: "f**k the life".to_string(),
            //         author: "karkli".to_string(),
            //         content: "what a bad day...".to_string(),
            //     }
            // } else {
            //     Weibo {
            //         content: "I'm a tencent employee".to_string(),
            //         username: "karkli".to_string(),
            //     }
            // }
            Weibo {
                content: "I'm a tencent employee".to_string(),
                username: "karkli".to_string(),
            }
        }
    }

    // 还可以有条件地实现方法，为特定的特征实现特殊的方法
    {
        use std::fmt::Display;

        struct pair<T> {
            x: T,
            y: T,
        }

        impl<T> pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        impl<T: Display + PartialOrd> pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("x {} is bigger than y {}", self.x, self.y)
                } else {
                    println!("x {} is smaller than y {}", self.x, self.y)
                }
            }
        }
    }
}
