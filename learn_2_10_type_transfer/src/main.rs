fn main() {
    // 不可以单纯比较两个不同类型的变量
    {
        let a: i32 = 10;
        let b: u16 = 100;

        // if a < b { // invalid!
        //     println!("10 < 100");
        // }
    }
    // 可以使用as转换
    {
        let a = 3.1 as i8;
        let b = 100_i8 as i32;
        let c = 'a' as u8;

        println!("{} {} {}", a, b, c);
    }

    // 还可以把内存地址转换成指针（高阶特性！）
    {
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        let first_addr = p1 as usize;
        let second_addr = first_addr + 4;
        let p2 = second_addr as *mut i32;
        unsafe {
            *p2 += 1;
        }
        assert_eq!(values[1], 3);
    }

    // 如果希望进行安全的转换（防止溢出等行为），则使用TryInto trait
    {
        use std::convert::TryInto;
        let a = 10_u8;
        let b = 1500_u16;
        // let b_: u8 = b.try_into().unwrap(); // 这里使用unwrap可能会panic，如果不用unwrap，返回的就是一个Result
        let b_: u8 = match b.try_into() {
            Ok(b1) => b1,
            Err(e) => {
                println!("{:?}", e.to_string());
                0
            }
        };
    }

    // 特征匹配时，是不会做任何强制转换的
    // 类型T可以转换成类型U，不代表impl T可以转化为impl U
    {
        trait Trait {}
        fn foo<X: Trait>(t: X) {}
        impl<'a> Trait for &'a i32 {}

        let t : &mut i32 = &mut 0;
        // foo(t); // wrong! &mut i32可以转化为&i32，&i32可以转化为Trait，不代表&mut i32也可以转化为Trait
    }

    // 假设有一个方法foo，有一个接收器
    // 那么调用value.foo()的时候，怎么判断foo的第一个接收器是什么类型呢？
    // 使用完全限定语法进行匹配
    {
        // 首先，判断是否可以直接调用T::foo(value)，称为值方法调用
        let s = String::from("hello");

        // 如果不行，就会尝试调用<&T>::foo(value)或者<&mut T>::foo(value)
        let mut s = "hello".to_string();
        s.push('a');

        // 如果都还是不行，就会对T进行解引用（这里可以解引用的类型T需满足Deref特征）
        // 即若T: Deref<Target=U>，则会使用U类型再次尝试，比如
        let mut s  = Box::new(String::new());
        s.push('a');

        // 若T不能被解引用，且T是一个定长类型，则尝试将其从定长类型转为不定长类型
        fn foo(a: &[i32]) {}
        let a: [i32;5] = [3;5];
        foo(&a);
    }

    // 再来看一个完全限定语法更难的例子
    {
        // 因为T实现了Clone，而clone方法要求&self的类型，所以&T可以用于调用Clone
        fn do_stuff<T: Clone>(value: &T) {
            let cloned = value.clone();
        }
        // 但这个例子呢？
        fn do_stuff_v2<T>(value: &T) {
            let cloned = value.clone();
        }
        // 为什么T的类型被擦除了还是可以？
        // 因为&T实现了clone，而完全限定方法判断&T不能调用clone（因为clone需要&self,返回的是Self）
        // 因此，只能使用&&T调用，而Self类型自然就是&T了
        // 为什么&T实现了clone？因为所有的引用类型都可以被复制（引用是不可变指针的别名）
        // 标准库中：impl<T: ?Sized> const Clone for &T

        // 更难的一个例子
        {
            use std::sync::Arc;
            #[derive(Clone)]
            // Container就是Arc的别名，为了给Arc加上Clone特征
            // 一个复杂类型是否能派生Clone特征，取决于内部的所有自类型是否都实现了Clone
            // 因此，也就是T有没有实现Clone
            struct Container<T>(Arc<T>);

            fn clone_container<T>(foo: &Container<i32>, bar: &Container<T>) {
                // i32实现了Clone，因此&Container<i32>自然能调用clone
                let foo_cloned = foo.clone();
                // T呢？T当然不能确定它有没有实现Clone，因此会尝试将bar再取引用，也就是
                // &&Container<T>，这个类型实现了Clone（引用都能实现Clone），因此Self类型就是&Container<T>
                let bar_cloned = bar.clone();
            }
        }

        // 你也可以不使用derive继承，自己实现Clone特征
        {
            use std::sync::Arc;
            struct Container<T>(Arc<T>);
            impl<T> Clone for Container<T> {
                fn clone(&self) -> Self {
                    Self(Arc::clone(&self.0))
                }
            }
        }

        // Transmutes不在我们学习的范围中，不应用这些奇技淫巧
    }
}
