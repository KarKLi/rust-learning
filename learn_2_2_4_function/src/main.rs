fn main() {
    // 一个很简单但是很power的函数
    {
        fn add(i: i32, j: i32) -> i32 {
            i + j
        }
        // 函数命名使用蛇形命名法
        fn print_helloworld() {
            println!("hello, world!");
        }
        print_helloworld();
    }
    // rust必须指出每个参数的类型（无论是泛型还是具名类型）
    {
        fn another_function(x: i32, y: f32) {
            println!("The value of x is: {}", x);
            println!("The value of y is: {}", y);
        }
        another_function(5, 6.1);
        // wrong!
        // fn another_function2(x:i32,y) {
        //     println!("The value of x is: {}", x);
        //     println!("The value of y is: {}", y);
        // }
    }
    // 函数的返回值就是函数体最后一条表达式的返回值，当然我们也可以使用 return 提前返回
    {
        fn plus_five(x: i32) -> i32 {
            x + 5
        }
        println!("plus_five(6): {}", plus_five(6));
        fn plus_or_minus(x:i32) -> i32 {
            if x > 5 {
                return x - 5
            }
        
            x + 5
        }
        println!("plus_or_minus(10): {}",plus_or_minus(10));
    }
    // 发散函数——表示永不返回的函数（要么panic要么infinite loop）
    {
        fn dead_end() -> ! {
            panic!("Die, insect!"); // 这句话是炎魔之王说的
        }
        // dead_end();
        fn forever() -> ! {
            loop {
                
            }
        }
    }
}
